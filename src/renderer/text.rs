use std::collections::HashMap;

use freetype::{face::LoadFlag, Library};

use crate::library::utils::{convert_coordinates, convert_size};

use super::{error::{Error, Result}, styles::Size, texture::Texture, vertice::Position};

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

pub fn calculate_text_size(characters: &HashMap<char, Character>, text: &str, start_position: Position, max_width: Option<f32>, scale: f32, window_size: Size) -> Size {
    let mut min_y = start_position.y;

    let mut text_size = Size { width: max_width.unwrap_or(0.0), height: 0.0 };

    let text_max_width = max_width.unwrap_or(0.0);
    let text_start_position = convert_coordinates(start_position, &window_size);

    let lines: Vec<&str> = text.split('\n').collect();

    let mut line_height = 0.0;

    for line in lines {
        let mut line_width = 0.0;
        let mut max_height = 0.0;
        let mut prev_height = 0.0;
        let mut height_offset = 0.0;
        let mut is_new_word = false;

        for (idx, ch) in line.chars().enumerate() {
            if ch == ' ' {
                line_width += (0.03 * window_size.width * scale) / 2.0;
                is_new_word = true;
                
                continue;
            }

            if text_max_width > 0.0 && is_new_word {
                if let Some(found_index) = line[idx..].find(' ') {
                    let found_index = found_index + idx;

                    let word_width = (calculate_word_width(&characters, &line[idx..found_index], scale, window_size.width) * window_size.width) / 2.0;
                    
                    if start_position.x + line_width + word_width >= start_position.x + text_max_width {
                        line_height += max_height + 0.09;
                        line_width = 0.0;
                        max_height = 0.0;
                        prev_height = 0.0;
                        height_offset = 0.0;
                        is_new_word = false;
                    }
                } else {
                    let word_width = (calculate_word_width(&characters, &line[idx..], scale, window_size.width) * window_size.width) / 2.0;

                    if start_position.x + line_width + word_width >= start_position.x + text_max_width {
                        line_height += max_height + 0.09;
                        line_width = 0.0;
                        max_height = 0.0;
                        prev_height = 0.0;
                        height_offset = 0.0;
                        is_new_word = false;
                    }
                }
            }

            assert!(characters.get(&ch) != None, "character must exist, provided: {}", ch);

            let character = characters.get(&ch).unwrap();

            let character_size = convert_size(Size { width: character.size.width * scale, height: character.size.height * scale }, &window_size);

            let offset_y = ((character.size.height - character.offset.y) * scale * 2.0) / window_size.height;
            
            if max_height == 0.0 {
                max_height = character_size.height;
            } else {
                if (character.size.height - character.offset.y) > 0.0 {
                    if max_height < (character_size.height - offset_y) {
                        max_height = character_size.height - offset_y;
                    }
                } else {
                    if max_height < character_size.height {
                        max_height = character_size.height;
                    }
                }
            }

            if prev_height == 0.0 {
                prev_height = character_size.height;
            }
            
            if is_new_word {
                if max_height == character_size.height {
                    height_offset += prev_height - max_height;
                } else {
                    height_offset += prev_height - character_size.height;
                }
            } else {
                height_offset += prev_height - character_size.height;
            }

            let character_start_y = ((text_start_position.y - line_height - height_offset - offset_y) * window_size.height) / 2.0;
            if min_y > character_start_y {
                min_y = character_start_y;
            }

            let character_end_y = character_start_y + character.size.height;
            if text_size.height < (character_end_y - min_y) {
                text_size.height = character_end_y - min_y;
            }

            prev_height = character_size.height;

            line_width += (character.size.width + character.offset.x) * scale;
            
            if is_new_word {
                is_new_word = false;
            }
        }

        if max_width.is_none() && text_size.width < line_width {
            text_size.width = line_width;
        }

        line_height += max_height + 0.08;
    }

    text_size
}

