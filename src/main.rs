use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use glium::{
    texture::{Texture2d, RawImage2d, CompressedSrgbTexture2d}, uniform, Rect, Frame, glutin::ContextBuilder, implement_vertex, index::PrimitiveType, uniforms::EmptyUniforms,
    Display, IndexBuffer, Program, Surface, VertexBuffer, DrawParameters
};

use std::{hash::Hash, convert::TryInto, collections::HashMap, time::{Duration, Instant}};

const VERTEX_SHADER: &'static str = r#"
#version 330 core
layout (location = 0) in vec2 position; // the position variable has attribute position 0
layout (location = 1) in vec3 color; // the position variable has attribute position 0
layout (location = 2) in vec2 tex_pos;

uniform vec2 size;
uniform bool is_percentage;
  
out vec4 vertex_color;
out vec2 vertex_tex_pos;

void main()
{
    float x, y;
    if (!is_percentage) {
        x = position.x * size.x;
        y = position.y * size.y;
        x = x - 1;
        y = 1 - y;
    } else {
        x = position.x * 2 - 1;
        y = 1 - position.y * 2;
    }
    gl_Position = vec4(x, y, 1.0, 1.0);
    vertex_color = vec4(color, 1.0);
    vertex_tex_pos = tex_pos;
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 330 core
out vec4 FragColor;

uniform sampler2D tex;
uniform bool use_texture;
  
in vec4 vertex_color;
in vec2 vertex_tex_pos;

void main()
{
    if (use_texture) {
        FragColor = texture(tex, vertex_tex_pos);
    } else {
        FragColor = vertex_color;
    }
} 
"#;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
    tex_pos: [f32; 2]
}

implement_vertex!(Vertex, position, color, tex_pos);

impl Vertex {
    pub fn colored(color: Color, x: f32, y: f32) -> Self {
        Self {
            color: color.into(),
            position: [x, y],
            tex_pos: [0.0, 0.0]
        }
    }
    pub fn textured(tex_pos: (f32, f32), x: f32, y: f32) -> Self {
        Self {
            position: [x, y],
            color: [0.0, 0.0, 0.0],
            tex_pos: [tex_pos.0, tex_pos.1]
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Color {
    r: u16,
    g: u16,
    b: u16
}

impl Color {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self {
            r,
            g,
            b
        }
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0]
    }
}

#[derive(Debug, Clone)]
struct Animation {
    start_time: Instant,
    duration: Duration,
    transitions: Vec<Transition>,
    pub done: bool
}

impl Animation {
    pub fn new(duration: Duration, transitions: Vec<Transition>) -> Self {
        Animation {
            start_time: Instant::now(),
            duration,
            transitions,
            done: false
        }
    }

    pub fn animate(&mut self) -> Vec<f32> {
        if self.done {
            return self.transitions.iter().map(|t| t.get_done()).collect();
        }
        let progress = (self.start_time.elapsed().as_millis() as f32 / self.duration.as_millis() as f32).min(1.0);
        if progress == 1.0 {
            self.done = true;
        }
        self.transitions.iter().map(|t| t.calculate(progress)).collect()
    }
}

enum Layout {
    Row { height: f32, x: f32, y: f32 },
    Col { width: f32, x: f32, y: f32 }
}

enum Texture {
    Image(CompressedSrgbTexture2d)
}

struct Renderer<TTextureId: Hash + Eq> {
    /// this holds the current frame
    frame: Frame,
    display: Display,
    program: Program,
    /// used for scaling the ui to the display
    viewport: (f32, f32),
    cursor: (f32, f32),
    layout_stack: Vec<Layout>,
    animations: HashMap<u32, Animation>,
    textures: HashMap<TTextureId, Texture>
}

impl<TTextureId: Hash + Eq> Renderer<TTextureId> {
    pub fn new(display: Display, program: Program) -> Self {
        let mut frame = display.draw();
        frame.set_finish().unwrap();
        Self {
            frame,
            display,
            program,
            viewport: (0.0, 0.0),
            cursor: (0.0, 0.0),
            layout_stack: vec![Layout::Col { width: 0.0, x: 0.0, y: 0.0 }],
            animations: HashMap::new(),
            textures: HashMap::new()
        }
    }

    pub fn set_image(&mut self, id: TTextureId, path: &str) {
        let image = {
            let image = image::io::Reader::open(path).unwrap().decode().unwrap().to_rgba8();
            let image_dimensions = image.dimensions();
            let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            CompressedSrgbTexture2d::new(&self.display, image).unwrap()
        };

        self.textures.insert(id, Texture::Image(image));
    }

    pub fn get_viewport(&self) -> (f32, f32) {
        let size = self.display.gl_window().window().inner_size();
        (size.width as f32, size.height as f32)
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = (0.0, 0.0);
    }

    fn setup_draw(&mut self, vertices: &[Vertex]) -> (VertexBuffer<Vertex>, IndexBuffer<u16>) {
        let vb = VertexBuffer::new(&self.display, vertices).unwrap();
        let ib = IndexBuffer::new(
            &self.display, 
            PrimitiveType::TriangleStrip, 
            &(0..(vertices.len() as u16)).collect::<Vec<u16>>()
        ).unwrap();

        (vb, ib)
    }

    fn draw_vertices(&mut self, vertices: &[Vertex], percentages: bool) {
        let (vb, ib) = self.setup_draw(vertices);

        let tex = Texture2d::empty(&self.display, 0, 0).unwrap();
        let uniforms = uniform! { 
            size: [1.0/self.viewport.0, 1.0/self.viewport.1],
            is_percentage: percentages,
            use_texture: false,
            tex: &tex
        };

        self.frame.draw(&vb, &ib, &self.program, &uniforms, &Default::default())
            .unwrap();
    }

    fn draw_texture(&mut self, pos: (f32, f32), size: (f32, f32), texture_id: TTextureId) {
        let (x, y) = pos;
        let (width, height) = size;
        let vertices = &[
            Vertex::textured((0.0, 1.0), x, y),
            Vertex::textured((0.0, 0.0), x, y + height),
            Vertex::textured((1.0, 1.0), x + width, y),
            Vertex::textured((1.0, 0.0), x + width, y + height)
        ];
        let (vb, ib) = self.setup_draw(vertices);

        match self.textures.get(&texture_id).unwrap() {
            Texture::Image(tex) => {
                let uniforms = uniform! { 
                    size: [1.0/self.viewport.0, 1.0/self.viewport.1],
                    is_percentage: false,
                    use_texture: true,
                    tex: tex
                };

                self.frame.draw(&vb, &ib, &self.program, &uniforms, &Default::default())
                    .unwrap();
            }
        }
    }

    pub fn clear(&mut self) {
        self.frame.clear_color(0.0, 0.0, 0.0, 1.0);
    }

    pub fn texture(&mut self, id: TTextureId, size: (f32, f32)) {
        let (width, height) = size;
        let x = self.cursor.0;
        let y = self.cursor.1;
        self.draw_texture((x, y), (width, height), id);
        self.handle_new_shape(width, height);
    }

    pub fn text(&mut self, value: &str) {
    }

    pub fn row(&mut self, f: impl Fn(&mut Self) -> ()) {
        self.layout_stack.push(Layout::Row {
            height: 0.0,
            x: self.cursor.0,
            y: self.cursor.1
        });
        f(self);
        if let Layout::Row { height, x, y } = self.layout_stack.pop().unwrap() {
            self.cursor.0 = x;
            self.cursor.1 = y + height;
        }
    }

    pub fn col(&mut self, f: impl Fn(&mut Self) -> ()) {
        self.layout_stack.push(Layout::Col {
            width: 0.0,
            x: self.cursor.0,
            y: self.cursor.1
        });
        f(self);
        if let Layout::Col { width, y, x } = self.layout_stack.pop().unwrap() {
            self.cursor.0 = x + width;
            self.cursor.1 = y;
        }
    }

    pub fn animate<const N: usize>(&mut self, id: u32, duration: Duration, transitions: &[Transition; N], f: impl Fn(&mut Self, [f32; N]) -> ()) {
        let result = match self.animations.get_mut(&id) {
            Some(animation) => animation.animate(),
            None => {
                let mut animation = Animation::new(duration, transitions.to_vec());
                let result = animation.animate();
                self.animations.insert(id, animation);
                result
            }
        };

        f(self, result.try_into().unwrap());
    }

    fn handle_new_shape(&mut self, shape_width: f32, shape_height: f32) {
        match self.layout_stack.iter_mut().last().unwrap() {
            Layout::Row { height, .. } => {
                self.cursor.0 += shape_width;
                if shape_height > *height {
                    *height = shape_height;
                }
            },
            Layout::Col { width, .. } => {
                self.cursor.1 += shape_height;
                if shape_width > *width {
                    *width = shape_width;
                }
            },
        }
    }

    pub fn space(&mut self, size: f32) {
        match self.layout_stack.iter().last().unwrap() {
            Layout::Row { .. } => self.cursor.0 += size,
            Layout::Col { .. } => self.cursor.1 += size,
        }
    }

    pub fn rectangle(&mut self, size: (f32, f32), color: Color) {
        let (width, height) = size;
        let (x, y) = self.cursor;

        self.draw_vertices(&[
            Vertex::colored(color, x, y),
            Vertex::colored(color, x, y + height),
            Vertex::colored(color, x + width, y),
            Vertex::colored(color, x + width, y + height)
        ], false);

        self.handle_new_shape(width, height);
    }

    pub(crate) fn next_frame(&mut self) {
        self.reset_cursor();
        self.viewport = self.get_viewport();
        self.frame = self.display.draw();
    }

    pub(crate) fn done(&mut self) {
        self.frame.set_finish().unwrap();
    }
}

trait Application {
    type TextureId: Eq + Hash;

    fn init(&mut self, renderer: &mut Renderer<Self::TextureId>);
    fn render(&mut self, renderer: &mut Renderer<Self::TextureId>);
    fn on_event(&mut self, _event: Event<()>) -> Option<ControlFlow> {
        None
    }
}

trait ApplicationWrapper<T: Application> {
    fn run(self);
    fn call_render(&mut self, renderer: &mut Renderer<T::TextureId>);
}

impl<T: 'static> ApplicationWrapper<T> for T where T: Application {
    fn call_render(&mut self, renderer: &mut Renderer<T::TextureId>) {
        renderer.clear();
        renderer.next_frame();
        self.render(renderer);
        renderer.done();
    }

    fn run(mut self) {
        let ev = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &ev).unwrap();
        let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
        let mut renderer = Renderer::new(display, program);

        self.init(&mut renderer);

        self.call_render(&mut renderer);

        ev.run(move |event, _, control_flow| {
            *control_flow = match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => ControlFlow::Exit,
                    WindowEvent::Resized(..) => ControlFlow::Poll,
                    _ => ControlFlow::Poll,
                },
                Event::MainEventsCleared => {
                    self.call_render(&mut renderer);
                    ControlFlow::Poll
                }
                _ => ControlFlow::Poll,
            };

            if let Some(cf) = self.on_event(event) {
                *control_flow = cf;
            }
        });
    }
}

#[derive(Clone, Debug)]
enum Transition {
    Linear(f32, f32)
}

impl Transition {
    pub fn calculate(&self, progress: f32) -> f32 {
        match self {
            Self::Linear(from, to) => {
                let d = to - from;
                from + d * progress
            }
        }
    }
    pub fn get_done(&self) -> f32 {
        match self {
            Self::Linear(_, end) => *end
        }
    }
}

struct App;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum TextureId {
    Moon
}

impl Application for App {
    type TextureId = TextureId;

    fn on_event(&mut self, event: Event<()>) -> Option<ControlFlow> {
        None
    }

    fn init(&mut self, r: &mut Renderer<Self::TextureId>) {
        r.set_image(TextureId::Moon, "test.jpg");
    }

    fn render(&mut self, r: &mut Renderer<Self::TextureId>) {
        r.texture(TextureId::Moon, (800.0, 600.0));
    }
}

fn main() {
    App.run();
}
