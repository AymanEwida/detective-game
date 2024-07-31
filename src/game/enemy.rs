use std::time::{Duration, Instant};

use crate::{library::utils::convert_path, renderer::{error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::{Character, Direction}, level::GameObject};

pub enum EnemyType {
    Regular
}

pub struct Enemy<'a> {
    character: Character<'a>,
    last_move_time: Instant,
    move_interval: Duration,
    moves_path: Vec<(u32, Direction)>,
    moves_count: u32,
}

impl Enemy<'_> {
    pub fn new(enemy_type: EnemyType, position: Position, path: &str) -> Self {
        match enemy_type {
            EnemyType::Regular => {
                Self {
                    character: Character::new(position, Size { width: 55.0, height: 60.0 }, "assets/game/regular-enemy.png"),
                    last_move_time: Instant::now(),
                    move_interval: Duration::from_millis(300),
                    moves_path: convert_path(path),
                    moves_count: 0
                }
            }
        }
    }
}

impl Enemy<'_> {
    pub fn draw(&self, render: &mut Render) -> Result<()> {
        self.character.draw(render)?;

        Ok(())
    }

    pub fn move_enemy(&mut self, speed: Option<f32>) {
        if self.last_move_time.elapsed() >= self.move_interval {
            if let Some((moves_number, direction)) = self.moves_path.first() {
                if *moves_number > 0 {
                    self.character.move_character(*direction, speed);

                    self.moves_path[0].0 -= 1;
                    self.moves_count += 1;
                } else {
                    self.moves_path[0].0 = self.moves_count;
                    self.moves_path.rotate_left(1);

                    self.moves_count = 0;

                    self.move_enemy(speed);
                }

                self.last_move_time = Instant::now();
            }
        }
    }

    pub fn collide(&self, other: &impl GameObject) -> bool {
        self.character.collide(other)
    }
}
