//! for reference regarding the usage of freetype in this file look at the following link
//!
//! https://learnopengl.com/In-Practice/Text-Rendering

use freetype as ft;
use std::{collections::HashMap, rc::Rc};

use glium::{
    texture::{ClientFormat, CompressedSrgbTexture2d, RawImage2d, Texture2d, Texture2dArray},
    Display,
};

pub struct CharacterInfo {
    pub bearing: (i32, i32),
    pub size: (i32, i32),
    pub advance: i32,
    pub texture: Texture2d,
}

pub struct Font {
    character_info: HashMap<char, CharacterInfo>,
    pub size: u32,
}

impl Font {
    pub fn from_memory(display: &Display, buffer: &[u8], font_size: u32) -> Self {
        let lib = ft::Library::init().expect("Failed to initialize FreeType library");
        let face = lib
            .new_memory_face(Rc::new(buffer.to_vec()), 0)
            .expect("Font not found");

        face.set_pixel_sizes(0, font_size).unwrap();

        let mut character_info = HashMap::new();

        for c in 0..127u8 {
            face.load_char(c as usize, ft::face::LoadFlag::RENDER)
                .unwrap();
            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            let mut image = RawImage2d::from_raw_rgb(
                bitmap.buffer().to_vec(),
                (bitmap.width() as u32, bitmap.rows() as u32),
            );
            image.format = ClientFormat::U8;
            let texture = Texture2d::new(display, image).unwrap();
            character_info.insert(
                c as char,
                CharacterInfo {
                    size: (bitmap.width(), bitmap.rows()),
                    bearing: (glyph.bitmap_left(), glyph.bitmap_top()),
                    advance: glyph.advance().x,
                    texture,
                },
            );
        }

        Font {
            character_info,
            size: font_size,
        }
    }

    pub fn get_info(&self, c: char) -> Option<&CharacterInfo> {
        self.character_info.get(&c)
    }
}
