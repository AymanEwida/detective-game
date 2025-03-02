use std::time::{Duration, Instant};

use crate::{library::{constants::DEFAULT_MOVEMENT_VALUE, utils::{calc_equidistant_points, calc_mid_point_position_of_quadrilateral_shape, check_point_in_triangle, convert_angle_to_radians, round_position_to_full_numbers}}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::Direction, door::Door, enemy::{DetectTraingle, Enemy}, level::GameObject, level_object::ObjectType, player::{Player, PlayerStatus}, wall::Wall};

pub const DEFAULT_SIZE_FOR_CAMERA: Size = Size { width: 30.0, height: 30.0 };
pub const DEFAULT_REPEAT_INTERVAL: Duration = Duration::from_millis(3000);

#[derive(Debug, Clone)]
pub struct Camera<'a> {
    position: Position,
    size: Size,
    image: &'a str,
    flip: bool,
    scale: Option<f32>,
    rotate: Option<f32>,
    repeat: bool,
    last_move_time: Instant,
    repeat_interval: Option<Duration>,
    detect_traingle: DetectTraingle,
    looking_to: Direction,
    already_detected_player: bool,
}

fn calc_detect_traingle(position: &Position, flip: bool, rotate: Option<f32>) -> DetectTraingle {
    let (direction, mut angle) = if flip {
        (Direction::Right, convert_angle_to_radians(45.0))
    } else { 
        (Direction::Left, convert_angle_to_radians(315.0))
    };

    let center = calc_mid_point_position_of_quadrilateral_shape(position, &DEFAULT_SIZE_FOR_CAMERA);

    let mut apex = Position { x: position.x + 15.0, y: position.y + 10.0 };

    if let Some(rotate_deg) = rotate {
        let rotate_deg = convert_angle_to_radians(rotate_deg);
        let calc_position = position.clone().rotate(center, rotate_deg);
        
        apex = Position { x: calc_position.x, y: calc_position.y };

        angle -= rotate_deg;
    }


    let mut detect_traingle = calc_equidistant_points(apex, 10.0, 150.0, direction);

    detect_traingle.0 = detect_traingle.0.rotate(center, angle);
    detect_traingle.1 = detect_traingle.1.rotate(center, angle);
    detect_traingle.2 = detect_traingle.2.rotate(center, angle);

    detect_traingle
}

impl Camera<'_> {
    pub fn new_without_repeat(position: Position, flip: bool, scale: Option<f32>, rotate: Option<f32>) -> Self {
        let looking_to = if flip { Direction::Right } else { Direction::Left };

        Self {
            position,
            size: DEFAULT_SIZE_FOR_CAMERA,
            image: "assets/game/camera.png",
            flip,
            scale,
            rotate,
            repeat: false,
            last_move_time: Instant::now(),
            repeat_interval: None,
            detect_traingle: calc_detect_traingle(&position, flip, rotate),
            looking_to,
            already_detected_player: false,
        }
    }

    pub fn new_with_repeat(position: Position, flip: bool, scale: Option<f32>, rotate: Option<f32>, repeat_time: Option<u64>) -> Self {
        let looking_to = if flip { Direction::Right } else { Direction::Left };

        let repeat_time = repeat_time.unwrap_or(0);
        
        let interval = if repeat_time <= 0 { DEFAULT_REPEAT_INTERVAL } else { Duration::from_millis(repeat_time) };

        Self {
            position,
            size: DEFAULT_SIZE_FOR_CAMERA,
            image: "assets/game/camera.png",
            flip,
            scale,
            rotate,
            repeat: true,
            last_move_time: Instant::now(),
            repeat_interval: Some(interval),
            detect_traingle: calc_detect_traingle(&position, flip, rotate),
            looking_to,
            already_detected_player: false,
        }
    }
}

impl<'a> Camera<'a> {
    pub fn draw(&mut self, render: &mut Render<'a>) -> Result<()> {
        if self.repeat && self.last_move_time.elapsed() >= self.repeat_interval.unwrap() {
            self.flip = !self.flip;

            let filp_factor = if self.flip { 1.0 } else { -1.0 };

            self.position = Position { x: self.position.x + (filp_factor * 12.0), y: self.position.y };

            self.detect_traingle = calc_detect_traingle(&self.position, self.flip, self.rotate);

            self.looking_to = if self.flip { Direction::Right } else { Direction::Left };
            self.last_move_time = Instant::now();
        }

        let (first_point, second_point, apex) = self.detect_traingle;

        render.draw_line(apex, first_point, Color::Blue, None, None, None);
        render.draw_line(apex, second_point, Color::Blue, None, None, None);
        render.draw_line(first_point, second_point, Color::Blue, None, None, None); 

        render.load_image(self.image, self.position, self.size, self.flip, None, self.scale, None, self.rotate)?;

        Ok(())
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
    }

    pub fn get_size(&self) -> Size {
        self.size        
    }

    pub fn get_type(&self) -> ObjectType {
        ObjectType::Camera
    }

    fn is_detecting_player(&self, player: &Player<'a>, walls: &[Wall<'a>], doors: &[Door<'a>]) -> bool {
        if player.get_status() == &PlayerStatus::Hidden {
            return false;
        }

        let (player_start, player_end) = player.get_calc_position(); 

        let camera_start = round_position_to_full_numbers(self.position, DEFAULT_MOVEMENT_VALUE, true, true);
        let camera_end = round_position_to_full_numbers(camera_start + self.size, DEFAULT_MOVEMENT_VALUE, true, true);

        if ((player_start.x >= camera_start.x && player_start.x <= camera_end.x) ||
            (player_end.x >= camera_start.x && player_end.x <= camera_end.x)) &&
            ((player_start.y >= camera_start.y && player_start.y < camera_end.y) || 
            (player_end.y > camera_start.y && player_end.y <= camera_end.y)) {
            return true;
        }

        let (first_point, second_point, apex) = self.detect_traingle;

        let first_point = round_position_to_full_numbers(first_point, DEFAULT_MOVEMENT_VALUE, false, false);
        let second_point = round_position_to_full_numbers(second_point, DEFAULT_MOVEMENT_VALUE, false, false);
        let apex = round_position_to_full_numbers(apex, DEFAULT_MOVEMENT_VALUE, false, false);
        
        check_point_in_triangle(&player_start, &first_point, &second_point, &apex) || 
        check_point_in_triangle(&Position { x: player_end.x, y: player_start.y }, &first_point, &second_point, &apex) ||
        check_point_in_triangle(&player_end, &first_point, &second_point, &apex) ||
        check_point_in_triangle(&Position { x: player_start.x, y: player_end.y }, &first_point, &second_point, &apex)
    }

    // TODO: find a way to get the nearest enemy and update it feilds
    fn get_nearest_enemy(&self, enemies: &'a mut [Enemy<'a>]) -> &mut Enemy<'a> {
        let first = &mut enemies[0];

        first
    }

    pub fn detect_player(&mut self, current_notoriety_level: u64, player: &mut Player<'a>, walls: &[Wall<'a>], doors: &[Door<'a>], enemies: &[Enemy<'a>]) -> u64 {
        let is_detecting = self.is_detecting_player(player, walls, doors);

        if is_detecting {
            let player_start = player.get_position();

            player.set_status(PlayerStatus::Detectit);

            if !self.already_detected_player {
                self.already_detected_player = true;

                // let nearest_enemy = self.get_nearest_enemy(enemies);
                // nearest_enemy.attach_camera(player_start); 

                if current_notoriety_level >= 3 {
                    return 3;
                }
    
                return current_notoriety_level + 1;
            }

            return current_notoriety_level;
        }

        current_notoriety_level
    }
}
