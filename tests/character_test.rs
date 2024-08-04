use detective_game::{game::character::*, renderer::{render::Size, vertice::Position}};

#[test]
fn test_collide_on_x_axis() {
    let character1 = Character::new(Position { x: 1.0, y: 1.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let character2 = Character::new(Position { x: 3.0, y: 1.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let actual = character1.collide(&character2);
    let expected = true;

    assert_eq!(actual, expected);
}

// #[test]
// fn test_collide_on_same_y() {
//     let character1 = Character::new(Position { x: 1.0, y: 1.0 }, Size { width: 3.0, height: 4.0 }, "test");
//     let character2 = Character::new(Position { x: 4.0, y: 1.0 }, Size { width: 3.0, height: 4.0 }, "test");
//     let actual = character1.collide(&character2);
//     let expected = true;

//     assert_eq!(actual, expected);
// }

#[test]
fn test_collide_on_y_axis() {
    let character1 = Character::new(Position { x: 1.0, y: 1.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let character2 = Character::new(Position { x: 1.0, y: 3.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let actual = character1.collide(&character2);
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_collide_on_same_x() {
    let character1 = Character::new(Position { x: 1.0, y: 1.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let character2 = Character::new(Position { x: 1.0, y: 5.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let actual = character1.collide(&character2);
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_collide_on_both_axes() {
    let character1 = Character::new(Position { x: 1.0, y: 1.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let character2 = Character::new(Position { x: 3.0, y: 3.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let actual = character1.collide(&character2);
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_collide_false() {
    let character1 = Character::new(Position { x: 1.0, y: 1.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let character2 = Character::new(Position { x: 5.0, y: 1.0 }, Size { width: 3.0, height: 4.0 }, "test");
    let actual = character1.collide(&character2);
    let expected = false;

    assert_eq!(actual, expected);
}
