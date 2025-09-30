use crate::renderer::{styles::Size, vertice::Position};

use super::level::GameObject;

pub const DEFAULT_CHARACTER_SIZE: Size = Size { width: 50.0, height: 60.0 };

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub trait Character<'a>: GameObject<'a> {
    fn move_character(&mut self, direction: Direction, value: f32) {
        let current_position = self.get_position();
        
        match direction {
            Direction::Up => {
                self.set_position(Position { x: current_position.x, y: current_position.y - value });
            },
            Direction::Down => {
                self.set_position(Position { x: current_position.x, y: current_position.y + value });
            },
            Direction::Left => {
                self.set_flip(false);

                self.set_position(Position { x: current_position.x - value, y: current_position.y });
            },
            Direction::Right => {
                self.set_flip(true);
                
                self.set_position(Position { x: current_position.x + value, y: current_position.y });
            }
        }
    }

    fn collide(&self, other: &impl GameObject<'a>) -> bool {
        let (character_start_position, character_end_position) = self.get_calc_position();
        let (other_start_position, other_end_position) = other.get_calc_position();
    
        character_start_position.x < other_end_position.x &&
        character_end_position.x > other_start_position.x &&
        character_start_position.y < other_end_position.y &&
        character_end_position.y > other_start_position.y
    }

    fn set_flip(&mut self, new_value: bool);
}
