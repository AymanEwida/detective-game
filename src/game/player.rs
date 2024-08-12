use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::{character::{Character, Direction}, level::GameObject};

pub struct Player<'a> {
    position: Position,
    prev_position: Option<Position>,
    size: Size,
    image: &'a str
}

impl Player<'_> {
    pub fn new(start_position: Position) -> Self {
        Self {
            position: start_position, 
            prev_position: None,
            size: Size { width: 50.0, height: 60.0 },
            image: "assets/game/detective.png"
        }
    }
}

impl<'a> GameObject<'a> for Player<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, None)?;

        Ok(())
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }
}

impl<'a> Character<'a> for Player<'a> {
    fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
    }
}

impl<'a> Player<'a> {
    pub fn move_player(&mut self, direction: Direction, speed: Option<f32>) {
        self.prev_position = Some(self.get_position());

        self.move_character(direction, speed);
    }

    pub fn move_to_prev_position(&mut self) {
        if let Some(prev_position) = self.prev_position {
            self.position = prev_position;
        }
    }

    pub fn get_prev_position(&self) -> Option<Position> {
        self.prev_position
    }

    pub fn move_to(&mut self, new_position: Position) {
        self.position = new_position;
    }

    pub fn is_off_window(&self, window_size: Size) -> bool {
        self.position.x > window_size.width ||
        (self.position.x + self.size.width) > window_size.width ||
        self.position.x < 0.0 ||
        self.position.y > window_size.height ||
        (self.position.y + self.size.height) > window_size.height ||
        self.position.y < 0.0
    }
}
