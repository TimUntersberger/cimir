#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    r: u16,
    g: u16,
    b: u16,
}

impl Color {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b }
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        ]
    }
}
