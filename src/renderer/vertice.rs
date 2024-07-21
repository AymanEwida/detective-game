use super::color::Color;

pub type Position = [f32; 2];

#[derive(Debug)]
pub struct Vertice(pub Position, pub Color);

#[repr(C, packed)]
#[derive(Debug, PartialEq)]
pub struct _VerticeData(pub Position, pub [f32; 3]);

impl Vertice {
    pub fn get_vertices_data(self) -> _VerticeData {
        _VerticeData(self.0, self.1.get_vertices_color_in_f32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vertices_data() {
        let vertice = Vertice([50.0, 25.0], Color::Black);
        let actual = vertice.get_vertices_data();
        let expected = _VerticeData([50.0, 25.0], [0.0, 0.0, 0.0]);

        assert_eq!(actual, expected);
    }
}
