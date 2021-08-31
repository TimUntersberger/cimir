pub use winit;

use winit::{
    event::VirtualKeyCode,
    event_loop::ControlFlow,
};

use chrono::prelude::*;

use crate::application::ApplicationWrapper;
use crate::renderer::Renderer;
use crate::key::Key;
use crate::primitives::{TextInputStyle, TextInputState};

mod animation;
mod appbar;
mod application;
mod key;
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
    state: TextInputState
}

impl App {
    pub fn new() -> Self {
        Self {
            state: TextInputState::new("")
        }
    }
}

impl Application for App {
    fn on_mouse_down(
        &mut self,
        left: bool,
        x: f32,
        y: f32,
        r: &mut Renderer
    ) -> Option<ControlFlow> {
        if left {
            r.active_id = r.get_hit(x, y);
        }
        None
    }

    fn on_key_down(
        &mut self,
        key: Key,
        r: &mut Renderer,
    ) -> Option<ControlFlow> {
        match key {
            Key::F1 => {
                r.change_font_size(30);
            },
            Key::F2 => {
                r.change_font_size(18);
            },
            Key::Backspace => {
                // if r.is_active(0) {
                //     self.value.pop();
                // }
            }
            _ => {}
        }
        None
    }

    fn init(&mut self, r: &mut Renderer) {
        r.set_background_color(Color::new(230, 230, 230));
    }

    fn render(&mut self, r: &mut Renderer) {
        r.set_cursor(10.0, 10.0, |r| {
            r.text_input(0, &mut self.state, CustomStyle);
            r.text_input(1, &mut self.state, CustomStyle);
            r.space(1.0);
            for c in self.state.value.chars() {
                r.space(1.0);
                r.label(&format!("* {}", c), ());
            }
            r.show_fps();
        });
    }
}

struct HoveredCustomStyle;
struct CustomStyle;

impl Into<TextInputStyle> for CustomStyle {
    fn into(self) -> TextInputStyle {
        TextInputStyle {
            padding: (5.0, 3.0).into(),
            min_width: 200.0,
            foreground_color: Color::BLACK,
            background_color: Some(Color::new(180, 180, 180)),
            ..Default::default()
        }
    }
}

impl Into<TextInputStyle> for HoveredCustomStyle {
    fn into(self) -> TextInputStyle {
        TextInputStyle {
            background_color: Some(Color::new(140, 140, 140)),
            ..CustomStyle.into()
        }
    }
}

fn main() {
    // appbar::Appbar::new().run();
    App::new().run();
}
