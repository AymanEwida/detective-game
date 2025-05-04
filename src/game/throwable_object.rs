use crate::{library::{constants::DEFAULT_MOVEMENT_VALUE, utils::{calc_control_point_game_coordinate_system, calculate_calc_position, length_of_line}}, renderer::{error::Result, render::{Render, Size}, vertice::Position}};

use super::{level::{EndStartPositions, GameObject}, player::DEFAULT_SIZE_FOR_INVENTORY_ITEM};

pub enum PathType {
   Curved,
   StraightLine,
}

pub struct ThrowableObject<'a> {
    start_position: Position,
    current_position: Position,
    end_position: Position,
    path_type: PathType,
    image: &'a str,
    size: Size,
    calc_position: EndStartPositions,
    length: usize,
    control_point: Position,
    iter_num: usize,
    is_finished: bool,
}

impl<'a> ThrowableObject<'a> {
    pub fn new(start_position: Position, end_position: Position, path_type: PathType, image: &'a str) -> Self {
        Self {
            start_position,
            current_position: start_position,
            end_position,
            path_type,
            image,
            size: DEFAULT_SIZE_FOR_INVENTORY_ITEM,
            calc_position: calculate_calc_position(start_position, DEFAULT_SIZE_FOR_INVENTORY_ITEM, DEFAULT_MOVEMENT_VALUE),
            length: length_of_line(&start_position, &end_position) as usize,
            control_point: calc_control_point_game_coordinate_system(&start_position, &end_position),
            iter_num: 0,
            is_finished: start_position == end_position,
        }
    }
}

impl<'a> GameObject<'a> for ThrowableObject<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.current_position, self.size, false, None, None, None, None)?;
        

        Ok(())
    }

    fn get_position(&self) -> Position {
        self.current_position
    }
    
    fn set_position(&mut self, new_position: Position) {
        self.current_position = new_position;
        self.set_calc_position();
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_calc_position(&self) -> EndStartPositions {
        self.calc_position        
    }
}

impl ThrowableObject<'_> {
    pub fn get_is_finished(&self) -> bool {
        self.is_finished
    }

    fn set_calc_position(&mut self) {
        self.calc_position = calculate_calc_position(self.current_position, self.size, DEFAULT_MOVEMENT_VALUE);
    }

    pub fn calc_next_position(&mut self) {
        match self.path_type {
            PathType::Curved => {
                let t = self.iter_num as f32 / self.length as f32;

                let x = (1.0 - t).powi(2) * self.start_position.x + 2.0 * (1.0 - t) * t * self.control_point.x + t.powi(2) * self.end_position.x;
                let y = (1.0 - t).powi(2) * self.start_position.y + 2.0 * (1.0 - t) * t * self.control_point.y + t.powi(2) * self.end_position.y;

                self.current_position = Position { x, y };

                self.iter_num += 3;
            },

            PathType::StraightLine => {}
        }

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
    }
}


