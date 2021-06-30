use crate::Application;

pub struct Appbar;

impl Appbar {
    pub fn new() -> Self {
        Self
    }
}

impl Application for Appbar {
    type TextureId = u32;

    fn init(&mut self, _r: &mut crate::Renderer<Self::TextureId>) {
        todo!()
    }

    fn render(&mut self, _r: &mut crate::Renderer<Self::TextureId>) {
        todo!()
    }
}
