use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::{character::Character, level::{GameObject, DEFAULT_SIZE}, level_object::{LevelObject, ObjectType}};

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
    original_image: &'a str,
    image: &'a str,
    scale: Option<f32>,
    rotate: Option<f32>,
    is_closed: bool,
    is_locked: bool,
    opens_by: Option<usize>,
}

impl<'a> GameObject<'a> for Door<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, false, None, self.scale, None, self.rotate)?;

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
    pub fn new(id: usize, door_type: DoorType, position: Position, size: Size, is_locked: bool, opens_by: Option<usize>, scale: Option<f32>, rotate: Option<f32>) -> std::result::Result<Self, String> {
        if is_locked && opens_by.is_none() {
            return Err("Please provide a opens_by id to a locked door".to_string());
        }

        let door_image_path = match door_type {
            DoorType::Regular => "assets/game/regular-close-door.png",
            DoorType::Locked => "assets/game/locked-door.png",
            DoorType::Coded => "assets/game/coded-door.png",
            _ => {
                return Err("Please provide only Regular, Locked, Coded".to_string());
            }
        };
        
        Ok(Self {
            id,
            door_type,
            position, 
            size, 
            original_image: door_image_path,
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
    pub fn get_door_type(&self) -> &DoorType {
        &self.door_type
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_opens_by_id(&self) -> Option<usize> {
        self.opens_by
    }

    pub fn open(&mut self) {
        if !self.is_locked {
            self.is_closed = false;
            self.image = "assets/game/regular-open-door.png";
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        self.image = self.original_image;
    }

    pub fn unlock(&mut self) {
        if self.is_locked && self.opens_by.is_some() {
            self.is_locked = false;
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
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
    character_move_position: Position,
}

impl<'a> GameObject<'a> for TeleportDoor<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, false, None, self.scale, None, self.rotate)?;

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
    pub fn new(id: usize, position: Position, connected_to: usize, character_move_position: Position, scale: Option<f32>, rotate: Option<f32>) -> Self {
        Self {
            id,
            position, 
            size: DEFAULT_SIZE_FOR_TELEPORT_DOOR, 
            image: "assets/game/teleport-door.webp",
            scale,
            rotate,
            connected_to,
            character_move_position,
        }
    }
}

impl<'a> TeleportDoor<'a> {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_connected_door_id(&self) -> usize {
        self.connected_to
    }

    pub fn get_character_move_position(&self) -> Position {
        self.character_move_position
    }

    pub fn teleport(&self, character: &mut impl Character<'a>, teleport_doors: &[TeleportDoor]) {
        let mut teleport_door_idx: i32 = -1;

        for i in 0..teleport_doors.len() {
            let teleport_door = &teleport_doors[i];

            if teleport_door.id == self.connected_to {
                teleport_door_idx = i as i32;

                break;
            } 
        }

        if teleport_door_idx != -1 {
            let teleport_door = &teleport_doors[teleport_door_idx as usize];

            character.set_position(teleport_door.get_position());
        } else {
            character.set_position(self.character_move_position);
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExitDoor<'a> {
    position: Position,
    size: Size,
    image: &'a str,
    scale: Option<f32>,
}

impl<'a> GameObject<'a> for ExitDoor<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, false, None, self.scale, None, None)?;

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
    pub fn new(position: Position, scale: Option<f32>) -> Self {
        Self {
            position, 
            size: DEFAULT_SIZE_FOR_EXIT_DOOR, 
            image: "assets/game/exit-door.png",
            scale,
        }
    }
}