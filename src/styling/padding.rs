#[derive(Debug, Copy, Clone)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

impl Into<Padding> for f32 {
    fn into(self) -> Padding {
        Padding {
            left: self,
            right: self,
            bottom: self,
            top: self,
        }
    }
}

impl Into<Padding> for (f32, f32) {
    fn into(self) -> Padding {
        Padding {
            left: self.0,
            right: self.0,
            bottom: self.1,
            top: self.1,
        }
    }
}

impl Into<Padding> for (f32, f32, f32, f32) {
    fn into(self) -> Padding {
        Padding {
            left: self.0,
            top: self.1,
            right: self.2,
            bottom: self.3,
        }
    }
}
