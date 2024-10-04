use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::{level::GameObject, level_object::{LevelObject, ObjectType}};

#[derive(Debug)]
pub struct Wall<'a> {
    position: Position,
    size: Size,
    image: &'a str,
    scale: Option<f32>,
    rotate: Option<f32>,
}

impl<'a> GameObject<'a> for Wall<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, false, self.scale, None, self.rotate)?;

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

impl<'a> LevelObject<'a> for Wall<'a> {
    fn get_type(&self) -> ObjectType {
        ObjectType::Wall
    } 
}

impl Wall<'_> {
    pub fn new(position: Position, size: Size, scale: Option<f32>, rotate: Option<f32>) -> Self {
        Self {
            position, 
            size, 
            image: "assets/game/wall.jpg",
            scale,
            rotate
        }
    }
    
}
