use detective_game::{library::utils::*, renderer::{render::Size, vertice::Position}};

#[test]
fn test_length_of_line_same_x_coordinates() {
    let input_start = Position { x: 5.0, y: -2.0 };
    let input_end = Position { x: 5.0, y: 4.0 };
    let actual = length_of_line(&input_start, &input_end);
    let expected = 6 as f32;

    assert_eq!(actual, expected);
}

#[test]
fn test_length_of_line_same_y_coordinates() {
    let input_start = Position { x: 0.0, y: 1.0 };
    let input_end = Position { x: 8.0, y: 1.0 };
    let actual = length_of_line(&input_start, &input_end);
    let expected = 8 as f32;

    assert_eq!(actual, expected);
}

#[test]
fn test_length_of_line_different_coordinates() {
    let input_start = Position { x: 1.0, y: 4.0 };
    let input_end = Position { x: 5.0, y: 7.0 };
    let actual = length_of_line(&input_start, &input_end);
    let expected = 5 as f32;

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_same_x_coordinates() {
    let input_start = Position { x: 5.0, y: -2.0 };
    let input_end = Position { x: 5.0, y: 4.0 };
    let actual = calc_mid_point(&input_start, &input_end);
    let expected = Position { x: 5.0, y: 1.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_same_y_coordinates() {
    let input_start = Position { x: 0.0, y: 1.0 };
    let input_end = Position { x: 8.0, y: 1.0 };
    let actual = calc_mid_point(&input_start, &input_end);
    let expected = Position { x: 4.0, y: 1.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_different_coordinates() {
    let input_start = Position { x: 1.0, y: 4.0 };
    let input_end = Position { x: 5.0, y: 7.0 };
    let actual = calc_mid_point(&input_start, &input_end);
    let expected = Position { x: 3.0, y: 5.5 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_start() {
    let input_coordinate = Position { x: 0.0, y: 0.0 };
    let input_size = Size { width: 8, height: 6 };
    let actual = convert_coordinates(input_coordinate, input_size);
    let expected = Position { x: -1.0, y: 1.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_middle() {
    let input_coordinate = Position { x: 4.0, y: 3.0 };
    let input_size = Size { width: 8, height: 6 };
    let actual = convert_coordinates(input_coordinate, input_size);
    let expected = Position { x: 0.0, y: 0.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_random() {
    let input_coordinate = Position { x: 6.0, y: 1.5 };
    let input_size = Size { width: 8, height: 6 };
    let actual = convert_coordinates(input_coordinate, input_size);
    let expected = Position { x: 0.5, y: 0.5 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_whole() {
    let input_coordinate = Position { x: 8.0, y: 6.0 };
    let input_size = Size { width: 8, height: 6 };
    let actual = convert_coordinates(input_coordinate, input_size);
    let expected = Position { x: 1.0, y: -1.0 };

    assert_eq!(actual, expected);
}
