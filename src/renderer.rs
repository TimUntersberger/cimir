pub use winit::{
    event::{Event, VirtualKeyCode, WindowEvent, ModifiersState},
    event_loop::ControlFlow,
};

use glium::{
    index::PrimitiveType,
    DrawParameters,
    Blend,
    texture::{CompressedSrgbTexture2d, RawImage2d, Texture2d},
    uniform, Display, Frame, IndexBuffer, Program, Surface, VertexBuffer,
};

use std::{collections::HashMap, convert::TryInto, time::{Duration, Instant}};

use crate::animation::{Animation, Transition};
use crate::color::Color;
use crate::font::Font;
use crate::key::Key;
use crate::shaders::{FONT_VERTEX_SHADER, FONT_FRAGMENT_SHADER};
use crate::vertex::{Vertex, FontVertex};

#[derive(Debug, Copy, Clone)]
pub struct MouseInfo {
    pub x: f64,
    pub y: f64,
    pub lmouseclick: bool,
    pub rmouseclick: bool
}

/// a hitbox is an area in the window that senses clicks/hovers/...
#[derive(Debug, Copy, Clone)]
pub struct Hitbox {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32
}

impl Hitbox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            height,
            width
        }
    }

    pub fn contains_pos(&self, x: f32, y: f32) -> bool {
        let left = self.x;
        let right = left + self.width;
        let top = self.y;
        let bottom = top + self.height;

        (left <= x && x <= right) && (top <= y && y <= bottom)
    }
}

pub struct Renderer {
    /// this holds the current frame
    frame: Frame,
    /// how long the last frame took to render in nanoseconds
    frame_time: u32,
    frame_start: Instant,
    display: Display,
    program: Program,
    font_program: Program,
    pub modifiers: ModifiersState,
    pub font: Font,
    /// used for scaling the ui to the display
    viewport: (f32, f32),
    cursor: (f32, f32),
    pub background_color: Color,
    layout_stack: Vec<Layout>,
    animations: HashMap<u32, Animation>,
    textures: HashMap<u32, Texture>,
    /// holds the current mouse information
    pub mouse: MouseInfo,
    /// holds the characters that were input between the last render and current render
    pub input: Vec<char>,
    /// holds the virtual keys that were pressed between the last render and current render
    pub keys: Vec<Key>,
    pub active_id: Option<u32>,
    pub hot_id: Option<u32>,
    pub(crate) hitboxes: HashMap<u32, Hitbox>,
    /// the hitboxes the renderer is currently inside
    hitbox_stack: Vec<Hitbox>
}

impl Renderer {
    pub fn new(display: Display, program: Program) -> Self {
        let mut frame = display.draw();
        frame.set_finish().unwrap();
        let font = Font::from_memory(&display, include_bytes!("../font.ttf"), 18);
        Self {
            frame,
            modifiers: Default::default(),
            background_color: Color::new(0, 0, 0),
            font,
            keys: Vec::new(),
            mouse: MouseInfo {
                x: 0.0,
                y: 0.0,
                lmouseclick: false,
                rmouseclick: false,
            },
            input: Vec::new(),
            font_program: Program::from_source(&display, FONT_VERTEX_SHADER, FONT_FRAGMENT_SHADER, None).unwrap(),
            display,
            frame_time: 0,
            frame_start: Instant::now(),
            program,
            viewport: (0.0, 0.0),
            cursor: (0.0, 0.0),
            layout_stack: vec![Layout::Col {
                width: 0.0,
                x: 0.0,
                y: 0.0,
            }],
            active_id: None,
            hot_id: None,
            animations: HashMap::new(),
            textures: HashMap::new(),
            hitboxes: HashMap::new(),
            hitbox_stack: Vec::new()
        }
    }

    pub fn change_font_size(&mut self, size: u32) {
        self.font = Font::from_memory(&self.display, include_bytes!("../font.ttf"), size);
    }

    pub fn pos(&self) -> (f32, f32) {
        self.cursor
    }

    pub fn is_active(&self, id: u32) -> bool {
        self.active_id.map(|aid| aid == id).unwrap_or(false)
    }

    pub fn is_hot(&self, id: u32) -> bool {
        self.hot_id.map(|aid| aid == id).unwrap_or(false)
    }

    pub(crate) fn clear_hitboxes(&mut self) {
        self.hitboxes.clear();
    }

    pub fn fps(&self) -> u32 {
        if self.frame_time == 0 {
            return 0;
        }

        1_000_000_000 / self.frame_time
    }

    fn projection_matrix(&self) -> [[f32; 4]; 4] {
        cgmath::ortho(0.0, self.viewport.0, self.viewport.1, 0.0, 0.0, 1.0).into()
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_image(&mut self, id: u32, data: &[u8]) {
        let image = {
            let image = image::io::Reader::new(std::io::Cursor::new(data))
                .with_guessed_format()
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

    fn draw_vertices(&mut self, vertices: &[Vertex]) {
        let (vb, ib) = self.setup_draw(vertices);

        let tex = Texture2d::empty(&self.display, 0, 0).unwrap();
        let uniforms = uniform! {
            use_texture: false,
            tex: &tex,
            projection: self.projection_matrix()
        };

        self.frame
            .draw(&vb, &ib, &self.program, &uniforms, &Default::default())
            .unwrap();
    }

    fn draw_texture(&mut self, size: (f32, f32), texture_id: u32) {
        let (x, y) = self.cursor;
        let (width, height) = size;
        let vertices = &[
            Vertex::textured((0.0, 1.0), x, y),
            Vertex::textured((0.0, 0.0), x, y + height),
            Vertex::textured((1.0, 1.0), x + width, y),
            Vertex::textured((1.0, 0.0), x + width, y + height),
        ];
        let (vb, ib) = self.setup_draw(vertices);

        match self.textures.get(&texture_id).expect("Texture not found") {
            Texture::Image(tex) => {
                let uniforms = uniform! {
                    use_texture: true,
                    projection: self.projection_matrix(),
                    tex: tex,
                };

                self.frame
                    .draw(&vb, &ib, &self.program, &uniforms, &Default::default())
                    .unwrap();
            }
        }
    }

    pub fn set_cursor(&mut self, x: f32, y: f32, mut f: impl FnMut(&mut Self)) {
        let cursor_copy = self.cursor;
        self.cursor.0 = if x < 0.0 {
            self.width() + x as f32
        } else { x as f32 };
        self.cursor.1 = if y < 0.0 {
            self.height() + y as f32
        } else { y as f32 };
        f(self);
        self.cursor = cursor_copy;
    }

    pub fn move_cursor(&mut self, x: f32, y: f32, mut f: impl FnMut(&mut Self)) {
        let cursor_copy = self.cursor;
        self.cursor.0 += x;
        self.cursor.1 += y;
        f(self);
        self.cursor = cursor_copy;
    }

    pub fn clear(&mut self) {
        let c: [f32; 3] = self.background_color.into();
        self.frame.clear_color(c[0], c[1], c[2], 1.0);
    }

    pub fn texture(&mut self, id: u32, size: (f32, f32)) {
        let (width, height) = size;
        self.draw_texture((width, height), id);
        self.handle_new_shape(width, height);
    }

    pub fn show_fps(&mut self) {
        self.set_cursor(-80.0, 0.0, |r| {
            r.text(&format!("{:4} fps", r.fps()), Color::BLACK);
        });
    }

    pub fn calculate_text_size(&self, text: &str) -> (f32, f32) {
        let mut x = 0.0;
        let mut width = 0.0;
        let height = self.font.size as f32;
        let scale = 1.0;

        for c in text.chars() {
            let info = self.font.get_info(c).expect("The character is missing from the font");
            let old_x = x;
            x += ((info.advance >> 6) as f32) * scale; // bitshift by 6 to get value in pixels (2^6 = 64)
            width += x - old_x;
        }

        (width, height)
    }

    pub fn text(&mut self, value: &str, color: Color) {
        let (mut x, y) = self.cursor;
        let mut width = 0.0;
        let mut height = 0.0;
        let scale = 1.0;

        let draw_params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        for c in value.chars() {
            let info = self.font.get_info(c).expect("The character is missing from the font");
            let xpos = x + info.bearing.0 as f32 * scale;
            let ypos = y + (info.size.1 - info.bearing.1) as f32 * scale + (self.font.size as f32 - info.size.1 as f32) * scale;
            let w = info.size.0 as f32 * scale;
            let h = info.size.1 as f32 * scale;
            if (ypos + h - y) > height {
                height = ypos + h - y;
            }
            let uniforms = uniform! {
                tex: &info.texture,
                projection: self.projection_matrix(),
            };
            let vertices = &[
                FontVertex {
                    position: [xpos, ypos + h],
                    tex_pos: [0.0, 1.0],
                    color: color.into()
                },
                FontVertex {
                    position: [xpos, ypos],
                    tex_pos: [0.0, 0.0],
                    color: color.into()
                },
                FontVertex {
                    position: [xpos + w, ypos],
                    tex_pos: [1.0, 0.0],
                    color: color.into()
                },
                FontVertex {
                    position: [xpos, ypos + h],
                    tex_pos: [0.0, 1.0],
                    color: color.into()
                },
                FontVertex {
                    position: [xpos + w, ypos],
                    tex_pos: [1.0, 0.0],
                    color: color.into()
                },
                FontVertex {
                    position: [xpos + w, ypos + h],
                    tex_pos: [1.0, 1.0],
                    color: color.into()
                },
            ];
            let old_x = x;
            // advance cursors for next glyph (note that advance is number of 1/64 pixels)
            x += ((info.advance >> 6) as f32) * scale; // bitshift by 6 to get value in pixels (2^6 = 64)
            width += x - old_x;
            let vb = VertexBuffer::new(&self.display, vertices).unwrap();
            let ib = IndexBuffer::new(
                &self.display,
                PrimitiveType::TriangleStrip,
                &(0..6).collect::<Vec<u16>>(),
            )
            .unwrap();
            self.frame
                .draw(&vb, &ib, &self.font_program, &uniforms, &draw_params)
                .unwrap();
        }
        self.handle_new_shape(width, height);

    }

    pub fn row(&mut self, mut f: impl FnMut(&mut Self) -> ()) {
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

    pub fn col(&mut self, mut f: impl FnMut(&mut Self) -> ()) {
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
        mut f: impl FnMut(&mut Self, [f32; N]) -> (),
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

    pub fn get_hit(&self, x: f32, y: f32) -> Option<u32> {
        self.hitboxes
            .iter()
            .find(|(_, hb)| hb.contains_pos(x, y))
            .map(|(id, _)| *id)
    }

    pub fn hitbox(&mut self, id: u32, mut f: impl FnMut(&mut Self, bool, bool) -> ()) {
        let is_hot = self.is_hot(id);
        let is_active = self.is_active(id);
        self.hitbox_stack.push(Hitbox::new(self.cursor.0, self.cursor.1, 0.0, 0.0));
        f(self, is_hot, is_active);
        self.hitboxes.insert(id, self.hitbox_stack.pop().unwrap());
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
        };

        if let Some(hitbox) = self.hitbox_stack.iter_mut().last() {
            let width = shape_width;//self.cursor.0 - hitbox.x;
            let height = shape_height;//self.cursor.1 - hitbox.y;

            if width > hitbox.width {
                hitbox.width = width;
            }

            if height > hitbox.height {
                hitbox.height = height;
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
            ]
        );

        self.handle_new_shape(width, height);
    }

    pub fn consume_input(&mut self) -> Vec<char> {
        std::mem::take(&mut self.input)
    }

    pub fn consume_keys(&mut self) -> Vec<Key> {
        std::mem::take(&mut self.keys)
    }

    pub(crate) fn next_frame(&mut self) {
        self.reset_cursor();
        self.viewport = self.get_viewport();
        self.frame = self.display.draw();
        self.frame_start = Instant::now();
    }

    pub(crate) fn done(&mut self) {
        self.frame.set_finish().unwrap();
        self.frame_time = self.frame_start.elapsed().as_nanos() as u32;
        self.input.clear();
        self.keys.clear();
    }
}

pub enum Layout {
    Row { height: f32, x: f32, y: f32 },
    Col { width: f32, x: f32, y: f32 },
}

pub enum Texture {
    Image(CompressedSrgbTexture2d),
}
