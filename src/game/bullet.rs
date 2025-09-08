use crate::{game::character::Direction, renderer::{error::Result, render::Render, styles::Size, vertice::Position}};

use super::{camera::Camera, level::GameObject, player::DEFAULT_SIZE_FOR_INVENTORY_ITEM};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BulletType {
    CameraGunBullet,
    Other
}

#[derive(Debug, Clone)]
pub struct Bullet<'a> {
    bullet_type: BulletType,
    damage_on_enemy: u8,
    image: &'a str,
    size: Size,
    calc_size: Size,
    start_position: Position,
    velocity: (f32, f32),
    current_position: Position,
    rotate: Option<f32>,
    is_finished: bool
}

impl<'a> Bullet<'a> {
    pub fn new(bullet_type: BulletType, damage_on_enemy: u8, image: &'a str, size: Option<Size>, start_position: Position, end_position: Position, speed: f32) -> Self {
        let dir_x = end_position.x - start_position.x;
        let dir_y = end_position.y - start_position.y;

        let dir_vec = Position {
            x: dir_x,
            y: dir_y,
        }.normalize(&Position { x: 0.0, y: 0.0 });

        let velocity = (dir_vec.x * speed, dir_vec.y * speed);

        let angle = (-dir_y).atan2(dir_x).to_degrees();

        let size = size.unwrap_or(DEFAULT_SIZE_FOR_INVENTORY_ITEM);

        Self {
            bullet_type,
            damage_on_enemy,
            image,
            size,
            calc_size: Size { width: 10.0, height: 10.0 },
            start_position,
            velocity,
            current_position: start_position,
            rotate: Some(angle),
            is_finished: false,
        }
    }

    pub fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        if !self.is_finished {
            render.load_image(self.image, self.current_position, DEFAULT_SIZE_FOR_INVENTORY_ITEM, false, None, None, None, self.rotate)?;
        }

        Ok(())
    }
}

impl<'a> Bullet<'a> {
    pub fn get_bullet_type(&self) -> &BulletType {
        &self.bullet_type
    }

    pub fn get_is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn set_is_finished(&mut self, new_val: bool) {
        self.is_finished = new_val;
    }

    pub fn get_damage_on_enemy(&self) -> u8 {
        self.damage_on_enemy
    }

    pub fn get_start_position(&self) -> Position {
        self.start_position
    }

    pub fn get_calc_size(&self) -> Size {
        self.calc_size
    }

    pub fn calc_next_position(&mut self) {
        self.current_position = Position {
            x: self.current_position.x + self.velocity.0,
            y: self.current_position.y + self.velocity.1
        }
    }

    pub fn collide(&self, other: &impl GameObject<'a>) -> bool {
        let (start_position, end_position) = (
            self.current_position,
            self.current_position + self.calc_size
        );
        let (other_start_position, other_end_position) = other.get_calc_position();
    
        start_position.x < other_end_position.x &&
        end_position.x > other_start_position.x &&
        start_position.y < other_end_position.y &&
        end_position.y > other_start_position.y
    }

    pub fn collide_with_camera(&self, camera: &Camera<'a>) -> bool {
        let (start_position, end_position) = (
            self.current_position,
            self.current_position + self.calc_size
        );
        let (mut camera_start_position, mut camera_end_position) = (
            camera.get_position(),
            camera.get_position() + camera.get_size()
        );

        let colliding = | camera_start: &Position, camera_end: &Position | -> bool {
            start_position.x <= camera_end.x &&
            end_position.x >= camera_start.x &&
            start_position.y <= camera_end.y &&
            end_position.y >= camera_start.y
        };

        let mut is_colliding = colliding(&camera_start_position, &camera_end_position);

        let directions = [Direction::Down, Direction::Left, Direction::Right, Direction::Up];

        for direction in directions.into_iter() {
            if is_colliding {
                break;
            }

            match direction {
                Direction::Up => {
                    camera_start_position = Position {
                        y: camera.get_position().y - camera.get_size().height,
                        ..camera.get_position()
                    };
                },

                Direction::Down => {
                    camera_start_position = Position {
                        y: camera.get_position().y + camera.get_size().height,
                        ..camera.get_position()
                    };
                },

                Direction::Left => {
                    camera_start_position = Position {
                        x: camera.get_position().x - camera.get_size().width,
                        ..camera.get_position()
                    };
                },

                Direction::Right => {
                    camera_start_position = Position {
                        x: camera.get_position().x + camera.get_size().width,
                        ..camera.get_position()
                    };
                },
            }

            camera_end_position = camera_start_position + camera.get_size();

            is_colliding = colliding(&camera_start_position, &camera_end_position);
        }

        is_colliding
    }

    pub fn is_off_border(&self, start_position: Option<Position>, size: Size) -> bool {
        let start_position = start_position.unwrap_or(Position { x: 0.0, y: 0.0 });

        self.current_position.x > (start_position.x + size.width) ||
        (self.current_position.x + self.size.width) > (start_position.x + size.width) ||
        self.current_position.x < start_position.x ||
        self.current_position.y > (start_position.y + size.height) ||
        (self.current_position.y + self.size.height) > (start_position.y + size.height) ||
        self.current_position.y < start_position.y
    }

    
}

