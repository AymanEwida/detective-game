use super::{color::Color, render::Size};

pub type PositionType = [f32; 2];

#[derive(Debug)]
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

#[derive(Debug)]
pub struct TextureVertice(pub Position);

#[repr(C, packed)]
#[derive(Debug, PartialEq)]
pub struct _VerticeData(pub PositionType, pub [f32; 3]);

impl Vertice {
    pub fn get_vertices_data(self) -> _VerticeData {
        _VerticeData(self.0.get_vertice_position(None), self.1.get_vertices_color_in_f32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vertice_position_without_size() {
        let vertice_position = Position { x: 200.0, y: -300.0 };
        let actual = vertice_position.get_vertice_position(None);
        let expected = [200.0, -300.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertice_position_with_width() {
        let vertice_position = Position { x: 200.0, y: -300.0 };
        let actual = vertice_position.get_vertice_position(Some(&Size { width: 100, height: 0 }));
        let expected = [300.0, -300.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertice_position_with_height() {
        let vertice_position = Position { x: 200.0, y: -300.0 };
        let actual = vertice_position.get_vertice_position(Some(&Size { width: 0, height: 100 }));
        let expected = [200.0, -400.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertice_position_with_full_size() {
        let vertice_position = Position { x: 200.0, y: -300.0 };
        let actual = vertice_position.get_vertice_position(Some(&Size { width: 200, height: 100 }));
        let expected = [400.0, -400.0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_vertices_data() {
        let vertice = Vertice(Position { x: 50.0, y: 25.0 }, Color::Black);
        let actual = vertice.get_vertices_data();
        let expected = _VerticeData([50.0, 25.0], [0.0, 0.0, 0.0]);

        assert_eq!(actual, expected);
    }
}
