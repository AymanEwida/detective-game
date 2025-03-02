use std::{hash::Hash, ops::{Add, Sub}};

use crate::library::utils::convert_coordinates;

use super::{color::Color, render::Size};

pub type PositionType = [f32; 2];

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Hash for Position {
   fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
       state.write_i32(self.x as i32);
       state.write_i32(self.y as i32);
   } 
}

impl Eq for Position {}

impl Add<f32> for Position {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<Size> for Position {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        Self {
            x: self.x + rhs.width,
            y: self.y + rhs.height
        }
    }
}

impl Sub<f32> for Position {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Sub<Size> for Position {
    type Output = Self;

    fn sub(self, rhs: Size) -> Self::Output {
        Self {
            x: self.x - rhs.width,
            y: self.y - rhs.height
        }
    }
}

impl Position {
    pub fn get_position_from_size(&self, size: &Size) -> Self {
        let x = self.x + size.width;
        let y = self.y - size.height;

        Self {
            x, 
            y
        }
    }
    
    pub fn rotate(&self, rotation_center: Position, angle_in_radians: f32) -> Self {
        let translated_x = self.x - rotation_center.x;
        let translated_y = self.y - rotation_center.y;

        let rotated_x = translated_x * angle_in_radians.cos() - translated_y * angle_in_radians.sin();
        let rotated_y = translated_x * angle_in_radians.sin() + translated_y * angle_in_radians.cos();

        Self {
            x: rotated_x + rotation_center.x,
            y: rotated_y + rotation_center.y
        }
    }

    pub fn to_position_array(&self) -> PositionType {
        [self.x, self.y]
    }

    pub fn get_neighbors(&self, value: f32) -> Vec<Self> {
        vec![
            Self { x: self.x, y: self.y - value },
            Self { x: self.x, y: self.y + value },
            Self { x: self.x - value, y: self.y },
            Self { x: self.x + value, y: self.y },
        ]
    }
}

#[derive(Debug)]
pub struct Vertice(pub Position, pub Color);

#[repr(C, packed)]
#[derive(Debug, PartialEq)]
pub struct _TextureVerticeData(pub PositionType, pub [f32; 2]);

#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone)]
pub struct _VerticeData(pub PositionType, pub [f32; 4]);

impl Vertice {
    pub fn get_vertice_data(&self, size: &Size<f32>) -> _VerticeData {
        let new_position = convert_coordinates(self.0, size);

        _VerticeData(new_position.to_position_array(), self.1.get_vertices_color_in_f32())
    }
}
