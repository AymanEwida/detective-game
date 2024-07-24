use detective_game::{library::utils::*, renderer::vertice::Position};

#[test]
fn test_template_literal_one_input() {
    let input_string = "My name is {}";
    let actual = template_literal(input_string, Some(&["Ayman"]));
    let expected = "My name is Ayman";

    assert_eq!(actual, expected);
}

#[test]
fn test_template_literal_multiple_input() {
    let input_string = "My name is {}, and this is {}";
    let actual = template_literal(input_string, Some(&["Ayman", "test"]));
    let expected = "My name is Ayman, and this is test";

    assert_eq!(actual, expected);
}

#[test]
fn test_template_literal_multiple_with_number_input() {
    let input_string = "My name is {}, and i am {} years old";
    let actual = template_literal(input_string, Some(&["Ayman", 19.to_string().as_str()]));
    let expected = "My name is Ayman, and i am 19 years old";

    assert_eq!(actual, expected);
}

#[test]
fn test_template_literal_with_no_input() {
    let input_string = "My name is Adam";
    let actual = template_literal(input_string, None);
    let expected = "My name is Adam";

    assert_eq!(actual, expected);
}

#[test]
fn test_template_literal_with_no_template_literal() {
    let input_string = "This a test";
    let actual = template_literal(input_string, Some(&["Name"]));
    let expected = "This a test";

    assert_eq!(actual, expected);
}

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