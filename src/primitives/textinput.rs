use crate::color::Color;
use crate::styling::Padding;
use crate::renderer::Renderer;
use crate::primitives::LabelStyle;
use crate::key::Key;

use chrono::prelude::*;

#[derive(Debug)]
pub struct TextInputState {
    pub value: String,
    pub last_typed_at: DateTime<Local>
}

impl TextInputState {
    pub fn new(s: &str) -> Self {
        Self {
            value: s.to_string(),
            last_typed_at: Local::now()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TextInputStyle {
    pub background_color: Option<Color>,
    pub foreground_color: Color,
    pub padding: Padding,
    pub min_width: f32
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            background_color: None,
            foreground_color: Color::BLACK,
            padding: 0.0.into(),
            min_width: 0.0
        }
    }
}

impl Into<LabelStyle> for TextInputStyle {
    fn into(self) -> LabelStyle {
        LabelStyle {
            background_color: self.background_color,
            foreground_color: self.foreground_color,
            padding: self.padding,
            min_width: self.min_width,
            ..Default::default()
        }
    }
}

impl Into<TextInputStyle> for () {
    fn into(self) -> TextInputStyle {
        Default::default()
    }
}

impl Renderer {
    pub fn text_input<T: Into<TextInputStyle>>(&mut self, id: u32, state: &mut TextInputState, style: T) {
        let style = style.into();
        self.hitbox(id, move |r, hot, active| {
            if hot || active { 
                let (_, y) = r.pos();
                let (_, height, text_end_x) = r.label(&state.value, style);

                if active {
                    let mut changed = false;
                    for c in r.consume_input() {
                        if c.is_alphanumeric() || c.is_whitespace() {
                            state.value.push(c);
                        }
                        changed = true;
                    }
                    for k in r.consume_keys() {
                        match k {
                            Key::Backspace => {
                                state.value.pop();
                                changed = true;
                            },
                            _ => {}
                        }
                    }
                    if changed {
                        state.last_typed_at = Local::now();
                    }
                    let cursor_height = r.font.size as f32;
                    let cursor_width = 1.5;

                    let current_millis = (Local::now() - state.last_typed_at).num_milliseconds() % 1000;
                    if current_millis < 500 {
                        r.set_cursor(text_end_x + 2.0, y + (height - cursor_height) / 2.0, |r| {
                            r.rectangle((cursor_width, cursor_height), Color::BLACK);
                        });
                    }
                }
            } else { 
                r.label(&state.value, style);
            };
        });
    }

}
