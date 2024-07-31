use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::{character::{Character, Direction}, level::GameObject};

pub struct Player<'a> {
    character: Character<'a>,
    prev_position: Option<Position>
}

impl Default for Player<'_> {
    fn default() -> Self {
        Self {
            character: Character::new(Position { x: 10.0, y: 10.0 }, Size { width: 50.0, height: 60.0 }, "assets/game/detective.png"),
            prev_position: None
        }
    }
}

impl Player<'_> {
    pub fn draw(&self, render: &Render) -> Result<()> {
        self.character.draw(render, "player".to_string())?;

        Ok(())
    }

    pub fn move_player(&mut self, direction: Direction, speed: Option<f32>) {
        self.prev_position = Some(self.get_position());

        self.character.move_character(direction, speed);
    }

    pub fn move_player_to_prev_position(&mut self) {
        if let Some(prev_position) = self.prev_position {
            self.character.position = prev_position;
        }
    }

    pub fn get_prev_position(&self) -> Option<Position> {
        self.prev_position
    }

    pub fn get_position(&self) -> Position {
        self.character.get_position()
    }

    pub fn collide(&self, other: &impl GameObject) -> bool {
        self.character.collide(other)
    }
}
