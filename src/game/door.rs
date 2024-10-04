use rand::Error;

use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::{level::{GameObject, DEFAULT_SIZE}, level_object::{LevelObject, ObjectType}};

pub const DEFAULT_SIZE_FOR_TELEPORT_DOOR: Size = Size { width: DEFAULT_SIZE + 20.0, height: 70.0 };
pub const DEFAULT_SIZE_FOR_EXIT_DOOR: Size = Size { width: 70.0, height: 60.0 };

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DoorType {
    Regular,
    Locked,
    Coded,
    TeleportDoor,
    ExitDoor,
}

#[derive(Debug)]
pub struct Door<'a> {
    id: usize,
    door_type: DoorType,
    position: Position,
    size: Size,
    image: &'a str,
    scale: Option<f32>,
    rotate: Option<f32>,
    is_closed: bool,
    is_locked: bool,
    opens_by: Option<usize>,
}

impl<'a> GameObject<'a> for Door<'a> {
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

impl<'a> LevelObject<'a> for Door<'a> {
    fn get_type(&self) -> ObjectType {
        ObjectType::Door(self.door_type)
    } 
}

impl Door<'_> {
    pub fn new(id: usize, door_type: DoorType, position: Position, size: Size, is_locked: bool, opens_by: Option<usize>, scale: Option<f32>, rotate: Option<f32>) -> std::result::Result<Self, Error> {
        if is_locked && opens_by.is_none() {
            return Err(Error::new("Please provide a opens_by id to a locked door"));
        }

        let door_image_path = match door_type {
            DoorType::Regular => "assets/game/regular-close-door.png",
            DoorType::Locked => "assets/game/locked-door.png",
            DoorType::Coded => "assets/game/coded-door.png",
            _ => {
                return Err(Error::new("Please provide only Regular, Locked, Coded"));
            }
        };
        
        Ok(Self {
            id,
            door_type,
            position, 
            size, 
            image: door_image_path,
            scale,
            rotate,
            is_closed: true,
            is_locked,
            opens_by
        })
    }
    
}

impl Door<'_> {
    pub fn open(&mut self) {
        if !self.is_locked {
            self.image = "assets/game/regular-open-door.png";
        }
    }

    pub fn close(&mut self) {
        self.image = "assets/game/regular-close-door.png";
    }

    pub fn unlock(&mut self) {
        if self.is_locked && self.opens_by.is_some() {
            self.is_locked = false;
        }
    }
}

#[derive(Debug)]
pub struct TeleportDoor<'a> {
    id: usize,
    position: Position,
    size: Size,
    image: &'a str,
    scale: Option<f32>,
    rotate: Option<f32>,
    connected_to: usize,
    player_move_position: Position,
}

impl<'a> GameObject<'a> for TeleportDoor<'a> {
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

impl<'a> LevelObject<'a> for TeleportDoor<'a> {
    fn get_type(&self) -> ObjectType {
        ObjectType::Door(DoorType::TeleportDoor)
    } 
}

impl TeleportDoor<'_> {
    pub fn new(id: usize, position: Position, connected_to: usize, player_move_position: Position, scale: Option<f32>, rotate: Option<f32>) -> Self {
        Self {
            id,
            position, 
            size: DEFAULT_SIZE_FOR_TELEPORT_DOOR, 
            image: "assets/game/teleport-door.webp",
            scale,
            rotate,
            connected_to,
            player_move_position,
        }
    }
}

#[derive(Debug)]
pub struct ExitDoor<'a> {
    position: Position,
    size: Size,
    image: &'a str,
}

impl<'a> GameObject<'a> for ExitDoor<'a> {
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

impl<'a> LevelObject<'a> for ExitDoor<'a> {
    fn get_type(&self) -> ObjectType {
        ObjectType::Door(DoorType::ExitDoor)
    } 
}

impl ExitDoor<'_> {
    pub fn new(position: Position) -> Self {
        Self {
            position, 
            size: DEFAULT_SIZE_FOR_EXIT_DOOR, 
            image: "assets/game/exit-door.png",
        }
    }
}