use crate::{
    color::Color,
    key::Key,
    renderer::Renderer,
    winit::{
        dpi::{LogicalPosition, LogicalSize},
        window::WindowBuilder,
        event_loop::ControlFlow
    },
    Application,
};

use chrono::prelude::*;

pub struct Appbar {
    active_ws: usize
}

impl Appbar {
    pub fn new() -> Self {
        Self {
            active_ws: 1
        }
    }
}

impl Application for Appbar {
    fn window(&mut self, w: WindowBuilder) -> WindowBuilder {
        w.with_decorations(false)
            .with_position(LogicalPosition::new(0.0, 0.0))
            .with_inner_size(LogicalSize::new(1920.0, 20.0))
            .with_always_on_top(true)
    }

    fn on_key_down(&mut self, key: Key, r: &mut Renderer) -> Option<ControlFlow> {
        match key {
            Key::One => self.active_ws = 1,
            Key::Two => self.active_ws = 2,
            Key::Three => self.active_ws = 3,
            _ => {}
        }
        None
    }

    fn init(&mut self, r: &mut Renderer) {
        r.set_background_color(Color::new(230, 230, 230));
    }

    fn render(&mut self, r: &mut Renderer) {
        r.row(|r| {
            for i in 1..4 {
                self.render_ws(r, i);
            }
            r.space(10.0);
            self.render_datetime(r, "%T");
            r.space(100.0);
            self.render_datetime(r, "%e %b %Y");
        });
        r.show_fps();
    }
}

impl Appbar {
    fn render_datetime(&self, r: &mut Renderer, fmt: &str) {
        r.move_cursor(0.0, -2.0, |r| {
            r.text(&Local::now().format(fmt).to_string(), Color::BLACK);
        });
    }

    fn render_ws(&self, r: &mut Renderer, id: usize) {
        let mut color = Color::new(210, 210, 210);
        if id == self.active_ws {
            color = Color::new(180, 180, 180);
        }
        r.rectangle((20.0, r.height()), color);
        r.move_cursor(-15.0, -2.0, |r| {
            r.text(&id.to_string(), Color::BLACK);
        });
    }
}
