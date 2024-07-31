use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::level::GameObject;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Character<'a> {
    pub position: Position,
    size: Size,
    image: &'a str
}

impl<'a> Character<'a> {
    pub fn new(position: Position, size: Size, image: &'a str) -> Self {
        Self {
            position,
            size,
            image
        }
    }
}

impl GameObject for Character<'_> {
    fn draw(&self, render: &mut Render) -> Result<()> {
        render.load_image(self.image, self.position, self.size)?;

        Ok(())
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }
}


impl Character<'_> {
    pub fn move_character(&mut self, direction: Direction, speed: Option<f32>) {
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

    pub fn collide(&self, other: &impl GameObject) -> bool {
        let other_position = other.get_position();
        let other_size = other.get_size();

        self.position.x < other_position.x + other_size.width &&
        self.position.x + self.size.width > other_position.x &&
        self.position.y < other_position.y + other_size.height &&
        self.position.y + self.size.height > other_position.y
    }
}
