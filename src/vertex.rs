pub use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use glium::implement_vertex;

use crate::color::Color;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
    tex_pos: [f32; 2],
}

implement_vertex!(Vertex, position, color, tex_pos);

impl Vertex {
    pub fn colored(color: Color, x: f32, y: f32) -> Self {
        Self {
            color: color.into(),
            position: [x, y],
            tex_pos: [0.0, 0.0],
        }
    }
    pub fn textured(tex_pos: (f32, f32), x: f32, y: f32) -> Self {
        Self {
            position: [x, y],
            color: [0.0, 0.0, 0.0],
            tex_pos: [tex_pos.0, tex_pos.1],
        }
    }
}
