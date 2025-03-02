use detective_game::{library::utils::convert_angle_to_radians, renderer::{color::Color, render::Size, vertice::{Position, Vertice, _VerticeData}}};

#[test]
fn test_to_position_array() {
    let vertice_position = Position { x: 200.0, y: -300.0 };
    let actual = vertice_position.to_position_array();
    let expected = [200.0, -300.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_get_position_from_size_with_width() {
    let vertice_position = Position { x: 200.0, y: -300.0 };
    let actual = vertice_position.get_position_from_size(&Size { width: 100.0, height: 0.0 }).to_position_array();
    let expected = [300.0, -300.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_get_position_from_size_with_height() {
    let vertice_position = Position { x: 200.0, y: -300.0 };
    let actual = vertice_position.get_position_from_size(&Size { width: 0.0, height: 100.0 }).to_position_array();
    let expected = [200.0, -400.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_get_position_from_size_with_full_size() {
    let vertice_position = Position { x: 200.0, y: -300.0 };
    let actual = vertice_position.get_position_from_size(&Size { width: 200.0, height: 100.0 }).to_position_array();
    let expected = [400.0, -400.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_rotate_position_0_degrees() {
    let vertice_position = Position { x: 10.0, y: 10.0 };
    let input_center = Position { x: 0.0, y: 0.0 };

    let actual = vertice_position.rotate(input_center, convert_angle_to_radians(0.0));
    let expected = Position { x: 10.0, y: 10.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_rotate_position_90_degrees() {
    let vertice_position = Position { x: 10.0, y: 10.0 };
    let input_center = Position { x: 0.0, y: 0.0 };

    let actual = vertice_position.rotate(input_center, convert_angle_to_radians(90.0));
    let expected = Position { x: -10.0, y: 10.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_rotate_position_180_degrees() {
    let vertice_position = Position { x: 10.0, y: 10.0 };
    let input_center = Position { x: 0.0, y: 0.0 };

    let actual = vertice_position.rotate(input_center, convert_angle_to_radians(180.0));
    let expected = Position { x: -9.999999, y: -10.000001 };

    assert_eq!(actual, expected);
}

#[test]
fn test_rotate_position_270_degrees() {
    let vertice_position = Position { x: 10.0, y: 10.0 };
    let input_center = Position { x: 0.0, y: 0.0 };

    let actual = vertice_position.rotate(input_center, convert_angle_to_radians(270.0));
    let expected = Position { x: 10.0, y: -10.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_rotate_position_360_degrees() {
    let vertice_position = Position { x: 10.0, y: 10.0 };
    let input_center = Position { x: 0.0, y: 0.0 };

    let actual = vertice_position.rotate(input_center, convert_angle_to_radians(360.0));
    let expected = Position { x: 9.999998, y: 10.000002 };

    assert_eq!(actual, expected);
}

#[test]
fn test_get_vertices_data() {
    let vertice = Vertice(Position { x: 50.0, y: 25.0 }, Color::Black);
    let input_size = Size { width: 100.0, height: 50.0 };
    let actual = vertice.get_vertice_data(&input_size);
    let expected = _VerticeData([0.0, 0.0], [0.0, 0.0, 0.0, 1.0]);

    assert_eq!(actual, expected);
}

#[test]
fn test_get_position_neighbors() {
    let position = Position { x: 10.0, y: 50.0 };
    let input_value = 10.0;
    let actual = position.get_neighbors(input_value);
    let expected= vec![
        Position { x: 10.0, y: 40.0 },
        Position { x: 10.0, y: 60.0 },
        Position { x: 0.0, y: 50.0 },
        Position { x: 20.0, y: 50.0 },
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_partial_ord_on_position_same_positions() {
    let position1 = Position { x: 10.0, y: 10.0 };
    let position2 = Position { x: 10.0, y: 10.0 };

    assert!(position1 >= position2);
    assert!(position1 <= position2);
}

#[test]
fn test_partial_ord_on_position_same_x() {
    let position1 = Position { x: 20.0, y: 10.0 };
    let position2 = Position { x: 20.0, y: 20.0 };

    assert!(position2 >= position1);
}

#[test]
fn test_partial_ord_on_position_same_y() {
    let position1 = Position { x: 10.0, y: 10.0 };
    let position2 = Position { x: 20.0, y: 10.0 };

    assert!(position2 >= position1);
}

#[test]
fn test_partial_ord_on_position_different() {
    let position1 = Position { x: 10.0, y: 10.0 };
    let position2 = Position { x: 20.0, y: 20.0 };
    let position3 = Position { x: 30.0, y: 30.0 };

    assert!(position2 >= position1 && position2 <= position3);
}
