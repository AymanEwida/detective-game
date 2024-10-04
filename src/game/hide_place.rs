use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::{level::GameObject, level_object::{LevelObject, ObjectType}};

pub const DEFAULT_SIZE_FOR_HIDE_PLACE: Size = Size { width: 45.0, height: 65.0 };

pub struct HidePlace<'a> {
    position: Position,
    size: Size,
    image: &'a str,
}

impl<'a> GameObject<'a> for HidePlace<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, false, None, None, None)?;

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

impl<'a> LevelObject<'a> for HidePlace<'a> {
    fn get_type(&self) -> ObjectType {
        ObjectType::HidePlace
    }
}

impl HidePlace<'_> {
    pub fn new(position: Position) -> Self {
        Self {
            position, 
            size: DEFAULT_SIZE_FOR_HIDE_PLACE, 
            image: "assets/game/hide-place1.webp",
        }
    }
}