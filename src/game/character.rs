use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

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

impl Character<'_> {
    pub fn draw(&self, render: &Render, key: String) -> Result<()> {
        render.load_image(key, self.image, self.position, self.size)?;

        Ok(())
    }

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
}
