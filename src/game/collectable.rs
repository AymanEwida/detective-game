use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::{level::{GameObject, DEFAULT_SIZE}, level_object::{LevelObject, ObjectType}};

pub const DEFAULT_SIZE_FOR_COLLECTABLE: Size = Size { width: 40.0, height: 40.0 };

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DoorCollectableType {
    CodePaper,
    Key,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollectableType {
    Coin,
    DoorCollectable(DoorCollectableType),
}

#[derive(Debug)]
pub struct DoorCollectable<'a> {
    id: usize,
    door_collectable_type: DoorCollectableType,
    position: Position,
    size: Size,
    image: &'a str,
    opens: usize,
}

impl<'a> GameObject<'a> for DoorCollectable<'a> {
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

impl<'a> LevelObject<'a> for DoorCollectable<'a> {
    fn get_type(&self) -> ObjectType {
        ObjectType::Collectable(CollectableType::DoorCollectable(self.door_collectable_type))
    }
}

impl DoorCollectable<'_> {
    pub fn new(id: usize, door_collectable_type: DoorCollectableType, position: Position, opens: usize) -> Self {
        let door_collectable_image_path = match &door_collectable_type {
            DoorCollectableType::CodePaper => "assets/game/code-paper.webp",
            DoorCollectableType::Key => "assets/game/key.png",
        };
        
        Self {
            id,
            door_collectable_type,
            position, 
            size: DEFAULT_SIZE_FOR_COLLECTABLE,
            image: door_collectable_image_path,
            opens,
        }
    }
}

#[derive(Debug)]
pub struct Coin<'a> {
    position: Position,
    size: Size,
    image: &'a str,
}

impl<'a> GameObject<'a> for Coin<'a> {
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

impl<'a> LevelObject<'a> for Coin<'a> {
    fn get_type(&self) -> ObjectType {
        ObjectType::Collectable(CollectableType::Coin)
    }
}

impl Coin<'_> {
    pub fn new(position: Position) -> Self {
        Self {
            position, 
            size: DEFAULT_SIZE_FOR_COLLECTABLE, 
            image: "assets/game/coin.png",
        }
    }
}