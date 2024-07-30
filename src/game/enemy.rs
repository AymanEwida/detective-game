use std::{thread::sleep, time::Duration};

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

    pub fn move_enemy(&mut self, gl_context: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>, render: &Render, path: &str, speed: Option<f32>) {
        let moves = convert_path(path);

        for (moves_number, direction) in moves {
            for _ in 0..moves_number {
                self.character.move_character(direction, speed);

                self.draw(render).expect("Unable to draw enemy");

                render.render();

                gl_context.swap_buffers().unwrap();

                sleep(Duration::from_millis(300));
            }
        }
    }

    pub fn get_position(&self) -> Position {
        self.character.position
    }
}
