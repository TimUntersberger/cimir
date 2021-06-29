use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use glium::{
    Frame, glutin::ContextBuilder, implement_vertex, index::PrimitiveType, uniforms::EmptyUniforms,
    Display, IndexBuffer, Program, Surface, VertexBuffer,
};

const VERTEX_SHADER: &'static str = r#"
#version 330 core
layout (location = 0) in vec2 position; // the position variable has attribute position 0
layout (location = 1) in vec3 color; // the position variable has attribute position 0
  
out vec4 vertexColor; // specify a color output to the fragment shader

void main()
{
    gl_Position = vec4(position, 1.0, 1.0);
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
}

impl<'a> Renderer<'a> {
    pub fn new(display: &'a Display, program: &'a Program) -> Self {
        Self {
            frame: display.draw(),
            display,
            program
        }
    }

    fn draw_vertices(&mut self, vertices: &[Vertex]) {
        let vb = VertexBuffer::new(self.display, vertices).unwrap();
        let ib = IndexBuffer::new(
            self.display, 
            PrimitiveType::TriangleStrip, 
            &(0..(vertices.len() as u16)).collect::<Vec<u16>>()
        ).unwrap();

        self.frame.draw(&vb, &ib, &self.program, &EmptyUniforms, &Default::default())
            .unwrap();
    }

    pub fn rectangle(&mut self, color: Color) {
        let color = color.into();
        self.draw_vertices(&[
            Vertex {
                position: [-0.5, -0.5],
                color,
            },
            Vertex {
                position: [-0.5, 0.5],
                color,
            },
            Vertex {
                position: [0.5, -0.5],
                color,
            },
            Vertex {
                position: [0.5, 0.5],
                color,
            },
        ])
    }

    pub fn done(self) {
        self.frame.finish().unwrap();
    }
}

trait Application {
    fn render(&mut self, renderer: &mut Renderer);
}

trait ApplicationWrapper {
    fn run(self);
}

impl<T: 'static> ApplicationWrapper for T where T: Application {
    fn run(mut self) {
        let ev = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &ev).unwrap();
        let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

        let mut renderer = Renderer::new(&display, &program);
        self.render(&mut renderer);
        renderer.done();

        ev.run(move |event, _, control_flow| {
            *control_flow = match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => ControlFlow::Exit,
                    WindowEvent::Resized(..) => {
                        let mut renderer = Renderer::new(&display, &program);
                        self.render(&mut renderer);
                        renderer.done();
                        ControlFlow::Poll
                    }
                    _ => ControlFlow::Poll,
                },
                _ => ControlFlow::Poll,
            };
        });
    }
}

struct App;

impl Application for App {
    fn render(&mut self, r: &mut Renderer) {
        r.rectangle(Color::new(20, 250, 25));
    }
}

fn main() {
    App.run();
}
