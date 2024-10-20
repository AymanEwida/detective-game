use detective_game::{game::{character::{Character, Direction}, enemy::*, level::{GameObject, DEFAULT_SIZE}, wall::Wall}, renderer::{render::Size, vertice::Position}};

#[test]
fn test_find_optimal_path_same_positions() {
    let enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "5r/0 5l/0", false);
    
    let input_target_position = Position { x: 10.0, y: 10.0 };
    let input_grid = vec![vec![]];
    let actual = enemy.find_optimal_path(input_target_position, Position { x: 0.0, y: 0.0 }, input_grid);
    let expected = None; 

    assert_eq!(actual, expected);
}

#[test]
fn test_find_optimal_path_same_xs_without_obstacles() {
    let enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "5r/0 5l/0", false);
    
    let input_target_position = Position { x: 10.0, y: 40.0 };
    let input_grid = vec![vec![true, true, true],  vec![true, true, true], vec![true, true, true], vec![true, true, true], vec![true, true, true]];
    let actual = enemy.find_optimal_path(input_target_position, Position { x: 0.0, y: 0.0 }, input_grid);
    let expected = Some(vec![(3, Direction::Down, 0)]); 

    assert_eq!(actual, expected);
}

#[test]
fn test_find_optimal_path_same_xs_with_obstacles() {
    let enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "5r/0 5l/0", false);
    
    let input_target_position = Position { x: 10.0, y: 40.0 };
    let input_grid = vec![vec![true, true, true],  vec![false, true, true], vec![true, false, true], vec![true, false, true], vec![true, true, true]];
    let actual = enemy.find_optimal_path(input_target_position, Position { x: 0.0, y: 0.0 }, input_grid);
    let expected = Some(vec![(1, Direction::Right, 0), (3, Direction::Down, 0), (1, Direction::Left, 0)]); 

    assert_eq!(actual, expected);
}

#[test]
fn test_find_optimal_path_same_ys_without_obstacles() {
    let enemy = Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 30.0 }, "5r/0 5l/0", false);

    let input_target_position = Position { x: 20.0, y: 30.0 };
    let input_grid = vec![vec![true, true, true],  vec![true, true, true], vec![true, true, true], vec![true, true, true], vec![true, true, true]];
    let actual = enemy.find_optimal_path(input_target_position, Position { x: 0.0, y: 0.0 }, input_grid);
    let expected = Some(vec![(2, Direction::Right, 0)]); 

    assert_eq!(actual, expected);
}

#[test]
fn test_find_optimal_path_same_ys_with_obstacles() {
    let enemy = Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 30.0 }, "5r/0 5l/0", false);
    
    let input_target_position = Position { x: 20.0, y: 30.0 };
    let input_grid = vec![vec![true, true, true],  vec![false, true, true], vec![true, false, true], vec![true, false, true], vec![true, true, true]];
    let actual = enemy.find_optimal_path(input_target_position, Position { x: 0.0, y: 0.0 }, input_grid);
    let expected = Some(vec![(1, Direction::Down, 0), (2, Direction::Right, 0), (1, Direction::Up, 0)]); 

    assert_eq!(actual, expected);
}

#[test]
fn test_find_optimal_path_different_position_without_obstacles() {
    let enemy = Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 40.0 }, "5r/0 5l/0", false);

    let input_target_position = Position { x: 10.0, y: 0.0 };
    let input_grid = vec![vec![true, true, true],  vec![true, true, true], vec![true, true, true], vec![true, true, true], vec![true, true, true]];
    let actual = enemy.find_optimal_path(input_target_position, Position { x: 0.0, y: 0.0 }, input_grid);
    let expected = Some(vec![(4, Direction::Up, 0), (1, Direction::Right, 0)]); 

    assert_eq!(actual, expected);
}

#[test]
fn test_find_optimal_path_different_position_with_obstacles() {
    let enemy = Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 40.0 }, "5r/0 5l/0", false);
    
    let input_target_position = Position { x: 10.0, y: 0.0 };
    let input_grid = vec![vec![true, true, true],  vec![false, true, true], vec![true, false, true], vec![true, false, true], vec![true, true, true]];
    let actual = enemy.find_optimal_path(input_target_position, Position { x: 0.0, y: 0.0 }, input_grid);
    let expected = Some(vec![(2, Direction::Right, 0), (4, Direction::Up, 0), (1, Direction::Left, 0)]); 

    assert_eq!(actual, expected);
}

#[test]
fn test_get_movement_grid_one_obstacles() {
    let enemy = Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 0.0 }, "2u/0 3d/0 1u/0", false);
    
    let input_doors = &[Wall::new(Position { x: 20.0, y: 10.0 }, Size { width: DEFAULT_SIZE, height: 50.0 }, None, None)];
    let input_window_start_position = Position { x: 0.0, y: 0.0 };
    let input_window_size = Size { width: 100.0, height: 80.0 };
    let actual = enemy.get_movement_grid(input_window_start_position, input_window_size, input_doors);
    let expected = (Position { x: 0.0, y: 0.0 }, vec![
        vec![false, false, false, false, false, true, true, true, true, true],
        vec![false, false, false, false, false, true, true, true, true, true],
        vec![false, false, false, false, false, true, true, true, true, true],
        vec![false, false, false, false, false, true, true, true, true, true],
        vec![false, false, false, false, false, true, true, true, true, true],
        vec![false, false, false, false, false, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true, true, true],
    ]); 

    assert_eq!(actual, expected);
}

#[test]
fn test_get_movement_grid_two_obstacles() {
    let enemy = Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 0.0 }, "2u/0 3d/0 1u/0", false);
    
    let input_doors = &[
        Wall::new(Position { x: 20.0, y: 10.0 }, Size { width: DEFAULT_SIZE, height: 50.0 }, None, None),
        Wall::new(Position { x: 50.0, y: 10.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, None, None)
    ];
    let input_window_start_position = Position { x: 0.0, y: 0.0 };
    let input_window_size = Size { width: 100.0, height: 80.0 };
    let actual = enemy.get_movement_grid(input_window_start_position, input_window_size, input_doors);
    let expected = (Position { x: 0.0, y: 0.0 }, vec![
        vec![false, false, false, false, false, false, false, false, false, true],
        vec![false, false, false, false, false, false, false, false, false, true],
        vec![false, false, false, false, false, false, false, false, false, true],
        vec![false, false, false, false, false, false, false, false, false, true],
        vec![false, false, false, false, false, true, true, true, true, true],
        vec![false, false, false, false, false, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true, true, true],
    ]); 

    assert_eq!(actual, expected);
}

#[test]
fn test_collide_and_move_enemy_to_prev_position() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "1u/0 1d/0", false);

    let wall = Wall::new(Position { x: 60.0, y: 10.0 }, Size { width: 50.0, height: 60.0 }, None, None);

    let input_direction = Direction::Right;
    
    enemy._set_prev_position();
    enemy.move_character(input_direction, enemy.get_movement_value());
    
    if enemy.collide(&wall) {
        enemy.move_to_prev_position();
    }

    let expected_position = Position { x: 10.0, y: 10.0 };

    assert_eq!(enemy.get_position(), expected_position);
}

#[test]
fn test_enemy_is_off_window_true_on_start_x_axis() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "1u/0 1d/0", false);

    enemy.set_movement_value(20.0);

    enemy.move_character(Direction::Left, enemy.get_movement_value());

    let actual = enemy.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_enemy_is_off_window_true_on_end_x_axis() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 70.0, y: 10.0 }, "1u/0 1d/0", false);

    enemy.set_movement_value(20.0);

    enemy.move_character(Direction::Right, enemy.get_movement_value());

    let actual = enemy.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_enemy_is_off_window_true_on_start_y_axis() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "1u/0 1d/0", false);

    enemy.set_movement_value(20.0);

    enemy.move_character(Direction::Up, enemy.get_movement_value());

    let actual = enemy.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_enemy_is_off_window_true_on_end_y_axis() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 50.0 }, "1u/0 1d/0", false);

    enemy.set_movement_value(20.0);

    enemy.move_character(Direction::Down, enemy.get_movement_value());

    let actual = enemy.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_enemy_is_off_window_false() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "1u/0 1d/0", false);

    enemy.move_character(Direction::Right, enemy.get_movement_value());
    enemy.move_character(Direction::Up, enemy.get_movement_value());

    let actual = enemy.is_off_window(Size { width: 80.0, height: 60.0 });
    let expected = false;

    assert_eq!(actual, expected);
}

#[test]
fn test_enemy_is_off_border_true_on_start_x_axis() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "1u/0 1d/0", false);

    enemy.move_character(Direction::Left, enemy.get_movement_value());

    let actual = enemy.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_enemy_is_off_border_true_on_end_x_axis() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 20.0, y: 10.0 }, "1u/0 1d/0", false);

    enemy.move_character(Direction::Right, enemy.get_movement_value());

    let actual = enemy.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_enemy_is_off_border_true_on_start_y_axis() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "1u/0 1d/0", false);

    enemy.move_character(Direction::Up, enemy.get_movement_value());

    let actual = enemy.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_enemy_is_off_border_true_on_end_y_axis() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 20.0 }, "1u/0 1d/0", false);

    enemy.move_character(Direction::Down, enemy.get_movement_value());

    let actual = enemy.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn test_enemy_is_off_border_false() {
    let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "1u/0 1d/0", false);

    enemy.move_character(Direction::Right, enemy.get_movement_value());
    enemy.move_character(Direction::Down, enemy.get_movement_value());

    let actual = enemy.is_off_border(Some(Position { x: 5.0, y: 5.0 }), Size { width: 70.0, height: 80.0 });
    let expected = false;

    assert_eq!(actual, expected);
}
