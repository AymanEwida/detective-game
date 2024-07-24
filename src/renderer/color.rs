#[derive(Debug)]
pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl Color {
    pub fn get_color_in_f32(self) -> (f32, f32, f32, f32) {
        match self {
            Self::White => (1.0, 1.0, 1.0, 1.0),
            Self::Black => (0.0, 0.0, 0.0, 1.0),
            Self::Red => (1.0, 0.0, 0.0, 1.0),
            Self::Green => (0.0, 1.0, 0.0, 1.0),
            Self::Blue => (0.0, 0.0, 1.0, 1.0),
            Self::RGB(red, green, blue) => (red as f32/255.0, green as f32/255.0, blue as f32/255.0, 1.0),
            Self::RGBA(red, green, blue, alpha) => (red as f32/255.0, green as f32/255.0, blue as f32/255.0, alpha as f32/255.0)
        }
    }

    pub fn get_vertices_color_in_f32(&self) -> [f32; 4] {
        match self {
            Self::White => [1.0, 1.0, 1.0, 1.0],
            Self::Black => [0.0, 0.0, 0.0, 1.0],
            Self::Red => [1.0, 0.0, 0.0, 1.0],
            Self::Green =>[0.0, 1.0, 0.0, 1.0],
            Self::Blue => [0.0, 0.0, 1.0, 1.0],
            Self::RGB(red, green, blue) => [*red as f32/255.0, *green as f32/255.0, *blue as f32/255.0, 1.0],
            Self::RGBA(red, green, blue, alpha) => [*red as f32/255.0, *green as f32/255.0, *blue as f32/255.0, *alpha as f32/255.0]
        }
    }
}
