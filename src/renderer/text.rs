use std::collections::HashMap;

use freetype::{face::LoadFlag, Library};

use super::{
    error::{Error, Result},
    styles::Size,
    texture::Texture,
    vertice::Position,
};

#[derive(Debug, PartialEq)]
pub struct Character {
    pub texture: Texture,
    pub size: Size,
    pub offset: Position,
    pub advance: f32,
}

impl Character {
    pub fn new(texture: Texture, size: Size, offset: Position, advance: f32) -> Self {
        Self {
            texture,
            size,
            offset,
            advance,
        }
    }
}

#[derive(Debug)]
pub struct FontMatrics {
    pub line_height: f32,
    pub ascender: f32, // the maximum height above baseline any glyph in the font can reach
    pub descender: f32, // the maximum depth below baseline any glyph can reach
}

impl FontMatrics {
    pub fn new(line_height: f32, ascender: f32, descender: f32) -> Self {
        Self {
            line_height,
            ascender,
            descender,
        }
    }
}

pub fn generated_characters_bitmap(
    characters_number: Option<u32>,
) -> Result<(HashMap<char, Character>, FontMatrics)> {
    let lib = Library::init().map_err(|error| {
        Error::LoadFontFaceError(format!(
            "message: Faild in Library init, real error: {}",
            error.to_string()
        ))
    })?;

    let face = lib
        .new_face("assets/fonts/Roboto-Regular.ttf", 0)
        .map_err(|error| {
            Error::LoadFontFaceError(format!(
                "message: Faild in Face (Font) creation, real error: {}",
                error.to_string()
            ))
        })?;
    face.set_char_size(0, 48 * 64, 0, 0).map_err(|error| {
        Error::LoadFontFaceError(format!(
            "message: Faild in set font size, real error: {}",
            error.to_string()
        ))
    })?;

    let characters_number = characters_number.unwrap_or(128);

    unsafe {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    }

    let mut characters = HashMap::with_capacity(characters_number as usize);

    for ascii_number in 0..characters_number {
        if let Some(ch) = char::from_u32(ascii_number) {
            face.load_char(ch as usize, LoadFlag::RENDER)
                .map_err(|error| {
                    Error::LoadCharacterError(format!(
                        "message: Unable to load {}, real error: {}",
                        ch,
                        error.to_string()
                    ))
                })?;

            let glyph = face.glyph();
            let bitmap = glyph.bitmap();

            let texture;
            unsafe {
                texture = Texture::new();
                texture.set_wrapping(gl::CLAMP_TO_EDGE);
                texture.set_filtering(gl::LINEAR);
                texture.load_bytes(
                    Size {
                        width: bitmap.width(),
                        height: bitmap.rows(),
                    },
                    bitmap.buffer(),
                );

                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl::Enable(gl::BLEND);
            }

            let character = Character::new(
                texture,
                Size {
                    width: bitmap.width() as f32,
                    height: bitmap.rows() as f32,
                },
                Position {
                    x: glyph.bitmap_left() as f32,
                    y: glyph.bitmap_top() as f32,
                },
                (glyph.advance().x >> 6) as f32,
            );

            characters.insert(ch, character);
        }
    }

    let size_metrics = face.size_metrics().unwrap();

    let line_height = (size_metrics.height >> 6) as f32;
    let descender = (size_metrics.descender >> 6) as f32;
    let ascender = (size_metrics.ascender >> 6) as f32;

    Ok((
        characters,
        FontMatrics::new(line_height, ascender, descender),
    ))
}

pub fn calculate_word_width(characters: &HashMap<char, Character>, word: &str, scale: f32) -> f32 {
    let mut sum_width = 0.0;

    for ch in word.chars() {
        assert!(characters.get(&ch) != None, "character must exist");

        let character = characters.get(&ch).unwrap();

        sum_width += character.advance * scale;
    }

    sum_width
}

pub fn calculate_text_size(
    characters: &HashMap<char, Character>,
    font_metrics: &FontMatrics,
    text: &str,
    start_position: Position,
    max_width: Option<f32>,
    scale: f32,
) -> (Size, Position) {
    let mut min_y = start_position.y;
    let mut max_y = start_position.y;

    let mut x = start_position.x;
    let mut y = start_position.y;

    let text_max_width = max_width.unwrap_or(0.0);

    let mut text_size = Size {
        width: text_max_width,
        height: 0.0,
    };

    let lines: Vec<&str> = text.split('\n').collect();

    for line in &lines {
        let mut is_new_word = false;

        for (idx, ch) in line.chars().enumerate() {
            if text_max_width > 0.0 && is_new_word {
                if let Some(found_index) = line[idx..].find(' ') {
                    let found_index = found_index + idx;

                    let word_width =
                        calculate_word_width(characters, &line[idx..found_index], scale);

                    if x + word_width >= start_position.x + text_max_width {
                        y += font_metrics.line_height * scale;
                        x = start_position.x;
                    }
                } else {
                    let word_width = calculate_word_width(characters, &line[idx..], scale);

                    if x + word_width >= start_position.x + text_max_width {
                        y += font_metrics.line_height * scale;
                        x = start_position.x;
                    }
                }
            }

            assert!(
                characters.get(&ch) != None,
                "character must exist, provided: {}",
                ch
            );

            let character = characters.get(&ch).unwrap();

            let character_start_position = Position {
                x: x + (character.offset.x * scale),
                y: y + (character.size.height - character.offset.y) * scale,
            };

            let character_size = Size {
                width: character.size.width * scale,
                height: character.size.height * scale,
            };

            if max_y > (character_start_position.y - character_size.height) {
                max_y = character_start_position.y - character_size.height;
            }

            if min_y < character_start_position.y {
                min_y = character_start_position.y;
            }

            is_new_word = ch == ' ';

            x += character.advance * scale;
        }

        if max_width.is_none() && text_size.width < (x - start_position.x) {
            text_size.width = x - start_position.x;
        }

        y += font_metrics.line_height * scale;
        x = start_position.x;
    }

    text_size.height = min_y + font_metrics.line_height * (lines.len() as f32 - 1.0) - max_y;

    return (
        text_size,
        Position {
            x: start_position.x,
            y: max_y,
        },
    );
}
