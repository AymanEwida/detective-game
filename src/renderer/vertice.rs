use crate::library::utils::convert_coordinates;

use super::{color::Color, render::Size};

pub type PositionType = [f32; 2];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn get_vertice_position(&self, size: Option<&Size>) -> PositionType {
        match size {
            Some(Size { width, height }) => {
                let x = self.x + width;
                let y = self.y - height;
                
                [x, y]
            },
            None => [self.x, self.y]
        }
    }

    pub fn get_vertice_position_with_rotate(&self, size: Option<&Size>, angle: f32) -> PositionType {
        assert!(angle >= 0.0 && angle <= 360.0, "rotate must be between 0.0 - 360.0 (includes)");

        match size {
            Some(Size { width, height }) => {
                let x = self.x + width;
                let y = self.y - height;
                
                [x * angle.cos() - y * angle.sin(), x * angle.sin() + y * angle.cos()]
            },
            None => {
                let x = self.x * angle.cos() - self.y * angle.sin();
                let y = self.x * angle.sin() + self.y * angle.cos();
                
                [x, y]
            },
        }
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
    pub fn get_vertice_data(self, size: &Size<f32>) -> _VerticeData {
        let new_position = convert_coordinates(self.0, size);

        _VerticeData(new_position.get_vertice_position(None), self.1.get_vertices_color_in_f32())
    }
}
