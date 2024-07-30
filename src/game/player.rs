use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::character::{Character, Direction};

pub struct Player<'a> {
    position: Position,
    size: Size,
    player_image: &'a str
}

impl Default for Player<'_> {
    fn default() -> Self {
        Self {
            position: Position { x: 10.0, y: 10.0 },
            size: Size { width: 50.0, height: 60.0 },
            player_image: "assets/game/detective.png"
        }
    }
}

impl Character for Player<'_> {
    fn draw(&self, render: &Render) -> Result<()> {
        render.load_image("player".to_string(), self.player_image, self.position, self.size)?;

        Ok(())
    }

    fn move_character(&mut self, direction: Direction,speed: Option<f32>) {
        let speed = speed.unwrap_or(10.0);

        match direction {
            Direction::Up => {
                self.position.y -= speed;
            },
            Direction::Down => {
                self.position.y += speed;
            },
            Direction::Left => {
                self.position.x -= speed;
            },
            Direction::Right => {
                self.position.x += speed;
            }
        }
    }

    fn get_position(&self) -> Position {
        self.position
    }
}
