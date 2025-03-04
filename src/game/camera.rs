use std::time::{Duration, Instant};

use crate::{library::{constants::DEFAULT_MOVEMENT_VALUE, utils::{absolute_f32, calc_equidistant_points, calc_mid_point_position_of_quadrilateral_shape, check_point_in_triangle, convert_angle_to_radians, get_moves_number, round_position_to_full_numbers, simple_object_detect_check}}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

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

        let y_offset = absolute_f32(position.y - apex.y);        
        if y_offset >= 25.0 {
            let sing = if position.y > apex.y { 1.0 } else { -1.0 }; 

            apex.y += y_offset * sing;
        }

        let x_offset = absolute_f32(position.x - apex.x);        
        if x_offset >= 25.0 {
            let sing = if position.x > apex.x { 1.0 } else { -1.0 }; 

            apex.x += x_offset * sing;
        }

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

            self.set_detect_traingle();

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
        self.set_detect_traingle();
    }

    fn set_detect_traingle(&mut self) {
        self.detect_traingle = calc_detect_traingle(&self.position, self.flip, self.rotate);
    }

    pub fn get_size(&self) -> Size {
        self.size        
    }

    pub fn get_type(&self) -> ObjectType {
        ObjectType::Camera
    }

    pub fn get_already_detected_player(&self) -> bool {
        self.already_detected_player
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

        if simple_object_detect_check(player.get_calc_position(), (camera_start, camera_end), walls) 
            || simple_object_detect_check(player.get_calc_position(), (camera_start, camera_end), doors) {
            return false;
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

    fn get_nearest_enemy(&self, detect_player_position: &Position, enemies: &[Enemy<'a>]) -> usize {
        assert!(enemies.len() > 0, "enemies must contain at least one enemy");

        let mut found_enemy = &enemies[0];

        let movement_grid = found_enemy.get_grid();
        assert!(movement_grid != &None, "movement_grid can not be none");
        let (grid_start_position, grid) = movement_grid.as_ref().unwrap();

        let mut moves_num = -1;

        for enemy in enemies.iter() {
            let moves_path = enemy.find_optimal_path(*detect_player_position, *grid_start_position, &grid);
            if moves_path.is_none() {
                continue;
            }

            let moves_path = moves_path.unwrap();

            let current_moves_path_num = get_moves_number(moves_path) as i32;
            
            if current_moves_path_num <= 7 {
                return enemy.get_id();
            }

            if moves_num == -1 || ((current_moves_path_num) < moves_num) {
                moves_num = current_moves_path_num;

                found_enemy = enemy;
            }
        }

        found_enemy.get_id()
    }

    pub fn detect_player(&mut self, current_notoriety_level: u64, player: &mut Player<'a>, walls: &[Wall<'a>], doors: &[Door<'a>], enemies: &[Enemy<'a>]) -> (u64, Option<usize>, Option<Position>) {
        if self.already_detected_player && player.get_status() != &PlayerStatus::Detectit {
            self.already_detected_player = false;
        }

        let is_detecting = self.is_detecting_player(player, walls, doors);

        if is_detecting {
            let player_start = player.get_position();

            player.set_status(PlayerStatus::Detectit);
            player.set_is_detected_by_enemy(false);

            if !self.already_detected_player {
                self.already_detected_player = true;

                let nearest_enemy_id = self.get_nearest_enemy(&player_start, enemies);

                if current_notoriety_level >= 3 {
                    return (3, Some(nearest_enemy_id), Some(player_start));
                }
    
                return (current_notoriety_level + 1, Some(nearest_enemy_id), Some(player_start));
            }

            return (current_notoriety_level, None, None);
        }

        (current_notoriety_level, None, None)
    }
}
