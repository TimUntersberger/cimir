use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use glium::{
    uniform, Rect, Frame, glutin::ContextBuilder, implement_vertex, index::PrimitiveType, uniforms::EmptyUniforms,
    Display, IndexBuffer, Program, Surface, VertexBuffer, DrawParameters
};

use std::{convert::TryInto, collections::HashMap, time::{Duration, Instant}};

const VERTEX_SHADER: &'static str = r#"
#version 330 core
layout (location = 0) in vec2 position; // the position variable has attribute position 0
layout (location = 1) in vec3 color; // the position variable has attribute position 0

uniform vec2 size;
uniform bool is_percentage;
  
out vec4 vertexColor; // specify a color output to the fragment shader

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
    vertexColor = vec4(color, 1.0);
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 330 core
out vec4 FragColor;
  
in vec4 vertexColor; // the input variable from the vertex shader (same name and same type)  

void main()
{
    FragColor = vertexColor;
} 
"#;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

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

struct Renderer<'a> {
    frame: Frame,
    display: &'a Display,
    program: &'a Program,
    viewport: (f32, f32),
    cursor: (f32, f32),
    layout_stack: Vec<Layout>,
    animations: &'a mut HashMap<u32, Animation>
}

impl<'a> Renderer<'a> {
    pub fn new(display: &'a Display, program: &'a Program, animations: &'a mut HashMap<u32, Animation>) -> Self {
        let size = display.gl_window().window().inner_size();
        let viewport = (size.width as f32, size.height as f32);
        Self {
            frame: display.draw(),
            display,
            program,
            viewport,
            cursor: (0.0, 0.0),
            layout_stack: vec![Layout::Col { width: 0.0, x: 0.0, y: 0.0 }],
            animations
        }
    }

    fn draw_vertices(&mut self, vertices: &[Vertex], percentages: bool) {
        let vb = VertexBuffer::new(self.display, vertices).unwrap();
        let ib = IndexBuffer::new(
            self.display, 
            PrimitiveType::TriangleStrip, 
            &(0..(vertices.len() as u16)).collect::<Vec<u16>>()
        ).unwrap();

        let uniforms = uniform! { 
            size: [1.0/self.viewport.0, 1.0/self.viewport.1],
            is_percentage: percentages
        };

        self.frame.draw(&vb, &ib, &self.program, &uniforms, &Default::default())
            .unwrap();
    }

    pub fn clear(&mut self) {
        self.frame.clear_color(0.0, 0.0, 0.0, 1.0);
    }

    pub fn row(&mut self, f: impl Fn(&mut Renderer) -> ()) {
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

    pub fn col(&mut self, f: impl Fn(&mut Renderer) -> ()) {
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

    pub fn animate<const N: usize>(&mut self, id: u32, duration: Duration, transitions: &[Transition; N], f: impl Fn(&mut Renderer, [f32; N]) -> ()) {
        let result = match self.animations.get_mut(&id) {
            Some(animation) => animation.animate(),
            None => {
                let mut animation = Animation::new(duration, transitions.to_vec());
                let result = animation.animate();
                self.animations.insert(id, animation);
                dbg!(&self.animations);
                result
            }
        };

        f(self, result.try_into().unwrap());
    }

    pub fn space(&mut self, size: f32) {
        match self.layout_stack.iter().last().unwrap() {
            Layout::Row { .. } => self.cursor.0 += size,
            Layout::Col { .. } => self.cursor.1 += size,
        }
    }

    pub fn rectangle(&mut self, size: (f32, f32), color: Color) {
        let color = color.into();
        let (width, height) = size;
        let (x, y) = self.cursor;

        self.draw_vertices(&[
            Vertex {
                position: [x, y],
                color,
            },
            Vertex {
                position: [x, y + height],
                color,
            },
            Vertex {
                position: [x + width, y],
                color,
            },
            Vertex {
                position: [x + width, y + height],
                color,
            },
        ], false);

        let shape_height = height;
        let shape_width = width;

        match self.layout_stack.iter_mut().last().unwrap() {
            Layout::Row { height, .. } => {
                self.cursor.0 += width;
                if shape_height > *height {
                    *height = shape_height;
                }
            },
            Layout::Col { width, .. } => {
                self.cursor.1 += height;
                if shape_width > *width {
                    *width = shape_width;
                }
            },
        }
    }

    pub fn done(self) {
        self.frame.finish().unwrap();
    }
}

trait Application {
    fn render(&mut self, renderer: &mut Renderer);
    fn on_event(&mut self, _event: Event<()>) -> Option<ControlFlow> {
        None
    }
}

trait ApplicationWrapper {
    fn run(self);
    fn call_render(&mut self, display: &Display, program: &Program, animations: &mut HashMap<u32, Animation>);
}

impl<T: 'static> ApplicationWrapper for T where T: Application {
    fn call_render(&mut self, display: &Display, program: &Program, animations: &mut HashMap<u32, Animation>) {
        let mut renderer = Renderer::new(display, program, animations);
        renderer.clear();
        self.render(&mut renderer);
        renderer.done();
    }

    fn run(mut self) {
        let ev = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &ev).unwrap();
        let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
        let mut animations = HashMap::new();

        self.call_render(&display, &program, &mut animations);

        ev.run(move |event, _, control_flow| {
            *control_flow = match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => ControlFlow::Exit,
                    WindowEvent::Resized(..) => ControlFlow::Poll,
                    _ => ControlFlow::Poll,
                },
                Event::MainEventsCleared => {
                    self.call_render(&display, &program, &mut animations);
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

impl Application for App {
    fn on_event(&mut self, event: Event<()>) -> Option<ControlFlow> {
        None
    }
    fn render(&mut self, r: &mut Renderer) {
        let btn_size = (200.0, 100.0);

        r.row(|r| {
            r.col(|r| {
                r.rectangle(btn_size, Color::new(0, 200, 0));
                r.space(10.0);
                r.rectangle(btn_size, Color::new(0, 200, 0));
            });
            r.space(10.0);
            r.col(|r| {
                r.rectangle(btn_size, Color::new(0, 200, 0));
                r.space(10.0);
                r.rectangle(btn_size, Color::new(0, 200, 0));
            });
            r.space(10.0);
            r.col(|r| {
                r.rectangle(btn_size, Color::new(0, 200, 0));
                r.space(10.0);
                r.rectangle(btn_size, Color::new(0, 200, 0));
            });
        });
    }
}

fn main() {
    App.run();
}
