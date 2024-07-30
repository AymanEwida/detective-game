use crate::renderer::{error::Result, render::Render, vertice::Position};

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub trait Character {
    fn draw(&self, render: &Render) -> Result<()>;
    fn move_character(&mut self, direction: Direction, speed: Option<f32>);
    fn get_position(&self) -> Position;
}