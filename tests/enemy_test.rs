use detective_game::{game::{character::Direction, enemy::*, level::DEFAULT_SIZE, wall::Wall}, renderer::{render::Size, vertice::Position}};

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
