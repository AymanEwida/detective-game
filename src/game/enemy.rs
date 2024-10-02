use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, time::{Duration, Instant}};

use crate::{library::utils::{calc_equidistant_points, convert_path, get_heuristic_score, PathVec}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::{Character, Direction}, level::GameObject, player::{Player, PlayerStatus}};

pub const DEFAULT_MOVE_INTERVAL: Duration = Duration::from_millis(300);

#[derive(Debug, PartialEq, Eq)]
struct PossibilityNode {
    pub position: Position,
    pub priority_score: i32
}

impl Ord for PossibilityNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority_score.cmp(&self.priority_score)
    }
}

impl PartialOrd for PossibilityNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
    start_searching_position: Option<Position>,
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
                    start_searching_position: None,
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

        // render.draw_line(position { x: self.start_position.x + 27.5, y: self.start_position.y }, position { x: self.start_position.x + 27.5, y: self.start_position.y + 50.0 + (3.0 * self.size.height / 5.0) }, color::blue, none, none, none);
        // render.draw_line(Position { x: self.start_position.x + 27.5, y: self.start_position.y + 50.0 + (3.0 * self.size.height / 5.0) }, Position { x: self.start_position.x + 50.0 + self.size.width, y: self.start_position.y + 50.0 + (3.0 * self.size.height / 5.0) }, Color::Blue, None, None, None);

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

    pub fn find_optimal_path(&self, target_position: Position, speed: f32, grid: Vec<Vec<bool>>) -> Option<PathVec> {
        if self.position == target_position {
            return None;
        }

        let mut movements = BinaryHeap::new();
        let mut came_from: HashMap<Position, Option<Position>> = HashMap::new();
        let mut position_score = HashMap::new();

        movements.push(PossibilityNode {
            position: self.position,
            priority_score: 0
        });
        came_from.insert(self.position, None);
        position_score.insert(self.position, 0);

        while let Some(current_movement) = movements.pop() {
            let current_position = current_movement.position;

            if current_position == target_position {
                let mut path = vec![current_position];
                
                let mut current = current_position;

                while let Some(previous) = came_from[&current] {
                    path.push(previous);
                    current = previous;
                }

                path.reverse();

                let mut moves = Vec::new();

                current = self.position;

                for i in 1..path.len() {
                    let current_position = path[i];

                    if current.y == current_position.y {
                        if current.x > current_position.x {
                            moves.push((1, Direction::Left, 0));                            
                        } else {
                            moves.push((1, Direction::Right, 0));
                        }
                    } else if current.x == current_position.x {
                        if current.y > current_position.y {
                            moves.push((1, Direction::Up, 0));
                        } else {
                            moves.push((1, Direction::Down, 0));
                        }
                    }

                    current = current_position;
                }

                return Some(moves);
            }

            for neighbor in current_position.get_neighbors(speed) {
                // mabye remove position check

                let neighbor_col = neighbor.x / speed;
                let neighbor_row = neighbor.y / speed;

                if neighbor_col >= 0.0 && neighbor_col < grid[0].len() as f32 &&
                   neighbor_row >= 0.0 && neighbor_row < grid.len() as f32 &&
                   grid[neighbor_row as usize][neighbor_col as usize] {
                    let tentative_score = position_score[&current_position] + speed as i32; 

                    if tentative_score < *position_score.get(&neighbor).unwrap_or(&i32::MAX) {
                        came_from.insert(neighbor, Some(current_position));
                        position_score.insert(neighbor, tentative_score);
                        movements.push(PossibilityNode {
                            position: neighbor,
                            priority_score: tentative_score + get_heuristic_score(&neighbor, &target_position, speed) as i32
                        });
                    }
                } 
            }
        }

        None
    }

    pub fn move_enemy(&mut self, player: &mut Player<'a>, current_notoriety_level: u64, speed: Option<f32>) {
        match self.mode {
            EnemyMode::Regular => {
                self.already_detected_player = false;

                if self.prev_mode != EnemyMode::Regular {
                    if self.move_interval != DEFAULT_MOVE_INTERVAL {
                        self.move_interval = DEFAULT_MOVE_INTERVAL;
                    }

                    if self.position != self.start_position {
                        self.current_moves_path = self.find_optimal_path(self.start_position, speed.unwrap_or(10.0), vec![vec![]]).unwrap_or(Vec::new());
                    } else {
                        self.moves_count = 0;

                        self.current_moves_path = convert_path(self.original_moves_path);

                        self.prev_mode = EnemyMode::Regular;
                    }
                }

                self.move_enemy_in_path(None, speed);
            },

            EnemyMode::Detecting => {
                if let Some(detect_player_position) = self.detect_player_position {
                    if self.position == detect_player_position && !self.is_detecting_player(player) {
                        self.start_searching_position = Some(self.position);
                        self.detect_player_position = None;
                    } else {
                        if self.prev_mode != EnemyMode::Detecting && self.move_interval != DEFAULT_MOVE_INTERVAL {
                            self.move_interval = Duration::from_millis(1500);
                        }
    
                        self.current_moves_path = self.find_optimal_path(detect_player_position, speed.unwrap_or(10.0), vec![vec![]]).unwrap_or(Vec::new());
    
                        self.move_enemy_in_path(Some(Duration::from_millis(300 - (current_notoriety_level * 50))), speed);
                    }
                } else {
                    if player.get_status() == &PlayerStatus::Detectit {
                        player.set_status(PlayerStatus::NotHidden);
                    }

                    self.mode = EnemyMode::Searching;
                }

                self.already_detected_player = true;
                self.prev_mode = EnemyMode::Detecting;
            },

            EnemyMode::Searching => {
                if let Some(start_searching_position) = self.start_searching_position {
                    if self.prev_mode != EnemyMode::Searching {
                        if self.move_interval != DEFAULT_MOVE_INTERVAL {
                            self.move_interval = DEFAULT_MOVE_INTERVAL;
                        }
    
                        self.current_moves_path = self.get_searching_path(speed.unwrap_or(10.0) as u32);
                    } else if self.position == (Position { x: start_searching_position.x + ((50.0 / speed.unwrap_or(10.0)) * speed.unwrap_or(10.0)), y: start_searching_position.y }) && !self.is_detecting_player(player) {
                        self.mode = EnemyMode::Regular;
                    } else {
                        self.move_enemy_in_path(None, speed);
                    }
                } else {
                    self.mode = EnemyMode::Regular;
                }

                self.already_detected_player = true;
                self.prev_mode = EnemyMode::Searching;
            }
        }
    }

    fn get_searching_path(&self, speed: u32) -> PathVec {
        let steps = 50 / speed;

        vec![(steps, Direction::Left, 0), (steps, Direction::Right, 0), (steps, Direction::Up, 0), (steps * 2, Direction::Down, 0), (steps, Direction::Up, 0), (steps, Direction::Right, 0)]
    } 

    fn is_detecting_player(&self, player: &Player<'a>) -> bool {
        if player.get_status() == &PlayerStatus::Hidden {
            return false;
        }

        let player_start = player.get_position();
        let player_size = player.get_size();
        let player_end = Position { x: player_start.x + player_size.width, y: player_start.y + player_size.height };

        let enemy_end = Position { x: self.position.x + self.size.width, y: self.position.y + self.size.height };

        if ((player_start.x >= self.position.x && player_start.x <= enemy_end.x) ||
            (player_end.x >= self.position.x && player_end.x <= enemy_end.x)) &&
            ((player_start.y >= self.position.y && player_start.y < enemy_end.y) || 
            (player_end.y > self.position.y && player_end.y <= enemy_end.y)) {
            return true;
        }

        let (first_point, second_point, apex) = self.detect_traingle;

        match self.moving_towards {
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
