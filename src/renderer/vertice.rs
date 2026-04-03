use std::{
    hash::Hash,
    ops::{Add, Mul, Sub},
};

use crate::library::utils::length_of_line;

use super::{color::Color, styles::Size};

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
            y: self.y + rhs,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Size> for Position {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        Self {
            x: self.x + rhs.width,
            y: self.y + rhs.height,
        }
    }
}

impl Sub<f32> for Position {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Size> for Position {
    type Output = Self;

    fn sub(self, rhs: Size) -> Self::Output {
        Self {
            x: self.x - rhs.width,
            y: self.y - rhs.height,
        }
    }
}

impl Mul<f32> for Position {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Position {
    pub fn get_position_from_size(&self, size: &Size) -> Self {
        let x = self.x + size.width;
        let y = self.y + size.height;

        Self { x, y }
    }

    pub fn rotate(&self, rotation_center: Position, angle_in_radians: f32) -> Self {
        let translated_x = self.x - rotation_center.x;
        let translated_y = self.y - rotation_center.y;

        let rotated_x =
            translated_x * angle_in_radians.cos() - translated_y * angle_in_radians.sin();
        let rotated_y =
            translated_x * angle_in_radians.sin() + translated_y * angle_in_radians.cos();

        Self {
            x: rotated_x + rotation_center.x,
            y: rotated_y + rotation_center.y,
        }
    }

    pub fn to_position_array(&self) -> PositionType {
        [self.x, self.y]
    }

    pub fn get_neighbors(&self, value: f32) -> Vec<Self> {
        vec![
            Self {
                x: self.x,
                y: self.y - value,
            },
            Self {
                x: self.x,
                y: self.y + value,
            },
            Self {
                x: self.x - value,
                y: self.y,
            },
            Self {
                x: self.x + value,
                y: self.y,
            },
        ]
    }

    pub fn lenght(&self, window_start: &Position) -> f32 {
        length_of_line(self, window_start)
    }

    pub fn normalize(&self, window_start: &Position) -> Self {
        let lenght = self.lenght(window_start);

        if lenght > 0.0 {
            Self {
                x: self.x / lenght,
                y: self.y / lenght,
            }
        } else {
            *self
        }
    }

    pub fn to_grid_position(&self, grid_start_position: Position, value: f32) -> GridPosition {
        let col = (self.x - grid_start_position.x) / value;
        let row = (self.y - grid_start_position.y) / value;

        GridPosition {
            row: row as usize,
            col: col as usize,
            distance: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize,
    pub distance: usize,
}

impl From<(Position, Position, f32)> for GridPosition {
    fn from(value: (Position, Position, f32)) -> Self {
        value.0.to_grid_position(value.1, value.2)
    }
}

impl GridPosition {
    pub fn get_neighbors(&self) -> Vec<Self> {
        let mut neighbors = vec![
            Self {
                row: self.row + 1,
                col: self.col,
                distance: 0,
            },
            Self {
                row: self.row,
                col: self.col + 1,
                distance: 0,
            },
        ];

        if self.row != 0 {
            neighbors.push(Self {
                row: self.row - 1,
                col: self.col,
                distance: 0,
            });
        }

        if self.col != 0 {
            neighbors.push(Self {
                row: self.row,
                col: self.col - 1,
                distance: 0,
            });
        }

        neighbors
    }

    pub fn to_position(&self, grid_start_position: Position, value: f32) -> Position {
        let col = self.col as f32;
        let row = self.row as f32;

        Position {
            x: (col * value) + grid_start_position.x,
            y: (row * value) + grid_start_position.y,
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
    pub fn get_vertice_data(&self) -> _VerticeData {
        _VerticeData(
            self.0.to_position_array(),
            self.1.get_vertices_color_in_f32(),
        )
    }
}
