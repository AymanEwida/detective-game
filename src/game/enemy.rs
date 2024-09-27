use std::time::{Duration, Instant};

use crate::{library::utils::{calc_equidistant_points, convert_path, get_optimal_path, PathVec}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::{Character, Direction}, level::GameObject, player::{Player, PlayerStatus}};

pub const DEFAULT_MOVE_INTERVAL: Duration = Duration::from_millis(300);

#[derive(Debug)]
pub enum EnemyType {
    Regular
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EnemyMode {
    Regular,
    Detecting,
    Searching
}

#[derive(Debug)]
pub struct Enemy<'a> {
    start_position: Position,
    position: Position,
    size: Size,
    image: &'a str,
    flip: bool,
    last_move_time: Instant,
    move_interval: Duration,
    current_moves_path: PathVec,
    original_moves_path: &'a str,
    moves_count: u32,
    moving_towards: Direction,
    detect_traingle: (Position, Position, Position),
    detect_player_position: Option<Position>,
    mode: EnemyMode,
    prev_mode: EnemyMode,
    already_detected_player: bool,
}

impl<'a> Enemy<'a> {
    pub fn new(enemy_type: EnemyType, start_position: Position, path: &'a str, flip: bool) -> Self {
        let moves_path = convert_path(path);
        let first_direction = moves_path[0].1;

        match enemy_type {
            EnemyType::Regular => {
                Self {
                    start_position: start_position,
                    position: start_position,
                    size: Size { width: 55.0, height: 60.0 },
                    image: "assets/game/regular-enemy.png",
                    flip,
                    last_move_time: Instant::now(),
                    move_interval: DEFAULT_MOVE_INTERVAL,
                    original_moves_path: path,
                    current_moves_path: moves_path,
                    moves_count: 0,
                    moving_towards: first_direction,
                    detect_traingle: calc_equidistant_points(Position { x: start_position.x + 27.5, y: start_position.y + 20.0 }, 30.0, 150.0, first_direction),
                    detect_player_position: None,
                    mode: EnemyMode::Regular,
                    prev_mode: EnemyMode::Regular,
                    already_detected_player: false,
                }
            }
        }
    }
}

impl<'a> GameObject<'a> for Enemy<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, self.flip, None, None, None)?;

        let (first_point, second_point, apex) = self.detect_traingle;

        render.draw_line(apex, first_point, Color::Red, None, None, None);
        render.draw_line(apex, second_point, Color::Red, None, None, None);
        render.draw_line(first_point, second_point, Color::Red, None, None, None); 

        Ok(())
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
    }

    fn get_size(&self) -> Size {
        self.size
    }
}

impl<'a> Character<'a> for Enemy<'a> {
    fn set_flip(&mut self, new_value: bool) {
        self.flip = new_value;
    }
}

impl<'a> Enemy<'a> {
    pub fn get_start_position(&self) -> Position {
        self.start_position
    }

    pub fn _set_start_position(&mut self, _new_start_position: Position) {
        self.start_position = _new_start_position;
    }
    
    fn move_enemy_in_path(&mut self, move_interval: Option<Duration>, speed: Option<f32>) {
        if self.last_move_time.elapsed() >= self.move_interval {
            if let Some((moves_number, direction, wait_time)) = self.current_moves_path.first() {
                if *moves_number > 0 {
                    self.moving_towards = *direction;

                    self.move_character(*direction, speed);

                    self.current_moves_path[0].0 -= 1;
                    self.moves_count += 1;

                    if self.moves_count == 1 {
                        self.move_interval = move_interval.unwrap_or(DEFAULT_MOVE_INTERVAL);
                    }
                } else {
                    if *wait_time == 0 {
                        self.move_interval = move_interval.unwrap_or(DEFAULT_MOVE_INTERVAL);
                    } else {
                        self.move_interval = Duration::from_millis(*wait_time);
                    }

                    self.current_moves_path[0].0 = self.moves_count;
                    self.current_moves_path.rotate_left(1);

                    self.moves_count = 0;

                    self.move_enemy_in_path(move_interval, speed);
                }

                self.detect_traingle = calc_equidistant_points(Position { x: self.position.x + 27.5, y: self.position.y + 20.0 }, 30.0, 150.0, self.moving_towards);
                self.last_move_time = Instant::now();
            }
        }
    }

    pub fn move_enemy(&mut self, current_notoriety_level: u64, speed: Option<f32>) {
        match self.mode {
            EnemyMode::Regular => {
                self.already_detected_player = false;

                if self.prev_mode != EnemyMode::Regular {
                    if self.move_interval != DEFAULT_MOVE_INTERVAL {
                        self.move_interval = DEFAULT_MOVE_INTERVAL;
                    }

                    if self.position != self.start_position {
                        self.current_moves_path = get_optimal_path(&self.position, &self.start_position, speed.unwrap_or(10.0) as u32);
                    } else {
                        self.current_moves_path = convert_path(self.original_moves_path);

                        self.prev_mode = EnemyMode::Regular;
                    }
                }

                self.move_enemy_in_path(None, speed);
            },

            EnemyMode::Detecting => {
                if let Some(detect_player_position) = self.detect_player_position {
                    if self.prev_mode != EnemyMode::Detecting && self.move_interval != DEFAULT_MOVE_INTERVAL {
                        self.move_interval = Duration::from_millis(1500);
                    }

                    self.current_moves_path = get_optimal_path(&self.position, &detect_player_position, speed.unwrap_or(10.0) as u32);

                    self.move_enemy_in_path(Some(Duration::from_millis(300 - (current_notoriety_level * 50))), speed);
                } else {
                    self.mode = EnemyMode::Regular;
                }

                self.already_detected_player = true;
                self.prev_mode = EnemyMode::Detecting;
            },

            EnemyMode::Searching => {
                self.already_detected_player = true;
                self.prev_mode = EnemyMode::Searching;
            }
        }
    }

    pub fn detect_player(&mut self, current_notoriety_level: u64, player: &mut Player<'a>) -> u64 {
        if player.get_status() != &PlayerStatus::Hidden {
            let player_start = player.get_position();
            let player_size = player.get_size();
            let player_end = Position { x: player_start.x + player_size.width, y: player_start.y + player_size.height };

            let enemy_end = Position { x: self.position.x + self.size.width, y: self.position.y + self.size.height };

            if ((player_start.x >= self.position.x && player_start.x <= enemy_end.x) ||
                (player_end.x >= self.position.x && player_end.x <= enemy_end.x)) &&
                ((player_start.y >= self.position.y && player_start.y < enemy_end.y) || 
                (player_end.y > self.position.y && player_end.y <= enemy_end.y)) {
                player.set_status(PlayerStatus::Detectit);
                self.mode = EnemyMode::Detecting;
                self.detect_player_position = Some(player_start);
                
                if !self.already_detected_player {
                    if current_notoriety_level >= 3 {
                        return 3;
                    }
                    
                    return current_notoriety_level + 1;
                }

                return current_notoriety_level;
            }

            let (first_point, second_point, apex) = self.detect_traingle;

            let is_detected = match self.moving_towards {
                Direction::Left => {
                    ((player_start.x >= first_point.x && player_start.x < apex.x) ||
                    (player_end.x > first_point.x && player_end.x <= apex.x)) &&
                    ((player_start.y < first_point.y && player_start.y >= second_point.y) ||
                    (player_end.y <= first_point.y && player_end.y > second_point.y)) 
                },

                Direction::Right => {
                    ((player_start.x < first_point.x && player_start.x >= apex.x) ||
                    (player_end.x <= first_point.x && player_end.x > apex.x)) &&
                    ((player_start.y < first_point.y && player_start.y >= second_point.y) ||
                    (player_end.y <= first_point.y && player_end.y > second_point.y))
                },

                Direction::Up => {
                    ((player_start.x >= second_point.x && player_start.x < first_point.x) ||
                    (player_end.x > second_point.x && player_end.x <= first_point.x)) &&
                    ((player_start.y >= first_point.y && player_start.y < apex.y) ||
                    (player_end.y > first_point.y && player_end.y <= apex.y)) 
                },

                Direction::Down => {
                    ((player_start.x >= second_point.x && player_start.x < first_point.x) ||
                    (player_end.x > second_point.x && player_end.x <= first_point.x)) &&
                    ((player_start.y < first_point.y && player_start.y >= apex.y) ||
                    (player_end.y <= first_point.y && player_end.y > apex.y)) 
                },
            };

            if is_detected {
                player.set_status(PlayerStatus::Detectit);
                self.mode = EnemyMode::Detecting;
                self.detect_player_position = Some(player_start);

                if !self.already_detected_player {
                    if current_notoriety_level >= 3 {
                        return 3;
                    }
    
                    return current_notoriety_level + 1;
                }

                return current_notoriety_level;
            }
        }

        current_notoriety_level
    }

    pub fn collide_with_player(&self, player: &Player<'a>) -> bool {
        if self.mode == EnemyMode::Regular || player.get_status() != &PlayerStatus::Detectit {
            return false;
        }

        let player_position = player.get_position();
        let player_size = player.get_size();

        self.position.x < player_position.x + player_size.width &&
        self.position.x + self.size.width > player_position.x &&
        self.position.y < player_position.y + player_size.height &&
        self.position.y + self.size.height > player_position.y
    }

    pub fn get_mode(&mut self) -> &EnemyMode {
        &self.mode
    }
}
