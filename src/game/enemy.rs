use core::fmt;
use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}, time::{Duration, Instant}};

use queues::{IsQueue, Queue};

use crate::{library::{constants::DEFAULT_MOVEMENT_VALUE, utils::{bfs_object_detect_check, calc_equidistant_points, calculate_calc_position, convert_path, get_estimated_position, get_heuristic_score, is_position_in_border, round_position_to_full_numbers, simple_object_detect_check, PathVec}}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::{GridPosition, Position}}};

use super::{character::{Character, Direction, DEFAULT_CHARACTER_SIZE}, door::Door, hide_place::HidePlace, level::{EndStartPositions, GameObject}, player::{Player, PlayerStatus}, wall::Wall};

pub type DetectTraingle = (Position, Position, Position);

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

#[allow(non_upper_case_globals)]
static mut enemies_count: usize = 0;

#[derive(Debug)]
pub struct Enemy<'a> {
    id: usize,
    start_position: Position,
    position: Position,
    prev_position: Option<Position>,
    calc_start_position: EndStartPositions,
    calc_position: EndStartPositions,
    size: Size,
    image: &'a str,
    flip: bool,
    movement_value: f32,
    last_move_time: Instant,
    move_interval: Duration,
    current_moves_path: PathVec,
    original_moves_path: &'a str,
    original_moves_path_vec: PathVec,
    moves_count: u32,
    moving_towards: Direction,
    detect_traingle: DetectTraingle,
    detect_player_position: Option<Position>,
    mode: EnemyMode,
    prev_mode: EnemyMode,
    already_detected_player: bool,
    start_searching_position: Option<Position>,
    default_search_path: Option<PathVec>,
    movement_grid: Option<(Position, Vec<Vec<bool>>)>,
    near_hide_places_positions: Option<Vec<Position>>,
    current_search_idx: usize,
    estimated_search_position: Option<Position>,
    is_colliding: bool,
    collide_info: (u32, Option<Direction>, f32),
    want_to_teleport: bool,
    draw_detect_traingle: bool,
    draw_move_path: bool,
}

impl<'a> Enemy<'a> {
    pub fn new(enemy_type: EnemyType, start_position: Position, path: &'a str, flip: bool) -> Self {
        unsafe {
            enemies_count += 1;
        }

        let moves_path = convert_path(path);
        let first_direction = moves_path[0].1;

        let size = DEFAULT_CHARACTER_SIZE;

        let calc_start_position = calculate_calc_position(start_position, size, DEFAULT_MOVEMENT_VALUE);

        match enemy_type {
            EnemyType::Regular => {
                Self {
                    id: unsafe { enemies_count },
                    start_position,
                    position: start_position,
                    prev_position: None,
                    calc_start_position,
                    calc_position: calc_start_position,
                    size,
                    image: "assets/game/regular-enemy.png",
                    flip,
                    movement_value: DEFAULT_MOVEMENT_VALUE,
                    last_move_time: Instant::now(),
                    move_interval: DEFAULT_MOVE_INTERVAL,
                    original_moves_path: path,
                    original_moves_path_vec: moves_path.clone(),
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
                    movement_grid: None,
                    near_hide_places_positions: None,
                    current_search_idx: 0,
                    estimated_search_position: None,
                    is_colliding: false,
                    collide_info: (0, None, DEFAULT_MOVEMENT_VALUE),
                    want_to_teleport: false,
                    draw_detect_traingle: true, // return to false after
                    draw_move_path: false
                }
            }
        }
    }
}

impl<'a> GameObject<'a> for Enemy<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, self.flip, None, None, None, None)?;

        if self.draw_detect_traingle {
            let (first_point, second_point, apex) = self.detect_traingle;

            render.draw_line(apex, first_point, Color::Red, None, None, None);
            render.draw_line(apex, second_point, Color::Red, None, None, None);
            render.draw_line(first_point, second_point, Color::Red, None, None, None); 
        }

        if self.draw_move_path {
            let half_size = self.size / 2.0;

            let mut current_position = self.start_position + half_size;

            for (steps, direction, ..) in self.original_moves_path_vec.iter() {
                let steps = steps * 10;
                
                let end_position;

                match direction {
                    Direction::Up => {
                        end_position = Position {
                            y: current_position.y - (steps as f32),
                            ..current_position
                        };
                    },

                    Direction::Down => {
                        end_position = Position {
                            y: current_position.y + (steps as f32),
                            ..current_position
                        };
                    },

                    Direction::Left => {
                        end_position = Position {
                            x: current_position.x - (steps as f32),
                            ..current_position
                        };
                    },

                    Direction::Right => {
                        end_position = Position {
                            x: current_position.x + (steps as f32),
                            ..current_position
                        };
                    },
                }

                render.draw_line(current_position, end_position, Color::White, None, None, None);

                current_position = end_position;
            }
        }

        Ok(())
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
        self.set_calc_position();
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_calc_position(&self) -> EndStartPositions {
        self.calc_position
    }
}

impl<'a> Character<'a> for Enemy<'a> {
    fn set_flip(&mut self, new_value: bool) {
        self.flip = new_value;
    }
}

impl<'a> Enemy<'a> {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_start_position(&self) -> Position {
        self.start_position
    }

    pub fn _set_start_position(&mut self, _new_start_position: Position) {
        self.start_position = _new_start_position;
        self.calc_start_position = calculate_calc_position(self.start_position, self.size, self.movement_value);
    }

    pub fn _set_prev_position(&mut self) {
        self.prev_position = Some(self.position);
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

    pub fn get_draw_detect_traingle(&self) -> bool {
        self.draw_detect_traingle
    }

    pub fn set_draw_detect_traingle(&mut self, new_val: bool) {
        self.draw_detect_traingle = new_val;
    }

    pub fn get_draw_move_path(&self) -> bool {
        self.draw_move_path
    }

    pub fn set_draw_move_path(&mut self, new_val: bool) {
        self.draw_move_path = new_val;
    }

    pub fn get_calc_start_position(&self) -> EndStartPositions {
        self.calc_start_position
    }
    
    fn set_calc_position(&mut self) {
        self.calc_position = calculate_calc_position(self.position, self.size, self.movement_value);
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
            self.set_calc_position();
        }
    }

    pub fn get_movement_grid(&self, window_start_position: Position, window_size: Size, walls: &[Wall<'a>]) -> (Position, Vec<Vec<bool>>) {
        let window_end_position = window_start_position + window_size;

        let grid_rows = (window_size.height / self.movement_value) as usize;
        let grid_cols = (window_size.width / self.movement_value) as usize;

        let mut grid = Vec::with_capacity(grid_rows);

        for row in 0..grid_rows {
            let mut col_vec = Vec::with_capacity(grid_cols);

            for col in 0..grid_cols {
                let current_position = Position { x: window_start_position.x + (col as f32 * self.movement_value), y: window_start_position.y + (row as f32 * self.movement_value) };

                let mut is_walkable = true;

                for wall in walls {
                    let (wall_start_position, wall_end_position) = wall.get_calc_position();
                    
                    let check_wall_positions = is_position_in_border(&window_start_position, &window_end_position, &wall_start_position);

                    if check_wall_positions.0 && check_wall_positions.1 {
                        if (current_position.x >= wall_start_position.x && current_position.y >= wall_start_position.y) && (current_position.x < wall_end_position.x && current_position.y < wall_end_position.y) {
                            is_walkable = false;

                            break;
                        } else if (current_position.y >= wall_start_position.y && current_position.y < wall_end_position.y) && current_position.x < wall_start_position.x {
                            if wall_start_position.x - current_position.x < self.size.width {
                                is_walkable = false;

                                break;
                            }
                        } else if (current_position.x >= wall_start_position.x && current_position.x < wall_end_position.x) && current_position.y < wall_start_position.y {
                            if wall_start_position.y - current_position.y < self.size.height {
                                is_walkable = false;

                                break;
                            }
                        } else if current_position.x < wall_start_position.x && current_position.y < wall_start_position.y {
                            if wall_start_position.x - current_position.x < self.size.width && wall_start_position.y - current_position.y < self.size.height {
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

    pub fn find_optimal_path(&self, target_position: Position, grid_start_position: Position, grid: &Vec<Vec<bool>>) -> Option<PathVec> {
        let (enemy_start, ..) = self.get_calc_position();

        if enemy_start == target_position {
            return None;
        }

        let mut movements = BinaryHeap::new();
        let mut came_from: HashMap<Position, Option<Position>> = HashMap::new();
        let mut position_score = HashMap::new();

        movements.push(PossibilityNode {
            position: enemy_start,
            priority_score: 0
        });
        came_from.insert(enemy_start, None);
        position_score.insert(enemy_start, 0);

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

                current = enemy_start;

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

    fn move_enemy_when_colliding(&mut self) {
        if self.is_colliding {
            self.collide_info.0 += 1;

            if self.collide_info.0 >= 3 {
                match self.moving_towards {
                    Direction::Up => {
                        if self.collide_info.1.is_none() {
                            self.move_character(Direction::Left, self.collide_info.2);

                            self.collide_info.1 = Some(Direction::Right);
                        } else {
                            let mut dir = self.collide_info.1.unwrap();

                            if dir != Direction::Left && dir != Direction::Right {
                                dir = Direction::Left;
                            }

                            self.move_character(dir, self.collide_info.2);

                            if dir == Direction::Left && self.collide_info.2 == 20.0 {
                                self.collide_info.2 = DEFAULT_MOVEMENT_VALUE;

                                self.collide_info.1 = Some(Direction::Right);
                            } else {
                                self.collide_info.1 = Some(Direction::Left);
                            }
                        }
                    },

                    Direction::Down => {
                        if self.collide_info.1.is_none() {
                            self.move_character(Direction::Right, self.collide_info.2);

                            self.collide_info.1 = Some(Direction::Left);
                        } else {
                            let mut dir = self.collide_info.1.unwrap();

                            if dir != Direction::Left && dir != Direction::Right {
                                dir = Direction::Right;
                            }

                            self.move_character(dir, self.collide_info.2);

                            if dir == Direction::Right && self.collide_info.2 == 20.0 {
                                self.collide_info.2 = DEFAULT_MOVEMENT_VALUE;

                                self.collide_info.1 = Some(Direction::Left);
                            } else {
                                self.collide_info.1 = Some(Direction::Right);
                            }
                        }
                    },

                    Direction::Left => {
                        if self.collide_info.1.is_none() {
                            self.move_character(Direction::Up, self.collide_info.2);

                            self.collide_info.1 = Some(Direction::Down);
                        } else {
                            let mut dir = self.collide_info.1.unwrap();

                            if dir != Direction::Up && dir != Direction::Down {
                                dir = Direction::Up;
                            }

                            self.move_character(dir, self.collide_info.2);

                            if dir == Direction::Up && self.collide_info.2 == 20.0 {
                                self.collide_info.2 = DEFAULT_MOVEMENT_VALUE;

                                self.collide_info.1 = Some(Direction::Down);
                            } else {
                                self.collide_info.1 = Some(Direction::Up);
                            }
                        }
                    },

                    Direction::Right => {
                        if self.collide_info.1.is_none() {
                            self.move_character(Direction::Down, self.collide_info.2);

                            self.collide_info.1 = Some(Direction::Up);
                        } else {
                            let mut dir = self.collide_info.1.unwrap();

                            if dir != Direction::Up && dir != Direction::Down {
                                dir = Direction::Down;
                            }

                            self.move_character(dir, self.collide_info.2);

                            if dir == Direction::Down && self.collide_info.2 == 20.0 {
                                self.collide_info.2 = DEFAULT_MOVEMENT_VALUE;

                                self.collide_info.1 = Some(Direction::Up);
                            } else {
                                self.collide_info.1 = Some(Direction::Down);
                            }
                        }
                    }
                }

                self.collide_info.2 += self.movement_value;

                if (self.collide_info.2 / DEFAULT_MOVEMENT_VALUE) % 2.0 == 0.0 {
                    self.collide_info.2 = 20.0;
                } else {
                    self.collide_info.2 = DEFAULT_MOVEMENT_VALUE;
                }

                self.collide_info.0 = 0;
                self.is_colliding = false;
                
                self.calc_position = calculate_calc_position(self.position, self.size, self.collide_info.2);
            }
        } 
    }

    pub fn get_grid(&self) -> &Option<(Position, Vec<Vec<bool>>)> {
        &self.movement_grid
    }

    pub fn move_enemy(&mut self, player: &mut Player<'a>, current_notoriety_level: u64, window_start_position: Position, window_size: Size, walls: &[Wall<'a>], doors: &[Door<'a>], hide_places: &[HidePlace<'a>]) -> u64 {
        self.move_enemy_when_colliding();

        let new_notority_level = self.detect_player(current_notoriety_level, player, walls, doors);

        let grid_start_position: Position;
        let grid: &Vec<Vec<bool>>;

        if self.movement_grid.is_none() {
            self.movement_grid = Some(self.get_movement_grid(window_start_position, window_size, walls));

            grid_start_position = self.movement_grid.as_ref().unwrap().0;
            grid = &self.movement_grid.as_ref().unwrap().1;
        } else {
            grid_start_position = self.movement_grid.as_ref().unwrap().0;
            grid = &self.movement_grid.as_ref().unwrap().1;
        }

        let start_position = self.get_calc_start_position().0;
        let (enemy_start, ..) = self.get_calc_position();

        match self.mode {
            EnemyMode::Regular => {
                self.already_detected_player = false;

                if self.prev_mode != EnemyMode::Regular {
                    if self.move_interval != DEFAULT_MOVE_INTERVAL {
                        self.move_interval = DEFAULT_MOVE_INTERVAL;
                    }

                    if enemy_start != start_position {
                        self.current_moves_path = self.find_optimal_path(
                            start_position,
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
                    if enemy_start == detect_player_position && !self.is_detecting_player(player, walls, doors) {
                        self.start_searching_position = Some(enemy_start);
                        self.detect_player_position = None;
                    } else {
                        if self.prev_mode != EnemyMode::Detecting && self.move_interval != DEFAULT_MOVE_INTERVAL {
                            self.move_interval = Duration::from_millis(1500);
                        }
    
                        self.current_moves_path = self.find_optimal_path(
                            detect_player_position,
                            grid_start_position,
                            grid
                        ).unwrap_or(Vec::new());

                        self.move_enemy_in_path(Some(Duration::from_millis(300 - (current_notoriety_level * 50))));
                    }
                } else {
                    if player.get_status() == &PlayerStatus::Detectit {
                        player.set_status(PlayerStatus::NotHidden);
                        player.set_is_detected_by_enemy(false);
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
        
                            let reach_hide_place = || {
                                (enemy_start == current_hide_place_position)
                                || (
                                    (
                                        enemy_start.y == current_hide_place_position.y && (enemy_start.x + self.movement_value) == current_hide_place_position.x
                                    ) || (
                                        enemy_start.x == current_hide_place_position.x && (enemy_start.y + self.movement_value) == current_hide_place_position.y
                                    )
                                )
                            };

                            if !reach_hide_place() {
                                self.current_moves_path = self.find_optimal_path(current_hide_place_position, grid_start_position, grid).unwrap_or(Vec::new());
                                self.move_enemy_in_path(None); 
                            } else {
                                if self.collide(player) {
                                    player.throw_form_hide_place(walls, &self.moving_towards);
                                    player.set_status(PlayerStatus::Detectit);
                                    player.set_is_detected_by_enemy(true);

                                    self.already_detected_player = true;
                                    self.mode = EnemyMode::Detecting;
                                    self.detect_player_position = Some(
                                        round_position_to_full_numbers(player.get_position(), self.movement_value, true, true)
                                    );
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
                                        self.estimated_search_position = Some(get_estimated_position(&enemy_start, steps, direction, self.movement_value));
                                    }

                                    assert!(self.estimated_search_position != None, "estimated search position must exist");

                                    let estimated_search_position = self.estimated_search_position.unwrap();

                                    if !self.is_colliding && enemy_start != estimated_search_position {
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
                            self.near_hide_places_positions = None;

                            self.mode = EnemyMode::Regular;
                        }
                    }
                } else {
                    self.current_search_idx = 0;
                    self.near_hide_places_positions = None;

                    self.mode = EnemyMode::Regular;
                }

                self.already_detected_player = true;
                self.prev_mode = EnemyMode::Searching;
            }
        }

        new_notority_level
    }

    pub fn get_near_hide_places_positions(&self, start_position: Option<Position>, hide_places: &[HidePlace<'a>]) -> Vec<Position> {
        assert!(self.movement_grid != None, "movement grid can not be none");

        let grid_start_position = self.movement_grid.as_ref().unwrap().0;
        let grid = &self.movement_grid.as_ref().unwrap().1;

        let (enemy_start, ..) = self.get_calc_position();
        let start_position = start_position.unwrap_or(enemy_start).to_grid_position(grid_start_position, self.movement_value);

        let mut q: Queue<GridPosition> = Queue::new();
        q.add(start_position).unwrap();

        let mut visited: HashSet<GridPosition> = HashSet::new();
        visited.insert(start_position);

        let is_colliding_with_hide_place = | start_position: Position, (hide_place_start, hide_place_end): (Position, Position) | -> bool {
            let end_position = start_position + self.size;

            start_position.x <= hide_place_end.x &&
            end_position.x >= hide_place_start.x &&
            start_position.y <= hide_place_end.y &&
            end_position.y >= hide_place_start.y
        };

        let is_hide_place = | position: Position | -> Option<Position> {
            for hide_place in hide_places {
                let (start, end) = hide_place.get_calc_position();

                if is_colliding_with_hide_place(position, (start, end)) {
                    return Some(hide_place.get_position());
                }
            }

            None
        };

        while let Ok(current_position) = q.remove() {
            if let Some(hide_place_position) = is_hide_place(current_position.to_position(grid_start_position, self.movement_value)) {
                return vec![hide_place_position];
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

        vec![]
    }

    fn get_default_search_path(&self) -> PathVec {
        let steps = 60 / self.movement_value as u32;

        vec![(steps, Direction::Left, 0), (steps, Direction::Right, 0), (steps, Direction::Up, 0), (steps * 2, Direction::Down, 0), (steps, Direction::Up, 0), (steps, Direction::Right, 0)]
    } 

    pub fn search(&mut self, target_search_position: Position) {
        self.mode = EnemyMode::Searching;
        self.start_searching_position = Some(target_search_position);
    }

    pub fn is_detecting_player(&self, player: &Player<'a>, walls: &[Wall<'a>], doors: &[Door<'a>]) -> bool {
        if player.get_status() == &PlayerStatus::Hidden {
            return false;
        }

        let (player_start, player_end) = player.get_calc_position();

        let (enemy_start, enemy_end) = self.get_calc_position();

        if ((player_start.x >= enemy_start.x && player_start.x <= enemy_end.x) ||
            (player_end.x >= enemy_start.x && player_end.x <= enemy_end.x)) &&
            ((player_start.y >= enemy_start.y && player_start.y < enemy_end.y) || 
            (player_end.y > enemy_start.y && player_end.y <= enemy_end.y)) {
            return true;
        }

        if simple_object_detect_check(player.get_calc_position(), self.get_calc_position(), walls) {
            return false;
        }

        let (first_point, second_point, apex) = self.detect_traingle;

        let first_point = round_position_to_full_numbers(first_point, self.movement_value, true, true);
        let second_point = round_position_to_full_numbers(second_point, self.movement_value, true, true);
        let apex = round_position_to_full_numbers(apex, self.movement_value, true, true);

        let is_seeing = match self.moving_towards {
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

        if is_seeing && bfs_object_detect_check(player.get_calc_position(), self.get_calc_position(), walls, doors, player.get_movement_value()) {
            return false;
        }

        is_seeing
    }

    pub fn detect_player(&mut self, current_notoriety_level: u64, player: &mut Player<'a>, walls: &[Wall<'a>], doors: &[Door<'a>]) -> u64 {
        let is_detected = self.is_detecting_player(player, walls, doors);

        if is_detected {
            let player_start = player.get_position();

            player.set_status(PlayerStatus::Detectit);
            player.set_is_detected_by_enemy(true);

            self.mode = EnemyMode::Detecting;
            self.detect_player_position = Some(round_position_to_full_numbers(player_start, self.movement_value, true, true));
    
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

    pub fn attach_camera(&mut self, detected_player_position: Position) {
        self.mode = EnemyMode::Detecting;
        self.detect_player_position = Some(round_position_to_full_numbers(detected_player_position, self.movement_value, true, true));
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

        let (start, end) = self.get_calc_position();

        start.x > (start_position.x + size.width) ||
        end.x > (start_position.x + size.width) ||
        start.x < start_position.x ||
        start.y > (start_position.y + size.height) ||
        end.y > (start_position.y + size.height) ||
        start.y < start_position.y
    }
}
