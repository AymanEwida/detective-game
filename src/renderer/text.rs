use std::collections::HashMap;

use freetype::{face::LoadFlag, Library};

use super::{error::{Error, Result}, render::Size, texture::Texture, vertice::Position};

#[derive(Debug, PartialEq)]
pub struct Character {
    pub texture: Texture,
    pub size: Size,
    pub offset: Position,
}

impl Character {
    pub fn new(texture: Texture, size: Size, offset: Position) -> Self {
        Self {
            texture,
            size,
            offset
        }
    }
}

pub fn generated_characters_bitmap(characters_number: Option<u32>) -> Result<HashMap<char, Character>> {
    let lib = Library::init()
        .map_err(| error | Error::LoadFontFaceError(format!("message: Faild in Library init, real error: {}", error.to_string())))?;

    let face = lib.new_face("assets/fonts/Roboto-Regular.ttf", 0)
        .map_err(| error | Error::LoadFontFaceError(format!("message: Faild in Face (Font) creation, real error: {}", error.to_string())))?;
    face.set_char_size(0, 48 * 64, 0, 0)
        .map_err(| error | Error::LoadFontFaceError(format!("message: Faild in set font size, real error: {}", error.to_string())))?;

    let characters_number = characters_number.unwrap_or(128);

    unsafe {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    }

    let mut characters = HashMap::with_capacity(characters_number as usize);

    for ascii_number in 0..characters_number {
        if let Some(ch) = char::from_u32(ascii_number) {
            face.load_char(ch as usize, LoadFlag::RENDER)
                .map_err(| error | Error::LoadCharacterError(format!("message: Unable to load {}, real error: {}", ch, error.to_string())))?;

            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            
            let texture;
            unsafe {
                texture = Texture::new();
                texture.set_wrapping(gl::CLAMP_TO_EDGE);
                texture.set_filtering(gl::LINEAR);
                texture.load_bytes(Size { width: bitmap.width(), height: bitmap.rows() }, bitmap.buffer());

                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
                gl::Enable(gl::BLEND);
            }

            let character = Character::new(texture, Size { width: bitmap.width() as f32, height: bitmap.rows() as f32 }, Position { x: glyph.bitmap_left() as f32, y: glyph.bitmap_top() as f32 });

            characters.insert(ch, character);
        }
    }

    Ok(characters)
}

pub fn calculate_word_width(characters: &HashMap<char, Character>, word: &str, scale: f32, window_width: f32) -> f32 {
    let mut sum_width = 0.0;
    
    for ch in word.chars() {
        assert!(characters.get(&ch) != None, "character must exist");

        let character = characters.get(&ch).unwrap();

        sum_width += ((character.size.width + character.offset.x) * scale * 2.0) / window_width;
    }

    sum_width
}
