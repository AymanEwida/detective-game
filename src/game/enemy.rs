use core::fmt;
use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, time::{Duration, Instant}};

use crate::{library::utils::{calc_equidistant_points, convert_path, get_estimated_position, get_heuristic_score, round_position_to_full_numbers, PathVec}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::{Character, Direction}, door::Door, hide_place::HidePlace, level::GameObject, player::{Player, PlayerStatus}, wall::Wall};

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

impl fmt::Display for EnemyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular => {
                write!(f, "Regular")
            },

            Self::Detecting => {
                write!(f, "Detecting")
            },

            Self::Searching => {
                write!(f, "Searching")
            }
        }
    }
}

#[derive(Debug)]
pub struct Enemy<'a> {
    start_position: Position,
    position: Position,
    prev_position: Option<Position>,
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
    default_search_path: Option<PathVec>,
    near_hide_places_positions: Option<Vec<Position>>,
    current_search_idx: usize,
    estimated_search_position: Option<Position>,
    is_colliding: bool,
    want_to_teleport: bool,
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
                    prev_position: None,
                    size: Size { width: 50.0, height: 60.0 },
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
                    default_search_path: None,
                    near_hide_places_positions: None,
                    current_search_idx: 0,
                    estimated_search_position: None,
                    is_colliding: false,
                    want_to_teleport: false,
                }
            }
        }
    }
}

impl<'a> GameObject<'a> for Enemy<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, self.flip, None, None, None, None)?;

        let (first_point, second_point, apex) = self.detect_traingle;

        render.draw_line(apex, first_point, Color::Red, None, None, None);
        render.draw_line(apex, second_point, Color::Red, None, None, None);
        render.draw_line(first_point, second_point, Color::Red, None, None, None); 

        // TODO: add logic to draw enemy original path with draw_line func from render

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

    pub fn _set_prev_position(&mut self) {
        self.prev_position = Some(self.position)
    }

    pub fn set_is_colliding(&mut self, new_value: bool) {
        self.is_colliding = new_value;
    }
    
    pub fn get_want_to_teleport(&self) -> bool {
        self.want_to_teleport
    }

    pub fn set_want_to_teleport(&mut self, new_value: bool) {
        self.want_to_teleport = new_value;
    }

    fn move_enemy_in_path(&mut self, move_interval: Option<Duration>) {
        if self.last_move_time.elapsed() >= self.move_interval {
            if let Some((moves_number, direction, wait_time)) = self.current_moves_path.first() {
                if *moves_number > 0 {
                    self.moving_towards = *direction;

                    self.prev_position = Some(self.position);
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

    pub fn move_to_prev_position(&mut self) {
        if let Some(prev_position) = self.prev_position {
            self.position = prev_position;
        }
    }

    pub fn get_movement_grid(&self, window_start_position: Position, window_size: Size, walls: &[Wall<'a>]) -> (Position, Vec<Vec<bool>>) {
        let grid_rows = (window_size.height / self.movement_value) as usize;
        let grid_cols = (window_size.width / self.movement_value) as usize;

        let mut grid = Vec::with_capacity(grid_rows);

        for row in 0..grid_rows {
            let mut col_vec = Vec::with_capacity(grid_cols);

            for col in 0..grid_cols {
                let current_position = Position { x: window_start_position.x + (col as f32 * self.movement_value), y: window_start_position.y + (row as f32 * self.movement_value) };

                let mut is_walkable = true;

                for wall in walls {
                    let wall_poistion = round_position_to_full_numbers(wall.get_position(), self.movement_value, true, true);
                    
                    if wall_poistion >= window_start_position && wall_poistion <= (window_start_position + window_size) {
                        let wall_size = wall.get_size();
                        let wall_max_position = round_position_to_full_numbers(wall_poistion + wall_size, self.movement_value, true, true);

                        if (current_position.x >= wall_poistion.x && current_position.y >= wall_poistion.y) && (current_position.x < wall_max_position.x && current_position.y < wall_max_position.y) {
                            is_walkable = false;

                            break;
                        } else if (current_position.y >= wall_poistion.y && current_position.y < wall_max_position.y) && current_position.x < wall_poistion.x {
                            if wall_poistion.x - current_position.x < self.size.width {
                                is_walkable = false;

                                break;
                            }
                        } else if (current_position.x >= wall_poistion.x && current_position.x < wall_max_position.x) && current_position.y < wall_poistion.y {
                            if wall_poistion.y - current_position.y < self.size.height {
                                is_walkable = false;

                                break;
                            }
                        } else if current_position.x < wall_poistion.x && current_position.y < wall_poistion.y {
                            if wall_poistion.x - current_position.x < self.size.width && wall_poistion.y - current_position.y < self.size.height {
                                is_walkable = false;

                                break;
                            }
                        }
                    }
                }

                col_vec.push(is_walkable);
            }

            grid.push(col_vec);
        }

        (window_start_position, grid)
    }

    pub fn find_optimal_path(&self, target_position: Position, grid_start_position: Position, grid: Vec<Vec<bool>>) -> Option<PathVec> {
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
                let neighbor_col = (neighbor.x - grid_start_position.x) / self.movement_value;
                let neighbor_row = (neighbor.y - grid_start_position.y) / self.movement_value;

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

    pub fn move_enemy(&mut self, player: &mut Player<'a>, current_notoriety_level: u64, window_start_position: Position, window_size: Size, walls: &[Wall<'a>], doors: &[Door<'a>], hide_places: &[HidePlace<'a>]) -> u64 {
        let new_notority_level = self.detect_player(current_notoriety_level, player, walls, doors);

        match self.mode {
            EnemyMode::Regular => {
                self.already_detected_player = false;

                if self.prev_mode != EnemyMode::Regular {
                    if self.move_interval != DEFAULT_MOVE_INTERVAL {
                        self.move_interval = DEFAULT_MOVE_INTERVAL;
                    }

                    if self.position != self.start_position {
                        let (grid_start_position, grid) = self.get_movement_grid(window_start_position, window_size, walls);
                        self.current_moves_path = self.find_optimal_path(
                            round_position_to_full_numbers(self.start_position, self.movement_value, true, true),
                            grid_start_position,
                            grid
                        ).unwrap_or(Vec::new());
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
                    if self.position == detect_player_position && !self.is_detecting_player(player, walls, doors) {
                        self.start_searching_position = Some(self.position);
                        self.detect_player_position = None;
                    } else {
                        if self.prev_mode != EnemyMode::Detecting && self.move_interval != DEFAULT_MOVE_INTERVAL {
                            self.move_interval = Duration::from_millis(1500);
                        }
    
                        let (grid_start_position, grid) = self.get_movement_grid(window_start_position, window_size, walls);
                        self.current_moves_path = self.find_optimal_path(
                            round_position_to_full_numbers(detect_player_position, self.movement_value, true, true),
                            grid_start_position,
                            grid
                        ).unwrap_or(Vec::new());
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
                    }

                    if self.near_hide_places_positions.is_none() {
                        self.near_hide_places_positions = Some(self.get_near_hide_places_positions(Some(start_searching_position), hide_places));
                    }

                    if let Some(near_hide_places_positions) = &self.near_hide_places_positions {
                        if near_hide_places_positions.len() > 0 && self.current_search_idx < near_hide_places_positions.len() {
                            let current_hide_place_position = round_position_to_full_numbers(near_hide_places_positions[self.current_search_idx], self.movement_value, false, true);
        
                            if self.position != current_hide_place_position {
                                let (grid_start_position, grid) = self.get_movement_grid(window_start_position, window_size, walls);
                                self.current_moves_path = self.find_optimal_path(current_hide_place_position, grid_start_position, grid).unwrap_or(Vec::new());
                                self.move_enemy_in_path(None); 
                            } else {
                                if self.collide(player) {
                                    player.throw_form_hide_place(walls, &self.moving_towards);
                                    player.set_status(PlayerStatus::Detectit);

                                    self.already_detected_player = true;
                                    self.mode = EnemyMode::Detecting;
                                    self.detect_player_position = Some(player.get_position());
                                }

                                if self.last_move_time.elapsed() >= Duration::from_millis(2000) {
                                    self.current_search_idx += 1;
                                }
                            }
                        } else if near_hide_places_positions.len() == 0 {
                            if self.default_search_path.is_none() {
                                self.default_search_path = Some(self.get_default_search_path());
                            }

                            if let Some(default_search_path) = &self.default_search_path {
                                if default_search_path.len() > 0 && self.current_search_idx < default_search_path.len() {
                                    let (steps, direction, wait_interval) = default_search_path[self.current_search_idx];

                                    if self.estimated_search_position.is_none() {
                                        self.estimated_search_position = Some(get_estimated_position(&self.position, steps, direction, self.movement_value));
                                    }

                                    assert!(self.estimated_search_position != None, "estimated search position must exist");

                                    let estimated_search_position = self.estimated_search_position.unwrap();

                                    if !self.is_colliding && self.position != estimated_search_position {
                                        self.current_moves_path = vec![(steps, direction, wait_interval)];
                                        self.move_enemy_in_path(None);
                                    } else {
                                        self.current_search_idx += 1;
                                        self.estimated_search_position = None;
                                    }
                                } else {
                                    self.current_search_idx = 0;

                                    self.mode = EnemyMode::Regular;
                                }
                            }
                        } else {
                            self.current_search_idx = 0;

                            self.mode = EnemyMode::Regular;
                        }
                    }
                } else {
                    self.current_search_idx = 0;

                    self.mode = EnemyMode::Regular;
                }

                self.already_detected_player = true;
                self.prev_mode = EnemyMode::Searching;
            }
        }

        new_notority_level
    }

    pub fn get_near_hide_places_positions(&self, start_position: Option<Position>, hide_places: &[HidePlace<'a>]) -> Vec<Position> {
        let start_position = start_position.unwrap_or(self.position);

        let mut near_hide_places_positions = Vec::new();

        for hide_place in hide_places {
            let hide_place_position = hide_place.get_position();

            let heuristic_distance = get_heuristic_score(&start_position, &hide_place_position, self.movement_value);

            if near_hide_places_positions.len() < 3 {
                let mut found_idx: i32 = -1;

                for (idx, near_hide_place_position) in near_hide_places_positions.iter().enumerate() {
                    if get_heuristic_score(&start_position, near_hide_place_position, self.movement_value) > heuristic_distance {
                        found_idx = idx as i32;

                        break;
                    }
                }

                if found_idx == -1 {
                    near_hide_places_positions.push(hide_place_position); 
                } else {
                    near_hide_places_positions.insert(found_idx as usize, hide_place_position);
                }
            } else {
                let mut found_idx: i32 = -1;

                for (idx, near_hide_place_position) in near_hide_places_positions.iter().enumerate() {
                    if get_heuristic_score(&start_position, near_hide_place_position, self.movement_value) > heuristic_distance {
                        found_idx = idx as i32;

                        break;
                    }
                }

                if found_idx >= 0 {
                    near_hide_places_positions.remove(near_hide_places_positions.len() - 1);
                    near_hide_places_positions.insert(found_idx as usize, hide_place_position);
                }
            }
        }

        near_hide_places_positions
    }

    fn get_default_search_path(&self) -> PathVec {
        let steps = 60 / self.movement_value as u32;

        vec![(steps, Direction::Left, 0), (steps, Direction::Right, 0), (steps, Direction::Up, 0), (steps * 2, Direction::Down, 0), (steps, Direction::Up, 0), (steps, Direction::Right, 0)]
    } 

    pub fn is_detecting_player(&self, player: &Player<'a>, walls: &[Wall<'a>], doors: &[Door<'a>]) -> bool {
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

        let mut is_able_to_see = true;

        for wall in walls {
            let wall_start = wall.get_position();
            let wall_end = wall_start + wall.get_size();

            let is_between_x_axis = ((self.position.x < wall_start.x && player_start.x >= wall_end.x) || (player_start.x < wall_start.x && self.position.x >= wall_end.x))
                && ((player_start.y >= wall_start.y && enemy_end.y <= wall_end.y) || (self.position.y >= wall_start.y && player_end.y <= wall_end.y));
            
            let is_between_y_axis = ((self.position.y < wall_start.y && player_start.y >= wall_end.y) || (player_start.y < wall_start.y && self.position.y >= wall_end.y))
                && ((player_start.x >= wall_start.x && enemy_end.x <= wall_end.x) || (self.position.x >= wall_start.x && player_end.y <= wall_end.x));
            
            if is_between_x_axis || is_between_y_axis {
                is_able_to_see = false;

                break;
            }
        }

        for door in doors {
            let door_start = door.get_position();
            let door_end = door_start + door.get_size();

            let is_between_x_axis = ((self.position.x < door_start.x && player_start.x >= door_end.x) || (player_start.x < door_start.x && self.position.x >= door_end.x))
                && ((player_start.y >= door_start.y && enemy_end.y <= door_end.y) || (self.position.y >= door_start.y && player_end.y <= door_end.y));

            let is_between_y_axis = ((self.position.y < door_start.y && player_start.y >= door_end.y) || (player_start.y < door_start.y && self.position.y >= door_end.y))
                && ((player_start.x >= door_start.x && enemy_end.x <= door_end.x) || (self.position.x >= door_start.x && player_end.y <= door_end.x));
            
            if (is_between_x_axis || is_between_y_axis) && door.is_closed() {
                is_able_to_see = false;

                break;
            }
        }

        if !is_able_to_see {
            return false;
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

    pub fn detect_player(&mut self, current_notoriety_level: u64, player: &mut Player<'a>, walls: &[Wall<'a>], doors: &[Door<'a>]) -> u64 {
        let is_detected = self.is_detecting_player(player, walls, doors);

        if is_detected {
            let player_start = player.get_position();

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

    pub fn get_movement_value(&self) -> f32 {
        self.movement_value
    }

    pub fn set_movement_value(&mut self, new_value: f32) {
        self.movement_value = new_value;
    }

    pub fn is_off_window(&self, window_size: Size) -> bool {
        self.position.x > window_size.width ||
        (self.position.x + self.size.width) > window_size.width ||
        self.position.x < 0.0 ||
        self.position.y > window_size.height ||
        (self.position.y + self.size.height) > window_size.height ||
        self.position.y < 0.0
    }

    pub fn is_off_border(&self, start_position: Option<Position>, size: Size) -> bool {
        let start_position = start_position.unwrap_or(Position { x: 0.0, y: 0.0 });

        self.position.x > (start_position.x + size.width) ||
        (self.position.x + self.size.width) > (start_position.x + size.width) ||
        self.position.x < start_position.x ||
        self.position.y > (start_position.y + size.height) ||
        (self.position.y + self.size.height) > (start_position.y + size.height) ||
        self.position.y < start_position.y
    }
}
