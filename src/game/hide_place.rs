use crate::{library::{constants::DEFAULT_MOVEMENT_VALUE, utils::calculate_calc_position}, renderer::{error::Result, render::{Render, Size}, vertice::Position}};

use super::{level::{EndStartPositions, GameObject}, level_object::{LevelObject, ObjectType}};

pub const DEFAULT_SIZE_FOR_HIDE_PLACE: Size = Size { width: 45.0, height: 65.0 };

#[derive(Debug)]
pub struct HidePlace<'a> {
    position: Position,
    calc_position: EndStartPositions,
    size: Size,
    image: &'a str,
    scale: Option<f32>,
}

impl<'a> GameObject<'a> for HidePlace<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, false, None, self.scale, None, None)?;

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

impl<'a> LevelObject<'a> for HidePlace<'a> {
    fn get_type(&self) -> ObjectType {
        ObjectType::HidePlace
    }
}

impl HidePlace<'_> {
    pub fn new(position: Position, scale: Option<f32>) -> Self {
        Self {
            position, 
            calc_position: calculate_calc_position(position, DEFAULT_SIZE_FOR_HIDE_PLACE, DEFAULT_MOVEMENT_VALUE),
            size: DEFAULT_SIZE_FOR_HIDE_PLACE, 
            image: "assets/game/hide-place1.webp",
            scale,
        }
    }
}

impl HidePlace<'_> {
    fn set_calc_position(&mut self) {
        self.calc_position = calculate_calc_position(self.position, self.size, DEFAULT_MOVEMENT_VALUE);
    }
}