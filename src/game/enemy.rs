use crate::{library::utils::convert_path, renderer::{error::Result, render::{Render, Size}, vertice::Position}};

use super::character::Character;

pub enum EnemyType {
    Regular
}

pub struct Enemy<'a> {
    pub character: Character<'a>
}

impl Enemy<'_> {
    pub fn new(enemy_type: EnemyType, position: Position) -> Self {
        match enemy_type {
            EnemyType::Regular => {
                Self {
                    character: Character::new(position, Size { width: 50.0, height: 60.0 }, "assets/game/regular-enemy.png")
                }
            }
        }
    }
}

impl Enemy<'_> {
    pub fn draw(&self, render: &Render) -> Result<()> {
        self.character.draw(render, "enemy".to_string())?;

        Ok(())
    }

    pub fn move_enemy(&mut self, path: &str, speed: Option<f32>) {
        let moves = convert_path(path);

        for (moves_number, direction) in moves {
            for _ in 0..moves_number {
                self.character.move_character(direction, speed);
            }
        }
    }

    pub fn get_position(&self) -> Position {
        self.character.position
    }
}
