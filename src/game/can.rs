use std::time::{Duration, Instant};

use crate::{library::{constants::DEFAULT_MOVEMENT_VALUE, utils::{calc_control_point_game_coordinate_system, calculate_calc_position, length_of_line}}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

use super::{level::{EndStartPositions, GameObject}, player::DEFAULT_SIZE_FOR_INVENTORY_ITEM};

#[derive(Debug, Clone)]
pub struct Can<'a> {
    start_position: Position,
    current_position: Position,
    end_position: Position,
    image: &'a str,
    size: Size,
    calc_position: EndStartPositions,
    length: usize,
    detect_radius: f32,
    control_point: Position,
    iter_num: usize,
    is_finished: bool,
    done: bool,
    last_hit_time: Instant,
    hit_duration_interval: Duration,
    added_detect_range: bool
}

impl<'a> Can<'a> {
    pub fn new(start_position: Position, end_position: Position, image: &'a str, detect_radius: f32) -> Self {
        Self {
            start_position,
            current_position: start_position,
            end_position,
            image,
            size: DEFAULT_SIZE_FOR_INVENTORY_ITEM,
            calc_position: calculate_calc_position(start_position, DEFAULT_SIZE_FOR_INVENTORY_ITEM, DEFAULT_MOVEMENT_VALUE),
            length: length_of_line(&start_position, &end_position) as usize,
            detect_radius,
            control_point: calc_control_point_game_coordinate_system(&start_position, &end_position),
            iter_num: 0,
            is_finished: start_position == end_position,
            done: false,
            last_hit_time: Instant::now(),
            hit_duration_interval: Duration::from_millis(1000),
            added_detect_range: false
        }
    }
}

impl<'a> Can<'a> {
    pub fn draw(&mut self, render: &mut Render<'a>) -> Result<()> {
        if !self.is_finished {
            render.load_image(self.image, self.current_position, self.size, false, None, None, None, None)?;
        } else {
            if self.last_hit_time.elapsed() <= self.hit_duration_interval {
                render.load_image("assets/game/can-hit-effect.png", self.current_position - self.size.width / 2.0, self.size, false, None, None, None, None)?;
                render.draw_geometric_object(self.current_position, self.detect_radius, Color::RGBA(0, 0, 255, 50), None, None, None, None);
            } else {
                self.done = true;
            }
        }
        

        Ok(())
    }

    pub fn get_position(&self) -> Position {
        self.current_position
    }
    
    pub fn set_position(&mut self, new_position: Position) {
        self.current_position = new_position;
        self.set_calc_position();
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn get_calc_position(&self) -> EndStartPositions {
        self.calc_position        
    }
}

impl<'a> Can<'a> {
    pub fn get_is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn set_is_finished(&mut self, new_is_finished: bool) {
        self.is_finished = new_is_finished;
        self.last_hit_time = Instant::now();
    }

    pub fn get_start_position(&self) -> Position {
        self.start_position
    }

    pub fn get_end_position(&self) -> Position {
        self.end_position
    }

    pub fn get_done(&self) -> bool {
        self.done
    }

    pub fn get_added_detect_range(&self) -> bool {
        self.added_detect_range
    }

    pub fn set_added_detect_range(&mut self, new_val: bool) {
        self.added_detect_range = new_val;
    }

    fn set_calc_position(&mut self) {
        self.calc_position = calculate_calc_position(self.current_position, self.size, DEFAULT_MOVEMENT_VALUE);
    }

    pub fn calc_next_position(&mut self) {
        let t = self.iter_num as f32 / self.length as f32;

        let x = (1.0 - t).powi(2) * self.start_position.x + 2.0 * (1.0 - t) * t * self.control_point.x + t.powi(2) * self.end_position.x;
        let y = (1.0 - t).powi(2) * self.start_position.y + 2.0 * (1.0 - t) * t * self.control_point.y + t.powi(2) * self.end_position.y;

        self.current_position = Position { x, y };

        self.iter_num += 3;

        self.set_calc_position();

        if self.start_position >= self.current_position {
            if self.iter_num >= self.length {
                self.is_finished = true;
            }
        } else {
            if self.iter_num >= self.length || self.current_position >= self.end_position {
                self.is_finished = true;
            }
        }

        if self.is_finished {
            self.last_hit_time = Instant::now();
        }
    }

    pub fn collide(&self, other: &impl GameObject<'a>) -> bool {
        let (start_position, end_position) = self.get_calc_position();
        let (other_start_position, other_end_position) = other.get_calc_position();
    
        start_position.x < other_end_position.x &&
        end_position.x > other_start_position.x &&
        start_position.y < other_end_position.y &&
        end_position.y > other_start_position.y
    }
}


