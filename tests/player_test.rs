use detective_game::{game::{character::{Character, Direction}, level::{GameObject, ObjectLevel, ObjectLevelType}, player::*}, renderer::{render::Size, vertice::Position}};

#[test]
fn test_move_player_up() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 });

    let input_direction = Direction::Up;
    let input_speed = None;
    
    player.move_player(input_direction, input_speed);

    let expected_prev_position = Some(Position { x: 10.0, y: 10.0 });
    let expected_position = Position { x: 10.0, y: 0.0 };

    assert_eq!(player.get_prev_position(), expected_prev_position);
    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_down() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 });

    let input_direction = Direction::Down;
    let input_speed = None;
    
    player.move_player(input_direction, input_speed);
    player.move_player(input_direction, input_speed);

    let expected_prev_position = Some(Position { x: 10.0, y: 20.0 });
    let expected_position = Position { x: 10.0, y: 30.0 };

    assert_eq!(player.get_prev_position(), expected_prev_position);
    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_left() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 });

    let input_direction = Direction::Left;
    let input_speed = None;
    
    player.move_player(input_direction, input_speed);

    let expected_prev_position = Some(Position { x: 10.0, y: 10.0 });
    let expected_position = Position { x: 0.0, y: 10.0 };

    assert_eq!(player.get_prev_position(), expected_prev_position);
    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_move_player_right() {
    let mut player = Player::new(Position { x: 10.0, y: 10.0 });

    let input_direction = Direction::Right;
    let input_speed = None;
    
    player.move_player(input_direction, input_speed);
    player.move_player(input_direction, input_speed);

    let expected_prev_position = Some(Position { x: 20.0, y: 10.0 });
    let expected_position = Position { x: 30.0, y: 10.0 };

    assert_eq!(player.get_prev_position(), expected_prev_position);
    assert_eq!(player.get_position(), expected_position);
}

#[test]
fn test_collide_and_move_player_to_prev_position() {
    let wall = ObjectLevel::new(ObjectLevelType::Wall, Position { x: 60.0, y: 10.0 }, Size { width: 50.0, height: 60.0 });
    let mut player = Player::new(Position { x: 10.0, y: 10.0 });

    let input_direction = Direction::Right;
    let input_speed = None;
    
    player.move_player(input_direction, input_speed);
    
    if player.collide(&wall) {
        player.move_player_to_prev_position();
    }

    let expected_position = Position { x: 10.0, y: 10.0 };

    assert_eq!(player.get_position(), expected_position);
}
