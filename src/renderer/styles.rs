use std::ops::Div;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size<T = f32> {
    pub width: T,
    pub height: T,
}

impl Div<f32> for Size {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        assert!(rhs != 0.0, "can not divide by zero!");

        Self {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }
}

impl Padding {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn new_padding_x_y(x: f32, y: f32) -> Self {
        Self {
            left: x,
            right: x,
            top: y,
            bottom: y,
        }
    }

    pub fn none() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }
}
