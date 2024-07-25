use super::{color::Color, render::Size};

pub type PositionType = [f32; 2];

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn get_vertice_position(&self, size: Option<&Size>) -> PositionType {
        match size {
            Some(Size { width, height }) => {
                let x = self.x + (*width as f32);
                let y = self.y - (*height as f32);
                
                [x, y]
            },
            None => [self.x, self.y]
        }
    }
}

#[derive(Debug)]
pub struct Vertice(pub Position, pub Color);

#[repr(C, packed)]
#[derive(Debug, PartialEq)]
pub struct _TextureVerticeData(pub PositionType, pub [f32; 2]);

#[repr(C, packed)]
#[derive(Debug, PartialEq)]
pub struct _VerticeData(pub PositionType, pub [f32; 4]);

impl Vertice {
    pub fn get_vertices_data(self) -> _VerticeData {
        _VerticeData(self.0.get_vertice_position(None), self.1.get_vertices_color_in_f32())
    }
}
