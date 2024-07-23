use detective_game::renderer::{color::Color, render::Size, vertice::{Position, Vertice, _VerticeData}};

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