use crate::{library::utils::length_of_line, renderer::vertice::Position};

use super::level::GameObject;

#[derive(Debug)]
pub struct DetectRange {
    radius: f32,
    center_position: Position,
}

impl DetectRange {
    pub fn new(radius: f32, center_position: Position) -> Self {
        Self {
            radius,
            center_position
        }
    }
}

impl DetectRange {
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
    
    pub fn get_center_position(&self) -> Position {
        self.center_position
    }

    pub fn is_in_range<'a>(&self, object: &impl GameObject<'a>) -> bool {
        length_of_line(&object.get_calc_position().0, &self.center_position) <= self.radius
    }
}


