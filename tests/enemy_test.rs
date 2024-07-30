// use detective_game::{game::enemy::*, renderer::vertice::Position};

// #[test]
// fn test_move_enemy_short() {
//     let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 });

//     let input_path = "3d 1r";
//     let input_speed = None;
    
//     enemy.move_enemy(input_path, input_speed);

//     let expected_position = Position { x: 20.0, y: 40.0 };

//     assert_eq!(enemy.get_position(), expected_position);
// }

// #[test]
// fn test_move_enemy_long() {
//     let mut enemy = Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 });

//     let input_path = "3d 5r 2u 4l";
//     let input_speed = None;
    
//     enemy.move_enemy(input_path, input_speed);

//     let expected_position = Position { x: 20.0, y: 20.0 };

//     assert_eq!(enemy.get_position(), expected_position);
// }
