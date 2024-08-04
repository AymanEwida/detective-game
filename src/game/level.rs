use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

pub trait GameObject<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()>;
    fn get_position(&self) -> Position;
    fn get_size(&self) -> Size;
}

pub enum ObjectLevelType {
    Wall
}

pub struct ObjectLevel<'a> {
    position: Position,
    size: Size,
    image: &'a str
}

impl ObjectLevel<'_> {
    pub fn new(object_type: ObjectLevelType, position: Position, size: Size) -> Self {
        match object_type {
            ObjectLevelType::Wall => {
                Self {
                    position,
                    size,
                    image: "assets/game/wall.jpg"
                }
            }
        }
    }
}

impl<'a> GameObject<'a> for ObjectLevel<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size)?;

        Ok(())
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }
}
