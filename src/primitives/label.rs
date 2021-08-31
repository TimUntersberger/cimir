use crate::color::Color;
use crate::styling::Padding;
use crate::renderer::Renderer;

#[derive(Debug)]
pub struct LabelStyle {
    pub background_color: Option<Color>,
    pub foreground_color: Color,
    pub padding: Padding,
    pub min_width: f32
}

impl Default for LabelStyle {
    fn default() -> Self {
        Self {
            background_color: None,
            foreground_color: Color::BLACK,
            padding: 0.0.into(),
            min_width: 0.0
        }
    }
}

impl Into<LabelStyle> for () {
    fn into(self) -> LabelStyle {
        Default::default()
    }
}

impl Renderer {
    pub fn label<T: Into<LabelStyle>>(&mut self, text: &str, style: T) -> (f32, f32, f32) {
        let style = style.into();

        // This is needed to more correctly position the text vertically.
        // Might change based on font and font size not sure yet.
        let font_sorcery = 2.0;
        let (x, y) = self.pos();
        let (width, height) = self.calculate_text_size(text);
        let rect_width = width.max(style.min_width) + style.padding.left + style.padding.right;
        let rect_height = height + style.padding.top + style.padding.bottom - font_sorcery * 1.5;
        let text_x = x + style.padding.left;
        let text_y = y + style.padding.top - font_sorcery * 2.0;
        self.rectangle((rect_width, rect_height), style.background_color.unwrap_or(self.background_color));
        self.set_cursor(text_x, text_y, |r| {
            r.text(text, style.foreground_color);
        });

        (rect_width, rect_height, text_x + width)
    }

}
