use crate::renderer::vertice::Position;

use super::level::GameObject;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
pub trait Character<'a>: GameObject<'a> {
    fn move_character(&mut self, direction: Direction, speed: Option<f32>) {
        let current_position = self.get_position();
        
        let speed = speed.unwrap_or(10.0);

        match direction {
            Direction::Up => {
                self.set_position(Position { x: current_position.x, y: current_position.y-speed });
            },
            Direction::Down => {
                self.set_position(Position { x: current_position.x, y: current_position.y+speed });
            },
            Direction::Left => {
                self.set_position(Position { x: current_position.x-speed, y: current_position.y });
            },
            Direction::Right => {
                self.set_position(Position { x: current_position.x+speed, y: current_position.y });
            }
        }
    }

    fn collide(&self, other: &impl GameObject<'a>) -> bool {
        let character_position = self.get_position();
        let character_size = self.get_size();

        let other_position = other.get_position();
        let other_size = other.get_size();
    
        character_position.x < other_position.x + other_size.width &&
        character_position.x + character_size.width > other_position.x &&
        character_position.y < other_position.y + other_size.height &&
        character_position.y + character_size.height > other_position.y
    }

    fn set_position(&mut self, new_position: Position);
}
