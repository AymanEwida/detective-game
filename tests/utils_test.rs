use detective_game::{game::character::Direction, library::{constants::HALF_PI, utils::*}, renderer::{render::Size, vertice::Position}};

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
fn test_calc_control_point_same_x_coordinates() {
    let input_start = Position { x: 5.0, y: -2.0 };
    let input_end = Position { x: 5.0, y: 4.0 };
    let actual = calc_control_point(&input_start, &input_end);
    let expected = Position { x: 5.0, y: 6.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_control_point_same_y_coordinates() {
    let input_start = Position { x: 0.0, y: 1.0 };
    let input_end = Position { x: 8.0, y: 1.0 };
    let actual = calc_control_point(&input_start, &input_end);
    let expected = Position { x: 4.0, y: 2.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_control_point_different_coordinates() {
    let input_start = Position { x: 1.0, y: 4.0 };
    let input_end = Position { x: 5.0, y: 7.0 };
    let actual = calc_control_point(&input_start, &input_end);
    let expected = Position { x: 3.0, y: 11.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_start() {
    let input_coordinate = Position { x: 0.0, y: 0.0 };
    let input_size = Size { width: 8.0, height: 6.0 };
    let actual = convert_coordinates(input_coordinate, &input_size);
    let expected = Position { x: -1.0, y: 1.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_middle() {
    let input_coordinate = Position { x: 4.0, y: 3.0 };
    let input_size = Size { width: 8.0, height: 6.0 };
    let actual = convert_coordinates(input_coordinate, &input_size);
    let expected = Position { x: 0.0, y: 0.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_on_y_axis() {
    let input_coordinate = Position { x: 0.0, y: 3.0 };
    let input_size = Size { width: 8.0, height: 6.0 };
    let actual = convert_coordinates(input_coordinate, &input_size);
    let expected = Position { x: -1.0, y: 0.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_on_x_axis() {
    let input_coordinate = Position { x: 4.0, y: 0.0 };
    let input_size = Size { width: 8.0, height: 6.0 };
    let actual = convert_coordinates(input_coordinate, &input_size);
    let expected = Position { x: 0.0, y: 1.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_random() {
    let input_coordinate = Position { x: 6.0, y: 1.5 };
    let input_size = Size { width: 8.0, height: 6.0 };
    let actual = convert_coordinates(input_coordinate, &input_size);
    let expected = Position { x: 0.5, y: 0.5 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_coordinates_whole() {
    let input_coordinate = Position { x: 8.0, y: 6.0 };
    let input_size = Size { width: 8.0, height: 6.0 };
    let actual = convert_coordinates(input_coordinate, &input_size);
    let expected = Position { x: 1.0, y: -1.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_size() {
    let input_object_size = Size { width: 40.0, height: 15.0 };
    let input_window_size = Size { width: 80.0, height: 60.0 };
    let actual = convert_size(input_object_size, &input_window_size);
    let expected = Size { width: 1.0, height: 0.5 };

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_path_short() {
    let input = "2d/0 2u/0";
    let actual = convert_path(input);
    let expected = vec![(2, Direction::Down, 0), (2, Direction::Up, 0)];

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_path_long() {
    let input = "2r/0 1d/0 3r/1000 3l/0 1u/0 2l/2000";
    let actual = convert_path(input);
    let expected = vec![(2, Direction::Right, 0), (1, Direction::Down, 0), (3, Direction::Right, 1000), (3, Direction::Left, 0), (1, Direction::Up, 0), (2, Direction::Left, 2000)];

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_angle_to_radians_test1() {
    let input = 90.0;
    let actual = convert_angle_to_radians(input);
    let expected = HALF_PI;

    assert_eq!(actual, expected);
}

#[test]
fn test_convert_angle_to_radians_test2() {
    let input = 45.0;
    let actual = convert_angle_to_radians(input);
    let expected = HALF_PI / 2.0;

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_position_of_triangle_test1() {
    let input_first_point = Position { x: 10.0, y: 50.0 };
    let input_second_point = Position { x: 50.0, y: 50.0  };
    let input_third_point = Position { x: 30.0, y: 20.0 };

    let actual = calc_mid_point_position_of_triangle(input_first_point, input_second_point, input_third_point);
    let expected = Position { x: 30.0, y: 40.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_position_of_triangle_test2() {
    let input_first_point = Position { x: 50.0, y: 20.0 };
    let input_second_point = Position { x: 50.0, y: 50.0  };
    let input_third_point = Position { x: 20.0, y: 30.0 };

    let actual = calc_mid_point_position_of_triangle(input_first_point, input_second_point, input_third_point);
    let expected = Position { x: 40.0, y: 33.333332 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_position_of_quadrilateral_shape_test1() {
    let input_top_left = Position { x: 10.0, y: 10.0 };
    let input_size = Size { width: 50.0, height: 50.0  };

    let actual = calc_mid_point_position_of_quadrilateral_shape(&input_top_left, &input_size);
    let expected = Position { x: 35.0, y: -15.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_position_of_quadrilateral_shape_test2() {
    let input_top_left = Position { x: 20.0, y: 35.0 };
    let input_size = Size { width: 60.0, height: 50.0  };

    let actual = calc_mid_point_position_of_quadrilateral_shape(&input_top_left, &input_size);
    let expected = Position { x: 50.0, y: 10.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_same_y() {
    let input_start = Position { x: 10.0, y: 10.0 };
    let input_end = Position { x: 50.0, y: 10.0  };

    let actual = calc_mid_point(&input_start, &input_end);
    let expected = Position { x: 30.0, y: 10.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_same_x() {
    let input_start = Position { x: 50.0, y: 10.0 };
    let input_end = Position { x: 50.0, y: -10.0  };

    let actual = calc_mid_point(&input_start, &input_end);
    let expected = Position { x: 50.0, y: 0.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_calc_mid_point_different() {
    let input_start = Position { x: -30.0, y: -20.0 };
    let input_end = Position { x: 30.0, y: 20.0  };

    let actual = calc_mid_point(&input_start, &input_end);
    let expected = Position { x: 0.0, y: 0.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_create_translate_on_x_axis_positive() {
    let input_translate = Position { x: 40.0, y: 0.0 };
    let input_window_size = Size { width: 80.0, height: 60.0  };

    let actual = create_translate(input_translate, &input_window_size);
    let expected = Position { x: 1.0, y: 0.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_create_translate_on_x_axis_negative() {
    let input_translate = Position { x: -20.0, y: 0.0 };
    let input_window_size = Size { width: 80.0, height: 60.0  };

    let actual = create_translate(input_translate, &input_window_size);
    let expected = Position { x: -0.5, y: 0.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_create_translate_on_y_axis_positive() {
    let input_translate = Position { x: 0.0, y: 30.0 };
    let input_window_size = Size { width: 80.0, height: 60.0  };

    let actual = create_translate(input_translate, &input_window_size);
    let expected = Position { x: 0.0, y: -1.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_create_translate_on_y_axis_negative() {
    let input_translate = Position { x: 0.0, y: -15.0 };
    let input_window_size = Size { width: 80.0, height: 60.0  };

    let actual = create_translate(input_translate, &input_window_size);
    let expected = Position { x: 0.0, y: 0.5 };

    assert_eq!(actual, expected);
}

#[test]
fn test_create_translate_on_both_axis() {
    let input_translate = Position { x: 80.0, y: 30.0 };
    let input_window_size = Size { width: 80.0, height: 60.0  };

    let actual = create_translate(input_translate, &input_window_size);
    let expected = Position { x: 2.0, y: -1.0 };

    assert_eq!(actual, expected);
}

#[test]
fn test_absolute_f32() {
    let input = 1.0;
    let actual = absolute_f32(input);
    let expected = 1.0;

    assert_eq!(actual, expected);

    let input = 0.0;
    let actual = absolute_f32(input);
    let expected = 0.0;

    assert_eq!(actual, expected);

    let input = -1.0;
    let actual = absolute_f32(input);
    let expected = 1.0;

    assert_eq!(actual, expected);
}
