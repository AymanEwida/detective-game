use detective_game::game::enemy::*;

#[test]
fn test_get_optimal_path_same_positions() {
    let input_start = Position { x: 10.0, y: 40.0 };
    let input_end = Position { x: 10.0, y: 40.0 };
    let input_speed = 10;
    let actual = get_optimal_path(&input_start, &input_end, input_speed);
    let expected = Vec::new(); 

    assert_eq!(actual, expected);
}

#[test]
fn test_get_optimal_path_same_ys_left() {
    let input_start = Position { x: 50.0, y: 40.0 };
    let input_end = Position { x: 10.0, y: 40.0 };
    let input_speed = 10;
    let actual = get_optimal_path(&input_start, &input_end, input_speed);
    let expected = vec![(4, Direction::Left, 0)]; 

    assert_eq!(actual, expected);
}

#[test]
fn test_get_optimal_path_same_ys_right() {
    let input_start = Position { x: 10.0, y: 40.0 };
    let input_end = Position { x: 30.0, y: 40.0 };
    let input_speed = 10;
    let actual = get_optimal_path(&input_start, &input_end, input_speed);
    let expected = vec![(2, Direction::Right, 0)]; 

    assert_eq!(actual, expected);
}

#[test]
fn test_get_optimal_path_same_xs_up() {
    let input_start = Position { x: 10.0, y: 40.0 };
    let input_end = Position { x: 10.0, y: 30.0 };
    let input_speed = 10;
    let actual = get_optimal_path(&input_start, &input_end, input_speed);
    let expected = vec![(1, Direction::Up, 0)]; 

    assert_eq!(actual, expected);
}

#[test]
fn test_get_optimal_path_same_xs_down() {
    let input_start = Position { x: 10.0, y: 10.0 };
    let input_end = Position { x: 10.0, y: 60.0 };
    let input_speed = 10;
    let actual = get_optimal_path(&input_start, &input_end, input_speed);
    let expected = vec![(5, Direction::Down, 0)]; 

    assert_eq!(actual, expected);
}

#[test]
fn test_get_optimal_path_different_xs_and_ys_up_left_and_right() {
    let input_speed = 10;
    
    let left_input_start = Position { x: 20.0, y: 40.0 };
    let left_input_end = Position { x: 10.0, y: 10.0 };
    let actual_left = get_optimal_path(&left_input_start, &left_input_end, input_speed);
    let expected_left = vec![(3, Direction::Up, 0), (1, Direction::Left, 0)]; 

    assert_eq!(actual_left, expected_left);

    let right_input_start = Position { x: 30.0, y: 50.0 };
    let right_input_end = Position { x: 60.0, y: 10.0 };
    let actual_right = get_optimal_path(&right_input_start, &right_input_end, input_speed);
    let expected_right = vec![(4, Direction::Up, 0), (3, Direction::Right, 0)]; 

    assert_eq!(actual_right, expected_right);
}

#[test]
fn test_get_optimal_path_different_xs_and_ys_down_left_and_right() {
    let input_speed = 10;

    let left_input_start = Position { x: 30.0, y: 10.0 };
    let left_input_end = Position { x: 10.0, y: 70.0 };
    let actual_left = get_optimal_path(&left_input_start, &left_input_end, input_speed);
    let expected_left = vec![(6, Direction::Down, 0), (2, Direction::Left, 0)]; 

    assert_eq!(actual_left, expected_left);

    let right_input_start = Position { x: 20.0, y: 20.0 };
    let right_input_end = Position { x: 60.0, y: 50.0 };
    let actual_right = get_optimal_path(&right_input_start, &right_input_end, input_speed);
    let expected_right = vec![(3, Direction::Down, 0), (4, Direction::Right, 0)]; 

    assert_eq!(actual_right, expected_right);
}