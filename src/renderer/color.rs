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

    pub fn get_vertices_color_in_f32(&self) -> [f32; 3] {
        match self {
            Self::White => [1.0, 1.0, 1.0],
            Self::Black => [0.0, 0.0, 0.0],
            Self::Red => [1.0, 0.0, 0.0],
            Self::Green =>[0.0, 1.0, 0.0],
            Self::Blue => [0.0, 0.0, 1.0],
            Self::RGB(red, green, blue) => [*red as f32/255.0, *green as f32/255.0, *blue as f32/255.0],
            _ => [0.0, 0.0, 0.0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_white_in_f32() {
        let color = Color::White;
        let actual = color.get_color_in_f32();
        let expected = (1.0, 1.0, 1.0, 1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_black_in_f32() {
        let color = Color::Black;
        let actual = color.get_color_in_f32();
        let expected = (0.0, 0.0, 0.0, 1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_red_in_f32() {
        let color = Color::Red;
        let actual = color.get_color_in_f32();
        let expected = (1.0, 0.0, 0.0, 1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_green_in_f32() {
        let color = Color::Green;
        let actual = color.get_color_in_f32();
        let expected = (0.0, 1.0, 0.0, 1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_blue_in_f32() {
        let color = Color::Blue;
        let actual = color.get_color_in_f32();
        let expected = (0.0, 0.0, 1.0, 1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_rgb_in_f32() {
        let color = Color::RGB(50, 50, 50);
        let actual = color.get_color_in_f32();
        let expected = (0.19607843, 0.19607843, 0.19607843, 1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_rgba_in_f32() {
        let color = Color::RGBA(255, 255, 0, 255);
        let actual = color.get_color_in_f32();
        let expected = (1.0, 1.0, 0.0, 1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertices_white_in_f32() {
        let color = Color::White;
        let actual = color.get_vertices_color_in_f32();
        let expected = [1.0, 1.0, 1.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertices_black_in_f32() {
        let color = Color::Black;
        let actual = color.get_vertices_color_in_f32();
        let expected = [0.0, 0.0, 0.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertices_red_in_f32() {
        let color = Color::Red;
        let actual = color.get_vertices_color_in_f32();
        let expected = [1.0, 0.0, 0.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertices_green_in_f32() {
        let color = Color::Green;
        let actual = color.get_vertices_color_in_f32();
        let expected = [0.0, 1.0, 0.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertices_blue_in_f32() {
        let color = Color::Blue;
        let actual = color.get_vertices_color_in_f32();
        let expected = [0.0, 0.0, 1.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertices_rgb_in_f32() {
        let color = Color::RGB(255, 255, 0);
        let actual = color.get_vertices_color_in_f32();
        let expected = [1.0, 1.0, 0.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertices_defalut_in_f32() {
        let color = Color::RGBA(255, 255, 255, 255);
        let actual = color.get_vertices_color_in_f32();
        let expected = [0.0, 0.0, 0.0];

        assert_eq!(actual, expected);
    }
}
