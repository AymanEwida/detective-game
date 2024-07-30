use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::character::{Character, Direction};

pub struct Player<'a> {
    character: Character<'a>
}

impl Default for Player<'_> {
    fn default() -> Self {
        Self {
            character: Character::new(Position { x: 10.0, y: 10.0 }, Size { width: 50.0, height: 60.0 }, "assets/game/detective.png")
        }
    }
}

impl Player<'_> {
    pub fn draw(&self, render: &Render) -> Result<()> {
        self.character.draw(render, "player".to_string())?;

        Ok(())
    }

    pub fn move_player(&mut self, direction: Direction, speed: Option<f32>) {
        self.character.move_character(direction, speed);
    }

    pub fn get_position(&self) -> Position {
        self.character.position
    }
}
