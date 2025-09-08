use std::{collections::HashSet, time::{Duration, Instant}, usize};

use queues::{IsQueue, Queue};

use crate::{game::character::DEFAULT_CHARACTER_SIZE, library::{constants::DEFAULT_MOVEMENT_VALUE, utils::{absolute_f32, calc_equidistant_points, calc_mid_point_position_of_quadrilateral_shape, check_point_in_triangle, convert_angle_to_radians, round_position_to_full_numbers, simple_object_detect_check}}, renderer::{color::Color, error::Result, render::Render, styles::Size, vertice::{GridPosition, Position}}};

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
    last_updated_time: Instant,
    repeat_interval: Option<Duration>,
    original_repeat_interval: Option<Duration>,
    detect_traingle: DetectTraingle,
    looking_to: Direction,
    already_detected_player: bool,
    is_disturbed: bool,
    disturb_duration: Option<Duration>,
    is_destroyed: bool,
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
            last_updated_time: Instant::now(),
            repeat_interval: None,
            original_repeat_interval: None,
            detect_traingle: calc_detect_traingle(&position, flip, rotate),
            looking_to,
            already_detected_player: false,
            is_disturbed: false,
            disturb_duration: None,
            is_destroyed: false,
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
            last_updated_time: Instant::now(),
            repeat_interval: Some(interval),
            original_repeat_interval: Some(interval),
            detect_traingle: calc_detect_traingle(&position, flip, rotate),
            looking_to,
            already_detected_player: false,
            is_disturbed: false,
            disturb_duration: None,
            is_destroyed: false,
        }
    }
}

impl<'a> Camera<'a> {
    pub fn draw(&mut self, render: &mut Render<'a>) -> Result<()> {
        if self.is_destroyed {
            render.load_image("assets/game/destroy_camera.png", self.position, Size { height: 45.0, width: 40.0 }, self.flip, None, self.scale, None, self.rotate)?;

            return Ok(());
        }

        if self.is_disturbed {
            assert!(self.disturb_duration != None, "disturb_duration can not be none");

            if self.last_updated_time.elapsed() >= self.disturb_duration.unwrap() { 
                self.is_disturbed = false;
                self.disturb_duration = None;
                self.last_updated_time = Instant::now();
            } else {
                render.load_image("assets/game/disturb_camera.png", self.position, Size { height: 45.0, width: 40.0 }, self.flip, Some(0.6), self.scale, None, self.rotate)?;

                return Ok(());
            }
        }

        if self.repeat && self.last_updated_time.elapsed() >= self.repeat_interval.unwrap() {
            self.flip = !self.flip;

            let filp_factor = if self.flip { 1.0 } else { -1.0 };

            self.position = Position { x: self.position.x + (filp_factor * 12.0), y: self.position.y };

            self.set_detect_traingle();

            self.looking_to = if self.flip { Direction::Right } else { Direction::Left };
            self.last_updated_time = Instant::now();
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
    
    pub fn get_is_disturbed(&self) -> bool {
        self.is_disturbed
    }

    pub fn set_is_disturbed(&mut self, new_val: bool, disturb_duration: Option<Duration>) {
        self.is_disturbed = new_val;

        if new_val {
            assert!(disturb_duration != None, "disturb_duration can not be None");

            self.disturb_duration = disturb_duration;
        } else {
            self.disturb_duration = None;
        }
    }

     pub fn get_is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    pub fn destroy(&mut self) {
        self.is_destroyed = true;
    }

    pub fn set_new_repeat_interval(&mut self, notoriety_level: u64) {
        if let Some(original_repeat_interval) = self.original_repeat_interval {
            if notoriety_level >= 4 {
                if notoriety_level == 4 {
                    self.repeat_interval = Some(Duration::from_millis(original_repeat_interval.as_millis() as u64 - 25));
                } else {
                    self.repeat_interval = Some(Duration::from_millis(original_repeat_interval.as_millis() as u64 - 50));
                }
            }
        }
    }

    fn is_detecting_player(&self, player: &Player<'a>, walls: &[Wall<'a>], doors: &[Door<'a>]) -> bool {
        if (player.get_status() == &PlayerStatus::Hidden) || (self.is_disturbed || self.is_destroyed) {
            return false;
        }

        let (player_start, player_end) = player.get_calc_position(); 

        let camera_start = round_position_to_full_numbers(self.position, DEFAULT_MOVEMENT_VALUE, true, true);
        let camera_end = round_position_to_full_numbers(camera_start + self.size, DEFAULT_MOVEMENT_VALUE, true, true);

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

    fn get_nearest_enemy(&self, detect_player_position: &Position, enemies: &[Enemy<'a>]) -> isize {
        if enemies.len() == 0 {
            return -1;
        }

        let found_enemy = &enemies[0];

        let movement_grid = found_enemy.get_grid();
        assert!(movement_grid != &None, "movement_grid can not be none");
        let (grid_start_position, grid) = movement_grid.as_ref().unwrap();

        let start_position = detect_player_position.to_grid_position(*grid_start_position, DEFAULT_MOVEMENT_VALUE);

        let mut q: Queue<GridPosition> = Queue::new();
        q.add(start_position).unwrap();

        let mut visited: HashSet<GridPosition> = HashSet::new();
        visited.insert(start_position);

        let is_colliding_with_enemy = | start_position: Position, (enemy_start, enemy_end): (Position, Position) | -> bool {
            let end_position = start_position + DEFAULT_CHARACTER_SIZE;

            start_position.x <= enemy_end.x &&
            end_position.x >= enemy_start.x &&
            start_position.y <= enemy_end.y &&
            end_position.y >= enemy_start.y
        };

        let is_enemy = | position: Position | -> Option<usize> {
            for enemy in enemies {
                let (start, end) = enemy.get_calc_position();

                if is_colliding_with_enemy(position, (start, end)) {
                    return Some(enemy.get_id());
                }
            }

            None
        };

        while let Ok(current_position) = q.remove() {
            if let Some(enemy_id) = is_enemy(current_position.to_position(*grid_start_position, DEFAULT_MOVEMENT_VALUE)) {
                return enemy_id as isize;
            }

            for neighbor in current_position.get_neighbors() {
                if neighbor.row < grid.len() && neighbor.col < grid[0].len() && grid[neighbor.row][neighbor.col] {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        q.add(neighbor).unwrap();
                    }
                } 
            }
        }
        
        -1
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
                let nearest_enemy = if nearest_enemy_id == -1 {
                    None
                } else {
                    Some(nearest_enemy_id as usize)
                };
                
                if current_notoriety_level >= 5 {
                    return (5, nearest_enemy, Some(player_start));
                }
    
                return (current_notoriety_level + 1, nearest_enemy, Some(player_start));
            }

            return (current_notoriety_level, None, None);
        }

        (current_notoriety_level, None, None)
    }
}
