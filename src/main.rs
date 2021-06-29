use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use glium::{
    uniform, Rect, Frame, glutin::ContextBuilder, implement_vertex, index::PrimitiveType, uniforms::EmptyUniforms,
    Display, IndexBuffer, Program, Surface, VertexBuffer, DrawParameters
};

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

struct Renderer<'a> {
    frame: Frame,
    display: &'a Display,
    program: &'a Program,
    viewport: (f32, f32),
    cursor: (f32, f32),
    row_stack: Vec<f32>,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &'a Display, program: &'a Program) -> Self {
        let size = display.gl_window().window().inner_size();
        let viewport = (size.width as f32, size.height as f32);
        Self {
            frame: display.draw(),
            display,
            program,
            viewport,
            cursor: (0.0, 0.0),
            row_stack: vec![]
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

    pub fn row(&mut self, f: impl Fn(&mut Renderer) -> ()) {
        self.row_stack.push(0.0);
        f(self);
        self.cursor.0 = 0.0;
        self.cursor.1 += self.row_stack.pop().unwrap();
    }

    pub fn space(&mut self, size: f32) {
        if !self.row_stack.is_empty() {
            self.cursor.0 += size;
        } else {
            self.cursor.1 += size;
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

        if let Some(val) = self.row_stack.iter_mut().last() {
            self.cursor.0 += width;
            if *val < height {
                *val = height;
            }
        } else {
            self.cursor.1 += height;
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
    fn call_render(&mut self, display: &Display, program: &Program);
}

impl<T: 'static> ApplicationWrapper for T where T: Application {
    fn call_render(&mut self, display: &Display, program: &Program) {
        let mut renderer = Renderer::new(display, program);
        self.render(&mut renderer);
        renderer.done();
    }

    fn run(mut self) {
        let ev = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &ev).unwrap();
        let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

        self.call_render(&display, &program);

        ev.run(move |event, _, control_flow| {
            *control_flow = match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => ControlFlow::Exit,
                    WindowEvent::Resized(..) => {
                        self.call_render(&display, &program);
                        ControlFlow::Poll
                    }
                    _ => ControlFlow::Poll,
                },
                _ => ControlFlow::Poll,
            };

            if let Some(cf) = self.on_event(event) {
                *control_flow = cf;
            }
        });
    }
}

struct App;

impl Application for App {
    fn on_event(&mut self, event: Event<()>) -> Option<ControlFlow> {
        dbg!(&event);
        None
    }
    fn render(&mut self, r: &mut Renderer) {
        let btn_size = (200.0, 100.0);

        r.row(|r| {
            r.rectangle(btn_size, Color::new(0, 200, 0));
            r.space(10.0);
            r.rectangle(btn_size, Color::new(0, 200, 0));
            r.space(10.0);
            r.rectangle(btn_size, Color::new(0, 200, 0));
        });
        r.space(10.0);
        r.rectangle(btn_size, Color::new(0, 200, 0));
        r.space(10.0);
        r.row(|r| {
            r.rectangle(btn_size, Color::new(0, 200, 0));
            r.space(10.0);
            r.rectangle(btn_size, Color::new(0, 200, 0));
            r.space(10.0);
            r.rectangle(btn_size, Color::new(0, 200, 0));
        });
    }
}

fn main() {
    App.run();
}
