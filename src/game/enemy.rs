use std::time::{Duration, Instant};

use crate::{library::utils::{calc_equidistant_points, convert_path}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::{Character, Direction}, level::GameObject, player::{Player, PlayerStatus}};

pub const DEFAULT_MOVE_INTERVAL: Duration = Duration::from_millis(300);

#[derive(Debug)]
pub enum EnemyType {
    Regular
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
    moves_path: Vec<(u32, Direction, u64)>,
    moves_count: u32,
    moving_towards: Direction,
    detect_traingle: (Position, Position, Position),
}

impl Enemy<'_> {
    pub fn new(enemy_type: EnemyType, start_position: Position, path: &str, flip: bool) -> Self {
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
                    moves_path,
                    moves_count: 0,
                    moving_towards: first_direction,
                    detect_traingle: calc_equidistant_points(Position { x: start_position.x + 27.5, y: start_position.y + 20.0 }, 30.0, 150.0, first_direction),
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
    
    pub fn move_enemy(&mut self, speed: Option<f32>) {
        if self.last_move_time.elapsed() >= self.move_interval {
            if let Some((moves_number, direction, wait_time)) = self.moves_path.first() {
                if *moves_number > 0 {
                    self.moving_towards = *direction;

                    self.move_character(*direction, speed);

                    self.moves_path[0].0 -= 1;
                    self.moves_count += 1;

                    if self.moves_count == 1 {
                        self.move_interval = DEFAULT_MOVE_INTERVAL;
                    }
                } else {
                    if *wait_time == 0 {
                        self.move_interval = DEFAULT_MOVE_INTERVAL;
                    } else {
                        self.move_interval = Duration::from_millis(*wait_time);
                    }

                    self.moves_path[0].0 = self.moves_count;
                    self.moves_path.rotate_left(1);

                    self.moves_count = 0;

                    self.move_enemy(speed);
                }

                self.detect_traingle = calc_equidistant_points(Position { x: self.position.x + 27.5, y: self.position.y + 20.0 }, 30.0, 150.0, self.moving_towards);
                self.last_move_time = Instant::now();
            }
        }
    }

    pub fn detect_player(&self, player: &mut Player<'a>) -> bool {
        if player.get_status() == &PlayerStatus::Hidden {
            return false;            
        }
        
        let player_start = player.get_position();
        let player_size = player.get_size();
        let player_end = Position { x: player_start.x + player_size.width, y: player_start.y + player_size.height };
        
        let (first_point, second_point, apex) = self.detect_traingle;

        // TODO: make detecting only when player inside the detecting triangle, find a way to calc the start point
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
}
