pub use winit;

use winit::{
    event::VirtualKeyCode,
    event_loop::ControlFlow,
};

use crate::application::ApplicationWrapper;
use crate::renderer::Renderer;
use crate::primitives::LabelStyle;

mod animation;
mod appbar;
mod application;
mod color;
mod font;
mod renderer;
mod shaders;
mod vertex;
mod primitives;
mod styling;

use application::Application;
use color::Color;

struct App {
    value: String
}

impl App {
    pub fn new() -> Self {
        Self {
            value: "".into()
        }
    }
}

impl Application for App {
    type TextureId = u32;

    fn on_key_down(
        &mut self,
        key: VirtualKeyCode,
        r: &mut Renderer<Self::TextureId>,
    ) -> Option<ControlFlow> {
        if r.is_active(0) {
            match key {
                VirtualKeyCode::LShift
                | VirtualKeyCode::LControl => {},
                VirtualKeyCode::Back => {
                    self.value.pop();
                },
                VirtualKeyCode::Space => {
                    self.value.push(' ');
                },
                key => {
                    self.value = format!("{}{:?}", self.value, key);
                }
            }
        }
        match key {
            VirtualKeyCode::F1 => {
                r.change_font_size(30);
            },
            VirtualKeyCode::F2 => {
                r.change_font_size(18);
            },
            _ => {}
        }
        None
    }

    fn init(&mut self, r: &mut Renderer<Self::TextureId>) {
        r.set_background_color(Color::new(230, 230, 230));
    }

    fn render(&mut self, r: &mut Renderer<Self::TextureId>) {
        r.space(10.0);
        r.row(|r| {
            r.space(10.0);
            r.hitbox(0, |r, active| {
                if active { 
                    let (_, y) = r.pos();
                    let (_, height, text_end_x) = r.label(&self.value, HoveredCustomStyle);
                    let cursor_height = r.font.size as f32;
                    r.set_cursor(text_end_x + 2.0, y + (height - cursor_height) / 2.0, |r| {
                        r.rectangle((1.5, cursor_height), Color::BLACK);
                    });
                } else { 
                    r.label(&self.value, CustomStyle);
                };
            });
        });
        r.show_fps();
    }
}

struct HoveredCustomStyle;
struct CustomStyle;

impl Into<LabelStyle> for CustomStyle {
    fn into(self) -> LabelStyle {
        LabelStyle {
            padding: (5.0, 3.0).into(),
            min_width: 200.0,
            foreground_color: Color::BLACK,
            background_color: Some(Color::new(180, 180, 180)),
            ..Default::default()
        }
    }
}

impl Into<LabelStyle> for HoveredCustomStyle {
    fn into(self) -> LabelStyle {
        LabelStyle {
            background_color: Some(Color::new(140, 140, 140)),
            ..CustomStyle.into()
        }
    }
}

fn main() {
    // appbar::Appbar::new().run();
    App::new().run();
}
