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
            VirtualKeyCode::A => {
                dbg!(r.is_active(0));
            },
            _ => {}
        }
        None
    }

    fn init(&mut self, r: &mut Renderer<Self::TextureId>) {
        r.set_image(TextureId::Moon, include_bytes!("../test.jpg"));
        r.set_background_color(Color::new(230, 230, 230));
    }

    fn render(&mut self, r: &mut Renderer<Self::TextureId>) {
        r.texture(TextureId::Moon, (r.width(), r.height()));
        // r.hitbox(0, |r| {
        //     r.rectangle((20.0, 20.0), Color::BLACK);
        // });
        // r.space(20.0);
    }
}

fn main() {
    // appbar::Appbar::new().run();
    App::new().run();
}
