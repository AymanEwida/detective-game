use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, time::{Duration, Instant}};

use crate::{game::level_object::ObjectType, library::utils::{absolute_f32, calc_equidistant_points, convert_path, get_heuristic_score, sum_direction_length_from_path, PathVec}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::{Character, Direction}, level::GameObject, level_object::LevelObject, player::{Player, PlayerStatus}};

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
    movement_value: f32,
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
                    movement_value: 10.0,
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
    
    fn move_enemy_in_path(&mut self, move_interval: Option<Duration>) {
        if self.last_move_time.elapsed() >= self.move_interval {
            if let Some((moves_number, direction, wait_time)) = self.current_moves_path.first() {
                if *moves_number > 0 {
                    self.moving_towards = *direction;

                    self.move_character(*direction, self.movement_value);

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

                    self.move_enemy_in_path(move_interval);
                }

                self.detect_traingle = calc_equidistant_points(Position { x: self.position.x + 27.5, y: self.position.y + 20.0 }, 30.0, 150.0, self.moving_towards);
                self.last_move_time = Instant::now();
            }
        }
    }

    pub fn get_movement_grid_from_near_objects(&self, end: &Position, window_start_position: Position, window_size: Size, doors_and_walls: &[impl LevelObject<'a>]) -> Vec<Vec<bool>> {
        let (grid_start_position, grid_size) = if self.position.y == end.y {
            let (distance, edge_length_start_position, edge_length_end_position, is_larger) = if self.position.x > end.x {
                let right_sum = sum_direction_length_from_path(self.original_moves_path, Direction::Right, self.movement_value);

                if right_sum < 50.0 {
                    let mut max_size_start_position = (window_start_position.x + window_size.width) - self.position.x;
                    if max_size_start_position >= 50.0 {
                        max_size_start_position = 50.0;
                    }

                    let mut max_size_end_position = end.x - window_start_position.x;
                    if max_size_end_position >= 50.0 {
                        max_size_end_position = 50.0;
                    }

                    (self.position.x - end.x, max_size_start_position, max_size_end_position, true)
                } else {
                    let mut max_size_end_position = end.x - window_start_position.x;
                    if max_size_end_position >= right_sum {
                        max_size_end_position = right_sum;
                    }

                    (self.position.x - end.x, right_sum, max_size_end_position, true)
                }
            } else {
                let left_sum = sum_direction_length_from_path(self.original_moves_path, Direction::Left, self.movement_value);

                if left_sum < 50.0 {
                    let mut max_size_start_position = self.position.x - window_start_position.x;
                    if max_size_start_position >= 50.0 {
                        max_size_start_position = 50.0;
                    }

                    let mut max_size_end_position = (window_start_position.x + window_size.width) - end.x;
                    if max_size_end_position >= 50.0 {
                        max_size_end_position = 50.0;
                    }

                    (end.x - self.position.x, max_size_start_position, max_size_end_position, false)
                } else {
                    let mut max_size_end_position = (window_start_position.x + window_size.width) - end.x;
                    if max_size_end_position >= left_sum {
                        max_size_end_position = left_sum;
                    }

                    (end.x - self.position.x, left_sum, max_size_end_position, false)
                }
            };

            let mut up_length = sum_direction_length_from_path(self.original_moves_path, Direction::Up, self.movement_value);
            if up_length < 50.0 {
                let max_distance = self.position.y - window_start_position.y;

                if max_distance >= 50.0 {
                    up_length = 50.0;
                } else {
                    up_length = max_distance;
                }
            }

            let mut down_length = sum_direction_length_from_path(self.original_moves_path, Direction::Down, self.movement_value);
            if down_length < 50.0 {
                let max_distance = (window_start_position.y + window_size.height) - self.position.y;

                if max_distance >= 50.0 {
                    down_length = 50.0;
                } else {
                    down_length = max_distance;
                }
            }

            let start_position_x; 
            if is_larger {
                start_position_x = self.position.x - edge_length_end_position - distance;
            } else {
                start_position_x = self.position.x - edge_length_start_position;
            }

            let mut start_position_y = self.position.y - up_length;
            if start_position_y < window_start_position.y {
                start_position_y = window_start_position.y;
            }
            
            (Position { x: start_position_x, y: start_position_y }, Size { width: distance + edge_length_start_position + edge_length_end_position, height: up_length + down_length })
        } else if self.position.x == end.x {
            let (distance, edge_length_start_position, edge_length_end_position, is_larger) = if self.position.y > end.y {
                let down_sum = sum_direction_length_from_path(self.original_moves_path, Direction::Down, self.movement_value);

                if down_sum < 50.0 {
                    let mut max_size_start_position = (window_start_position.y + window_size.height) - self.position.y;
                    if max_size_start_position >= 50.0 {
                        max_size_start_position = 50.0;
                    }

                    let mut max_size_end_position = end.y - window_start_position.y;
                    if max_size_end_position >= 50.0 {
                        max_size_end_position = 50.0;
                    }

                    (self.position.y - end.y, max_size_start_position, max_size_end_position, true)
                } else {
                    let mut max_size_end_position = end.y - window_start_position.y;
                    if max_size_end_position >= down_sum {
                        max_size_end_position = down_sum;
                    }

                    (self.position.y - end.y, down_sum, max_size_end_position, true)
                }
            } else {
                let up_sum = sum_direction_length_from_path(self.original_moves_path, Direction::Up, self.movement_value);
                
                if up_sum < 50.0 {
                    let mut max_size_start_position = self.position.y - window_start_position.y;
                    if max_size_start_position >= 50.0 {
                        max_size_start_position = 50.0;
                    }

                    let mut max_size_end_position = (window_start_position.y + window_size.height) - end.y;
                    if max_size_end_position >= 50.0 {
                        max_size_end_position = 50.0;
                    }

                    (end.y - self.position.y, max_size_start_position, max_size_end_position, false)
                } else {
                    let mut max_size_end_position = (window_start_position.y + window_size.height) - end.y;
                    if max_size_end_position >= up_sum {
                        max_size_end_position = up_sum;
                    }

                    (end.y - self.position.y, up_sum, max_size_end_position, false)
                }
            };

            let mut left_length = sum_direction_length_from_path(self.original_moves_path, Direction::Left, self.movement_value);
            if left_length < 50.0 {
                let max_distance = self.position.x - window_start_position.x;

                if max_distance >= 50.0 {
                    left_length = 50.0;
                } else {
                    left_length = max_distance;
                }
            }

            let mut right_length = sum_direction_length_from_path(self.original_moves_path, Direction::Right, self.movement_value);
            if right_length < 50.0 {
                let max_distance = (window_start_position.x + window_size.width) - self.position.x;

                if max_distance >= 50.0 {
                    right_length = 50.0;
                } else {
                    right_length = max_distance;
                }
            }

            let start_position_x = self.position.x - left_length; 

            let start_position_y;
            if is_larger {
                start_position_y = self.position.y - edge_length_end_position - distance;
            } else {
                start_position_y = self.position.y - edge_length_start_position;
            }

            (Position { x: start_position_x, y: start_position_y }, Size { width: left_length + right_length, height: distance + edge_length_end_position + edge_length_start_position })
        } else {
            let (distance_x, edge_length_start_position, edge_length_end_position, is_larger) = if self.position.x > end.x {
                let right_sum = sum_direction_length_from_path(self.original_moves_path, Direction::Right, self.movement_value);

                if right_sum < 50.0 {
                    let mut max_size_start_position = (window_start_position.x + window_size.width) - self.position.x;
                    if max_size_start_position >= 50.0 {
                        max_size_start_position = 50.0;
                    }

                    let mut max_size_end_position = end.x - window_start_position.x;
                    if max_size_end_position >= 50.0 {
                        max_size_end_position = 50.0;
                    }

                    (self.position.x - end.x, max_size_start_position, max_size_end_position, true)
                } else {
                    let mut max_size_end_position = end.x - window_start_position.x;
                    if max_size_end_position >= right_sum {
                        max_size_end_position = right_sum;
                    }

                    (self.position.x - end.x, right_sum, max_size_end_position, true)
                }
            } else {
                let left_sum = sum_direction_length_from_path(self.original_moves_path, Direction::Left, self.movement_value);

                if left_sum < 50.0 {
                    let mut max_size_start_position = self.position.x - window_start_position.x;
                    if max_size_start_position >= 50.0 {
                        max_size_start_position = 50.0;
                    }

                    let mut max_size_end_position = (window_start_position.x + window_size.width) - end.x;
                    if max_size_end_position >= 50.0 {
                        max_size_end_position = 50.0;
                    }

                    (end.x - self.position.x, max_size_start_position, max_size_end_position, false)
                } else {
                    let mut max_size_end_position = (window_start_position.x + window_size.width) - end.x;
                    if max_size_end_position >= left_sum {
                        max_size_end_position = left_sum;
                    }

                    (end.x - self.position.x, left_sum, max_size_end_position, false)
                }
            };

            let distance_y = absolute_f32(self.position.y - end.y);

            let mut up_length = sum_direction_length_from_path(self.original_moves_path, Direction::Up, self.movement_value);
            if up_length < 50.0 {
                let max_distance = self.position.y - window_start_position.y;

                if max_distance >= 50.0 {
                    up_length = 50.0;
                } else {
                    up_length = max_distance;
                }
            }

            let mut down_length = sum_direction_length_from_path(self.original_moves_path, Direction::Down, self.movement_value);
            if down_length < 50.0 {
                let max_distance = (window_start_position.y + window_size.height) - self.position.y;

                if max_distance >= 50.0 {
                    down_length = 50.0;
                } else {
                    down_length = max_distance;
                }
            }

            let start_position_x; 
            if is_larger {
                start_position_x = self.position.x - edge_length_end_position - distance_x;
            } else {
                start_position_x = self.position.x - edge_length_start_position;
            }

            let mut start_position_y = self.position.y - up_length - distance_y;
            if start_position_y < window_start_position.y {
                start_position_y = window_start_position.y;
            }

            let mut up_distance = self.position.y - (up_length + distance_y);
            if up_distance < window_start_position.y {
                up_distance = self.position.y - window_start_position.y;
            } else {
                up_distance = up_length + distance_y;
            }

            let mut down_distance = self.position.y + (down_length + distance_y);
            if (window_start_position.y + window_size.height) < down_distance {
                down_distance = (window_start_position.y + window_size.height) - self.position.y;
            } else {
                down_distance = down_length + distance_y;
            }

            (Position { x: start_position_x, y: start_position_y }, Size { width: distance_x + edge_length_end_position + edge_length_start_position, height: up_distance + down_distance })
        };

        let grid_rows = (grid_size.height / self.movement_value) as usize;
        let grid_cols = (grid_size.width / self.movement_value) as usize;

        let mut grid = Vec::with_capacity(grid_rows);

        for row in 0..grid_rows {
            let mut col_vec = Vec::with_capacity(grid_cols);

            for col in 0..grid_cols {
                let current_position = Position { x: grid_start_position.x + (col as f32 * self.movement_value), y: grid_start_position.y + (row as f32 * self.movement_value) };

                let mut is_walkable = true;

                for object in doors_and_walls {
                    let object_poistion = object.get_position();
                    
                    if object_poistion >= grid_start_position && object_poistion <= (grid_start_position + grid_size) {
                        let object_size = object.get_size();
                        let object_max_position = object_poistion + object_size;

                        match object.get_type() {
                            ObjectType::Wall => {
                                if (current_position.x >= object_poistion.x && current_position.y >= object_poistion.y) && (current_position.x < object_max_position.x && current_position.y < object_max_position.y) {
                                    is_walkable = false;
        
                                    break;
                                } else if current_position.x < object_poistion.x {
                                    if object_poistion.x - current_position.x < self.size.width {
                                        is_walkable = false;

                                        break;
                                    }
                                } else if current_position.x >= object_poistion.x && current_position.x < object_max_position.x {
                                    if current_position.y < object_poistion.y {
                                        if object_poistion.y - current_position.y < self.size.height {
                                            is_walkable = false;

                                            break;
                                        }
                                    }
                                }
                            },
    
                            ObjectType::Door(_) => {
                                if current_position == object_poistion {
                                    is_walkable = true;
    
                                    break;
                                } else if (current_position.x >= object_poistion.x && current_position.y >= object_poistion.y) && (current_position.x <= object_max_position.x && current_position.y <= object_max_position.y) {
                                    is_walkable = false;

                                    break;
                                }
                            },
    
                            _ => {
                                continue;
                            }
                        }
                    }
                }

                col_vec.push(is_walkable);
            }

            grid.push(col_vec);
        }

        grid
    }

    pub fn find_optimal_path(&self, target_position: Position, grid: Vec<Vec<bool>>) -> Option<PathVec> {
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
                let mut last_direction = None;
                let mut count = 0;

                current = self.position;

                for i in 1..path.len() {
                    let current_position = path[i];

                    let direction = if current.y == current_position.y {
                        if current.x > current_position.x {
                            Direction::Left
                        } else {
                            Direction::Right
                        }
                    } else {
                        if current.y > current_position.y {
                            Direction::Up
                        } else {
                            Direction::Down
                        }
                    };

                    if i != 1 && last_direction != Some(direction) {
                        moves.push((count, last_direction.unwrap(), 0));
                        count = 0;
                    }

                    current = current_position;
                    last_direction = Some(direction);
                    count += 1;

                    if i == path.len() - 1 {
                        moves.push((count, last_direction.unwrap(), 0));
                    }
                }

                return Some(moves);
            }

            for neighbor in current_position.get_neighbors(self.movement_value) {
                // mabye remove position check

                let neighbor_col = neighbor.x / self.movement_value;
                let neighbor_row = neighbor.y / self.movement_value;

                if neighbor_col >= 0.0 && neighbor_col < grid[0].len() as f32 &&
                   neighbor_row >= 0.0 && neighbor_row < grid.len() as f32 &&
                   grid[neighbor_row as usize][neighbor_col as usize] {
                    let tentative_score = position_score[&current_position] + self.movement_value as i32; 

                    if tentative_score < *position_score.get(&neighbor).unwrap_or(&i32::MAX) {
                        came_from.insert(neighbor, Some(current_position));
                        position_score.insert(neighbor, tentative_score);
                        movements.push(PossibilityNode {
                            position: neighbor,
                            priority_score: tentative_score + get_heuristic_score(&neighbor, &target_position, self.movement_value) as i32
                        });
                    }
                } 
            }
        }

        None
    }

    pub fn move_enemy(&mut self, player: &mut Player<'a>, current_notoriety_level: u64) {
        match self.mode {
            EnemyMode::Regular => {
                self.already_detected_player = false;

                if self.prev_mode != EnemyMode::Regular {
                    if self.move_interval != DEFAULT_MOVE_INTERVAL {
                        self.move_interval = DEFAULT_MOVE_INTERVAL;
                    }

                    if self.position != self.start_position {
                        self.current_moves_path = self.find_optimal_path(self.start_position, vec![vec![]]).unwrap_or(Vec::new());
                    } else {
                        self.moves_count = 0;

                        self.current_moves_path = convert_path(self.original_moves_path);

                        self.prev_mode = EnemyMode::Regular;
                    }
                }

                self.move_enemy_in_path(None);
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
    
                        self.current_moves_path = self.find_optimal_path(detect_player_position, vec![vec![]]).unwrap_or(Vec::new());
    
                        self.move_enemy_in_path(Some(Duration::from_millis(300 - (current_notoriety_level * 50))));
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
    
                        self.current_moves_path = self.get_searching_path();
                    } else if self.position == (Position { x: start_searching_position.x + ((50.0 / self.movement_value) * self.movement_value), y: start_searching_position.y }) && !self.is_detecting_player(player) {
                        self.mode = EnemyMode::Regular;
                    } else {
                        self.move_enemy_in_path(None);
                    }
                } else {
                    self.mode = EnemyMode::Regular;
                }

                self.already_detected_player = true;
                self.prev_mode = EnemyMode::Searching;
            }
        }
    }

    fn get_searching_path(&self) -> PathVec {
        let steps = 50 / self.movement_value as u32;

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

        // TODO: test player.collide(enemy)
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

            // TODO: test player.collide(enemy)
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

        self.collide(player)
    }

    pub fn get_mode(&mut self) -> &EnemyMode {
        &self.mode
    }

    pub fn set_movement_value(&mut self, new_value: f32) {
        self.movement_value = new_value;
    }
}
