use detective_game::{game::{character::*, player::Player}, renderer::vertice::Position};

#[test]
fn test_collide_on_x_axis() {
    let character = Player::new(Position { x: 1.0, y: 1.0 }, false);
    let game_object = Player::new(Position { x: 3.0, y: 1.0 }, false);
    let actual = character.collide(&game_object);
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_collide_on_y_axis() {
    let character = Player::new(Position { x: 1.0, y: 1.0 }, false);
    let game_object = Player::new(Position { x: 1.0, y: 3.0 }, false);
    let actual = character.collide(&game_object);
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_collide_on_both_axes() {
    let character = Player::new(Position { x: 1.0, y: 1.0 }, false);
    let game_object = Player::new(Position { x: 3.0, y: 3.0 }, false);
    let actual = character.collide(&game_object);
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_collide_false() {
    let character = Player::new(Position { x: 10.0, y: 10.0 }, false);
    let game_object = Player::new(Position { x: 70.0, y: 10.0 }, false);
    let actual = character.collide(&game_object);
    let expected = false;

    assert_eq!(actual, expected);
}
