pub use winit;

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use std::{hash::Hash, time::Duration};

use crate::application::ApplicationWrapper;
use crate::renderer::Renderer;

mod animation;
mod appbar;
mod application;
mod color;
mod font;
mod renderer;
mod shaders;
mod vertex;

use animation::{Animation, Transition};
use application::Application;
use color::Color;

struct App {
    dark: bool,
    titlebar_animation_id: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            dark: false,
            titlebar_animation_id: 0,
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum TextureId {
    Moon,
}

impl Application for App {
    type TextureId = TextureId;

    fn on_key_down(
        &mut self,
        key: VirtualKeyCode,
        r: &mut Renderer<Self::TextureId>,
    ) -> Option<ControlFlow> {
        match key {
            VirtualKeyCode::Key1 => {
                self.dark = !self.dark;
                if self.dark {
                    r.set_background_color(Color::new(51, 51, 51));
                } else {
                    r.set_background_color(Color::new(230, 230, 230));
                }
            }
            VirtualKeyCode::Key2 => {
                r.reset_animation(self.titlebar_animation_id);
            }
            _ => {}
        }
        None
    }

    fn init(&mut self, r: &mut Renderer<Self::TextureId>) {
        r.set_image(TextureId::Moon, "test.jpg");
        r.set_background_color(Color::new(230, 230, 230));
    }

    fn render(&mut self, r: &mut Renderer<Self::TextureId>) {
        let color = if self.dark {
            Color::new(40, 40, 40)
        } else {
            Color::new(200, 200, 200)
        };
        r.animate(
            self.titlebar_animation_id,
            Duration::from_millis(500),
            &[Transition::Linear(0.0, 40.0)],
            |r, [height]| {
                r.row(|r| {
                    r.rectangle((r.remaining_width(), height), color);
                });
            },
        );
        r.show_fps();
        // r.space(20.0);
        // r.texture(TextureId::Moon, (800.0, 600.0));
    }
}

fn main() {
    // appbar::Appbar::new().run();
    App::new().run();
}
