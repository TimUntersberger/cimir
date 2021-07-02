pub use winit;

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use std::{hash::Hash, time::Duration};

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

impl Application for App {
    type TextureId = u32;

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
        r.set_background_color(Color::new(230, 230, 230));
    }

    fn render(&mut self, r: &mut Renderer<Self::TextureId>) {
        r.space(10.0);
        r.row(|r| {
            r.space(10.0);
            r.hitbox(0, |r, active| {
                if active { 
                    r.label("Click me!", HoveredCustomStyle);
                } else { 
                    r.label("Click me!", CustomStyle);
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
            padding: 20.0.into(),
            foreground_color: Color::WHITE,
            background_color: Some(Color::BLACK),
            ..Default::default()
        }
    }
}

impl Into<LabelStyle> for HoveredCustomStyle {
    fn into(self) -> LabelStyle {
        LabelStyle {
            background_color: Some(Color::new(5, 5, 5)),
            ..CustomStyle.into()
        }
    }
}

fn main() {
    // appbar::Appbar::new().run();
    App::new().run();
}
