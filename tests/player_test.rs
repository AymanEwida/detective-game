use detective_game::{game::{character::{Character, Direction}, level::{GameObject, ObjectLevel, ObjectLevelType}, player::*}, renderer::{render::Size, vertice::Position}};

#[test]
fn test_move_player_up() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    let input_direction = Direction::Up;
    
    player.move_player(input_direction);

    let expected_prev_position = Some(Position { x: 10.0, y: 10.0 });
    let expected_position = Position { x: 10.0, y: 0.0 };

    assert_eq!(player.get_prev_position(), expected_prev_position);
    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_down() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    let input_direction = Direction::Down;
    
    player.move_player(input_direction);
    player.move_player(input_direction);

    let expected_prev_position = Some(Position { x: 10.0, y: 20.0 });
    let expected_position = Position { x: 10.0, y: 30.0 };

    assert_eq!(player.get_prev_position(), expected_prev_position);
    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_left() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    let input_direction = Direction::Left;
    
    player.move_player(input_direction);

    let expected_prev_position = Some(Position { x: 10.0, y: 10.0 });
    let expected_position = Position { x: 0.0, y: 10.0 };

    assert_eq!(player.get_prev_position(), expected_prev_position);
    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_right() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    let input_direction = Direction::Right;
    
    player.move_player(input_direction);
    player.move_player(input_direction);

    let expected_prev_position = Some(Position { x: 20.0, y: 10.0 });
    let expected_position = Position { x: 30.0, y: 10.0 };

    assert_eq!(player.get_prev_position(), expected_prev_position);
    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_collide_and_move_player_to_prev_position() {
    let wall = ObjectLevel::new(ObjectLevelType::Wall, Position { x: 60.0, y: 10.0 }, Size { width: 50.0, height: 60.0 }, false, None, None);
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    let input_direction = Direction::Right;
    
    player.move_player(input_direction);
    
    if player.collide(&wall) {
        player.move_to_prev_position();
    }

    let expected_position = Position { x: 10.0, y: 10.0 };

    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_to_new_position() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    let input_new_position = Position { x: 20.0, y: 10.0 };
    
    player.move_to(input_new_position, false);

    let expected_position = Position { x: 20.0, y: 10.0 };

    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_player_is_off_window_true_on_start_x_axis() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    player.set_movement_value(20.0);

    player.move_player(Direction::Left);

    let actual = player.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_player_is_off_window_true_on_end_x_axis() {
    let mut player = Player::new(Position { x: 70.0, y: 10.0 }, false);

    player.set_movement_value(20.0);

    player.move_player(Direction::Right);

    let actual = player.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_player_is_off_window_true_on_start_y_axis() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    player.set_movement_value(20.0);

    player.move_player(Direction::Up);

    let actual = player.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_player_is_off_window_true_on_end_y_axis() {
    let mut player = Player::new(Position { x: 10.0, y: 50.0 }, false);

    player.set_movement_value(20.0);

    player.move_player(Direction::Down);

    let actual = player.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_player_is_off_window_false() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    player.move_player(Direction::Right);
    player.move_player(Direction::Up);

    let actual = player.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = false;

    assert_eq!(actual, expected);
}

#[test]
fn test_player_is_off_border_true_on_start_x_axis() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    player.move_player(Direction::Left);

    let actual = player.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_player_is_off_border_true_on_end_x_axis() {
    let mut player = Player::new(Position { x: 20.0, y: 10.0 }, false);

    player.move_player(Direction::Right);

    let actual = player.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_player_is_off_border_true_on_start_y_axis() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    player.move_player(Direction::Up);

    let actual = player.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_player_is_off_border_true_on_end_y_axis() {
    let mut player = Player::new(Position { x: 10.0, y: 20.0 }, false);

    player.move_player(Direction::Down);

    let actual = player.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_player_is_off_border_false() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, false);

    player.move_player(Direction::Right);
    player.move_player(Direction::Down);

    let actual = player.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = false;

    assert_eq!(actual, expected);
}
