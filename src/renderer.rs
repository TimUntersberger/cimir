pub use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use glium::{
    index::PrimitiveType,
    texture::{CompressedSrgbTexture2d, RawImage2d, Texture2d},
    uniform, Display, Frame, IndexBuffer, Program, Surface, VertexBuffer,
};

use std::{collections::HashMap, convert::TryInto, hash::Hash, time::Duration};

use crate::animation::{Animation, Transition};
use crate::color::Color;
use crate::vertex::Vertex;

pub struct Renderer<TTextureId: Hash + Eq> {
    /// this holds the current frame
    frame: Frame,
    display: Display,
    program: Program,
    /// used for scaling the ui to the display
    viewport: (f32, f32),
    cursor: (f32, f32),
    background_color: Color,
    layout_stack: Vec<Layout>,
    animations: HashMap<u32, Animation>,
    textures: HashMap<TTextureId, Texture>,
}

impl<TTextureId: Hash + Eq> Renderer<TTextureId> {
    pub fn new(display: Display, program: Program) -> Self {
        let mut frame = display.draw();
        frame.set_finish().unwrap();
        Self {
            frame,
            background_color: Color::new(0, 0, 0),
            display,
            program,
            viewport: (0.0, 0.0),
            cursor: (0.0, 0.0),
            layout_stack: vec![Layout::Col {
                width: 0.0,
                x: 0.0,
                y: 0.0,
            }],
            animations: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_image(&mut self, id: TTextureId, path: &str) {
        let image = {
            let image = image::io::Reader::open(path)
                .unwrap()
                .decode()
                .unwrap()
                .to_rgba8();
            let image_dimensions = image.dimensions();
            let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            CompressedSrgbTexture2d::new(&self.display, image).unwrap()
        };

        self.textures.insert(id, Texture::Image(image));
    }

    pub fn remaining_width(&self) -> f32 {
        self.viewport.0 - self.cursor.0
    }

    pub fn remaining_height(&self) -> f32 {
        self.viewport.1 - self.cursor.1
    }

    pub fn width(&self) -> f32 {
        self.viewport.0
    }

    pub fn height(&self) -> f32 {
        self.viewport.1
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
            &(0..(vertices.len() as u16)).collect::<Vec<u16>>(),
        )
        .unwrap();

        (vb, ib)
    }

    fn draw_vertices(&mut self, vertices: &[Vertex], percentages: bool) {
        let (vb, ib) = self.setup_draw(vertices);

        let tex = Texture2d::empty(&self.display, 0, 0).unwrap();
        let size = [self.viewport.0, self.viewport.1];
        let uniforms = uniform! {
            size: size,
            is_percentage: percentages,
            use_texture: false,
            tex: &tex
        };

        self.frame
            .draw(&vb, &ib, &self.program, &uniforms, &Default::default())
            .unwrap();
    }

    fn draw_texture(&mut self, size: (f32, f32), texture_id: TTextureId) {
        let (x, y) = self.cursor;
        let (width, height) = size;
        let vertices = &[
            Vertex::textured((0.0, 1.0), x, y),
            Vertex::textured((0.0, 0.0), x, y + height),
            Vertex::textured((1.0, 1.0), x + width, y),
            Vertex::textured((1.0, 0.0), x + width, y + height),
        ];
        let (vb, ib) = self.setup_draw(vertices);

        match self.textures.get(&texture_id).unwrap() {
            Texture::Image(tex) => {
                let uniforms = uniform! {
                    size: [self.viewport.0 / 100.0, self.viewport.1 / 100.0],
                    is_percentage: false,
                    use_texture: true,
                    tex: tex
                };

                self.frame
                    .draw(&vb, &ib, &self.program, &uniforms, &Default::default())
                    .unwrap();
            }
        }
    }

    pub fn clear(&mut self) {
        let c: [f32; 3] = self.background_color.into();
        self.frame.clear_color(c[0], c[1], c[2], 1.0);
    }

    pub fn texture(&mut self, id: TTextureId, size: (f32, f32)) {
        let (width, height) = size;
        self.draw_texture((width, height), id);
        self.handle_new_shape(width, height);
    }

    pub fn text(&mut self, _value: &str) {}

    pub fn row(&mut self, f: impl Fn(&mut Self) -> ()) {
        self.layout_stack.push(Layout::Row {
            height: 0.0,
            x: self.cursor.0,
            y: self.cursor.1,
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
            y: self.cursor.1,
        });
        f(self);
        if let Layout::Col { width, y, x } = self.layout_stack.pop().unwrap() {
            self.cursor.0 = x + width;
            self.cursor.1 = y;
        }
    }

    pub fn animate<const N: usize>(
        &mut self,
        id: u32,
        duration: Duration,
        transitions: &[Transition; N],
        f: impl Fn(&mut Self, [f32; N]) -> (),
    ) {
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
            }
            Layout::Col { width, .. } => {
                self.cursor.1 += shape_height;
                if shape_width > *width {
                    *width = shape_width;
                }
            }
        }
    }

    pub fn reset_animation(&mut self, id: u32) {
        if let Some(animation) = self.animations.get_mut(&id) {
            animation.reset();
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

        self.draw_vertices(
            &[
                Vertex::colored(color, x, y),
                Vertex::colored(color, x, y + height),
                Vertex::colored(color, x + width, y),
                Vertex::colored(color, x + width, y + height),
            ],
            false,
        );

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

pub enum Layout {
    Row { height: f32, x: f32, y: f32 },
    Col { width: f32, x: f32, y: f32 },
}

pub enum Texture {
    Image(CompressedSrgbTexture2d),
}
