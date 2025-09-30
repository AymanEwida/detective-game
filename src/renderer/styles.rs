use std::ops::Div;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size<T=f32> {
    pub width: T,
    pub height: T
}

impl Div<f32> for Size {
    type Output = Self;
    
    fn div(self, rhs: f32) -> Self::Output {
        assert!(rhs != 0.0, "can not divide by zero!");

        Self {
            width: self.width / rhs,
            height: self.width / rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub x: f32,
    pub y: f32
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

impl Padding {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn new_padding_x(x: f32) -> Self {
        Self {
            x,
            y: 0.0
        }
    }

    pub fn new_padding_y(y: f32) -> Self {
        Self {
            x: 0.0,
            y
        }
    }
}

