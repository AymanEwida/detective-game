use detective_game::{game::{character::Direction, player::*}, renderer::vertice::Position};

#[test]
fn test_move_player_up() {
    let mut player = Player::default();

    let input_direction = Direction::Up;
    let input_speed = None;
    
    player.move_player(input_direction, input_speed);

    let expected_position = Position { x: 10.0, y: 0.0 };

    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_down() {
    let mut player = Player::default();

    let input_direction = Direction::Down;
    let input_speed = None;
    
    player.move_player(input_direction, input_speed);

    let expected_position = Position { x: 10.0, y: 20.0 };

    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_left() {
    let mut player = Player::default();

    let input_direction = Direction::Left;
    let input_speed = None;
    
    player.move_player(input_direction, input_speed);

    let expected_position = Position { x: 0.0, y: 10.0 };

    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_right() {
    let mut player = Player::default();

    let input_direction = Direction::Right;
    let input_speed = None;
    
    player.move_player(input_direction, input_speed);

    let expected_position = Position { x: 20.0, y: 10.0 };

    assert_eq!(player.get_position(), expected_position);
}
