use crate::{
    color::Color,
    winit::{
        dpi::{LogicalPosition, LogicalSize},
        window::WindowBuilder,
    },
    Application,
};

pub struct Appbar;

impl Appbar {
    pub fn new() -> Self {
        Self
    }
}

impl Application for Appbar {
    type TextureId = u32;

    fn window(&mut self, w: WindowBuilder) -> WindowBuilder {
        w.with_decorations(false)
            .with_position(LogicalPosition::new(0.0, 0.0))
            .with_inner_size(LogicalSize::new(1920.0, 20.0))
            .with_always_on_top(true)
    }

    fn init(&mut self, r: &mut crate::Renderer<Self::TextureId>) {
        r.set_background_color(Color::new(10, 10, 10));
    }

    fn render(&mut self, r: &mut crate::Renderer<Self::TextureId>) {
        r.row(|r| {
            r.rectangle((20.0, r.height()), Color::new(20, 20, 20));
            r.rectangle((20.0, r.height()), Color::new(30, 30, 30));
            r.rectangle((20.0, r.height()), Color::new(20, 20, 20));
        });
    }
}
