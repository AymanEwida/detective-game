use std::{f32::consts::PI, fs, io::{Error, ErrorKind}, path::Path};

use rand::Rng;

use crate::{game::{character::Direction, door::Door, level::{EndStartPositions, GameObject}, level_object::LevelObject, wall::Wall}, renderer::{render::Size, vertice::Position}};

use super::constants::GAME_ASSETS_DIR;

pub type PathVec = Vec<(u32, Direction, u64)>;

pub fn length_of_line(start: &Position, end: &Position) -> f32 {
    ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt()
}

pub fn calc_mid_point(start: &Position, end: &Position) -> Position {
    let x_middle = (start.x + end.x) / 2.0;
    let y_middle = (start.y + end.y) / 2.0;

    Position { x: x_middle, y: y_middle }
}

pub fn calc_control_point(start: &Position, end: &Position) -> Position {
    let control_x = (start.x + end.x) / 2.0;
    let control_y = start.y.abs() + end.y.abs();

    Position { x: control_x, y: control_y }
}

pub fn convert_coordinates(coordinate: Position, size: &Size) -> Position {
    let half_width = size.width / 2.0;
    let half_height = size.height / 2.0;

    let new_x = ((coordinate.x - half_width) / half_width).abs();
    let new_y = ((coordinate.y - half_height) / half_height).abs();

    if coordinate.x < half_width {
        if coordinate.y <= half_height {
            Position {
                x: new_x * -1.0,
                y: new_y
            }
        } else {
            Position {
                x: new_x * -1.0,
                y: new_y * -1.0
            }
        }
    } else {
        if coordinate.y <= half_height {
            Position {
                x: new_x,
                y: new_y
            }
        } else {
            Position {
                x: new_x,
                y: new_y * -1.0
            }
        }
    }
}

pub fn convert_size(object_size: Size, window: &Size) -> Size {
    Size { width: (object_size.width * 2.0) / window.width, height: (object_size.height * 2.0) / window.height }
}

pub fn convert_path(path: &str) -> PathVec {
    let full_path = path.to_lowercase();

    full_path.split(' ').map(| full_move_path | {
        let move_path_and_wait_time: Vec<&str> = full_move_path.split('/').collect();

        let move_path = move_path_and_wait_time[0];
        let move_number = move_path[0..move_path.len()-1].parse::<u32>().unwrap_or(0);

        let move_direction = match move_path[move_path.len()-1..].to_lowercase().as_ref() {
            "u" => Direction::Up,
            "d" => Direction::Down,
            "l" => Direction::Left,
            "r" => Direction::Right,
            _ => Direction::Down
        };

        let wait_time = move_path_and_wait_time[move_path_and_wait_time.len()-1].parse::<u64>().unwrap_or(0);

        (move_number, move_direction, wait_time)
    }).collect()
}

pub fn get_moves_number(moves_path: PathVec) -> u32 {
    let mut sum = 0;

    for (moves_number, ..) in moves_path.iter() {
        sum += *moves_number;
    }

    sum
}

pub fn sum_direction_length_from_path(path: &str, direction: Direction, speed: f32) -> f32 {
    let direction = match direction {
        Direction::Left => 'l',
        Direction::Right => 'r',
        Direction::Up => 'u',
        Direction::Down => 'd' 
    };

    let mut steps = String::new();
    let mut count = 0;

    for ch in path.chars() {
        if ch == ' ' {
            steps = String::new();

            continue;
        }

        if ch >= '0' && ch <= '9' {
            steps.push(ch); 
        } else if ch == direction {
            for i in 0..steps.len() {
                let num = steps[i..i+1]
                    .chars()
                    .next()
                    .unwrap()
                    .to_ascii_lowercase() as i32 - 48;

                count += 10_i32.pow((steps.len() - 1 - i) as u32) * num;
            }
        }
    }

    count as f32 * speed
}

pub fn convert_angle_to_radians(angle: f32) -> f32 {
    angle * (PI / 180.0)
}

pub fn calc_mid_point_position_of_triangle(first_point_position: Position, second_point_position: Position, third_point_position: Position) -> Position {
    Position {
        x: (first_point_position.x + second_point_position.x + third_point_position.x) / 3.0,
        y: (first_point_position.y + second_point_position.y + third_point_position.y) / 3.0 
    }
}

pub fn calc_mid_point_position_of_quadrilateral_shape(top_left: &Position, size: &Size) -> Position {
    Position {
        x: top_left.x + (size.width / 2.0),
        y: top_left.y - (size.height / 2.0) 
    }
}

pub fn create_translate(translate: Position, window_size: &Size) -> Position {
    Position {
        x: (translate.x / window_size.width) * 2.0,
        y: (translate.y / window_size.height) * -2.0
    }
}

pub fn absolute_f32(num: f32) -> f32 {
    if num >= 0.0 {
        return num;
    }

    num * -1.0
}

pub fn get_level_challenges(level: u8) -> Result<Vec<String>, std::io::Error> {
    let level_challenges_file_path = format!("./{}challenges/level{}.txt", GAME_ASSETS_DIR, level);
    let content = fs::read_to_string(Path::new(&level_challenges_file_path))?; 

    let mut challenges_vec: Vec<&str> = content.split("\n").collect();
    if content.contains("\r\n") {
        challenges_vec = content.split("\r\n").collect();
    }

    if challenges_vec.len() == 0 {
        return Err(Error::new(ErrorKind::InvalidData, format!("There is no challenges in the file, path is: {}", level_challenges_file_path)));
    }

    let mut challenges = Vec::new();
    while challenges.len() < 3 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..challenges_vec.len());

        let challenge = challenges_vec[idx].to_string();

        if !challenges.contains(&challenge) {
            challenges.push(challenge);
        }
    }

    Ok(challenges)
}

pub fn calc_equidistant_points(apex: Position, angle: f32, line_length: f32, angle_direction: Direction) -> (Position, Position, Position) {
    assert!(angle > 0.0 && angle < 180.0, "angle must be bigger than 0.0 and smaller than 180.0");
   
    let angle = convert_angle_to_radians(angle);

    let middle_line_length = angle.cos() * line_length;
    let middle_part_line_length = (line_length.powi(2) - middle_line_length.powi(2)).sqrt(); 

    let middle_point;
    let top_point;
    let bottom_point;        

    match angle_direction {
        Direction::Right => {
            middle_point = Position { x: apex.x + middle_line_length, y: apex.y };
            top_point = Position { x: middle_point.x, y: middle_point.y + middle_part_line_length };
            bottom_point = Position { x: middle_point.x, y: middle_point.y - middle_part_line_length };
        },

        Direction::Left => {
            middle_point = Position { x: apex.x - middle_line_length, y: apex.y };
            top_point = Position { x: middle_point.x, y: middle_point.y + middle_part_line_length };
            bottom_point = Position { x: middle_point.x, y: middle_point.y - middle_part_line_length };
        },

        Direction::Down => {
            middle_point = Position { x: apex.x, y: apex.y + middle_line_length };
            top_point = Position { x: middle_point.x + middle_part_line_length, y: middle_point.y};
            bottom_point = Position { x: middle_point.x - middle_part_line_length, y: middle_point.y };
        },

        Direction::Up => {
            middle_point = Position { x: apex.x, y: apex.y - middle_line_length };    
            top_point = Position { x: middle_point.x + middle_part_line_length, y: middle_point.y};
            bottom_point = Position { x: middle_point.x - middle_part_line_length, y: middle_point.y };
        },
    }

    (top_point, bottom_point, apex)
}

pub fn get_heuristic_score(a: &Position, b: &Position, value: f32) -> f32 {
    (absolute_f32(a.x - b.x) + absolute_f32(a.y - b.y)) / value
}


pub fn round_position_to_full_numbers(position: Position, value: f32, is_middle_towrds_up_x_axis: bool, is_middle_towrds_up_y_axis: bool) -> Position {
    let decimal_round_position = Position { x: position.x.round(), y: position.y.round() };

    let rounded_x = if is_middle_towrds_up_x_axis {
        (decimal_round_position.x / value).round() * value
    } else {
        if (decimal_round_position.x % value) <= (value / 2.0).round() {
            (decimal_round_position.x / value).floor() * value
        } else {
            (decimal_round_position.x / value).ceil() * value
        }
    };

    let rounded_y = if is_middle_towrds_up_y_axis {
        (decimal_round_position.y / value).round() * value
    } else {
        if (decimal_round_position.y % value) <= (value / 2.0).round() {
            (decimal_round_position.y / value).floor() * value
        } else {
            (decimal_round_position.y / value).ceil() * value
        }
    };

    Position { x: rounded_x, y: rounded_y }
}

pub fn get_estimated_position(position: &Position, steps: u32, direction: Direction, value: f32) -> Position {
    let distance = steps as f32 * value;

    match direction {
        Direction::Left => {
            Position {
                x: position.x - distance,
                ..*position
            }
        },

        Direction::Right => {
            Position {
                x: position.x + distance,
                ..*position
            }
        },

        Direction::Up => {
            Position {
                y: position.y - distance,
                ..*position
            }
        },

        Direction::Down => {
            Position {
                y: position.y + distance,
                ..*position
            }
        }
    }
}

pub fn is_position_in_border(border_start: &Position, border_end: &Position, position: &Position) -> (bool, bool) {
    (
        position.x >= border_start.x && position.x <= border_end.x,
        position.y >= border_start.y && position.y <= border_end.y
    )
}

pub fn calculate_calc_position(start_position: Position, size: Size, value: f32) -> EndStartPositions {
    let start = round_position_to_full_numbers(start_position, value, true, true);
    let end = round_position_to_full_numbers(start + size, value, true, true);

    (start, end)
}

pub fn check_point_in_triangle(point: &Position, first: &Position, second: &Position, apex: &Position) -> bool {
    let first_to_second = (point.x - first.x) * (second.y - first.y) - (point.y - first.y) * (second.x - first.x);
    let second_to_apex = (point.x - second.x) * (apex.y - second.y) - (point.y - second.y) * (apex.x - second.x);
    let apex_to_first = (point.x - apex.x) * (first.y - apex.y) - (point.y - apex.y) * (first.x - apex.x);

    (first_to_second.signum() == second_to_apex.signum()) && (first_to_second.signum() == apex_to_first.signum()) && (second_to_apex.signum() == apex_to_first.signum())
}

pub fn get_attached_enemy_index(attached_enemies: &Vec<(usize, Position)>, search_id: usize) -> i32 {
    let mut found_idx = -1;

    for (idx, (attached_enemy_id, ..)) in attached_enemies.iter().enumerate() {
        let attached_enemy_id = *attached_enemy_id;

        if attached_enemy_id == search_id {
            found_idx = idx as i32;
        }
    }

    found_idx
}

pub fn simple_object_detect_check<'a>(player_calc_position: EndStartPositions, other_calc_position: EndStartPositions, objects: &[impl LevelObject<'a>]) -> bool {
    let (player_start, player_end) = player_calc_position; 
    let (other_start, other_end) = other_calc_position; 

    for object in objects {
        let (object_start, object_end) = object.get_calc_position();

        let is_between_x_axis = (
            (player_end.y <= object_start.y && other_start.y >= object_end.y)
            || (other_end.y <= object_start.y && player_start.y >= object_end.y)
        ) && (
            (player_start.x >= object_start.x && other_end.x <= object_end.x)
            || (player_end.x <= object_end.x && other_start.x >= object_start.x)
        );

        let is_between_y_axis = (
            (player_end.x <= object_start.x && other_start.x >= object_end.x)
            || (other_end.x <= object_start.x && player_start.x >= object_end.x)
        ) && (
            (player_start.y >= object_start.y && other_end.y <= object_end.y)
            || (player_end.y <= object_end.y && other_start.y >= object_start.y)
        );

        if is_between_x_axis || is_between_y_axis {
            return true;
        }
    }

    false
}

pub fn bfs_object_detect_check<'a>(player_calc_position: EndStartPositions, other_calc_position: EndStartPositions, walls: &[Wall<'a>], doors: &[Door<'a>], value: f32) -> bool {
    let (player_start, player_end) = player_calc_position; 
    let (other_start, ..) = other_calc_position; 

    let distance = (absolute_f32(player_start.x - other_start.x), absolute_f32(player_start.y - other_start.y));
    let dirction: (Direction, Direction);

    if player_start.x < other_start.x {
        if player_start.y < other_start.y {
            dirction = (Direction::Right, Direction::Down);
        } else {
            dirction = (Direction::Right, Direction::Up);
        }
    } else {
        if player_start.y < other_start.y {
            dirction = (Direction::Left, Direction::Down);
        } else {
            dirction = (Direction::Left, Direction::Up);
        }
    }

    if distance.0 == 0.0 && distance.1 == 0.0 {
        return true;
    }

    let mut check_positions = vec![];

    let mut x_count = 0.0;

    while x_count <= distance.0 {
        let x_pos: (f32, f32);
        if dirction.0 == Direction::Right {
            x_pos = (player_start.x + x_count, player_end.x + x_count);
        } else {
            x_pos = (player_start.x - x_count, player_end.x + x_count);
        }

        let mut y_count = 0.0;

        while y_count <= distance.1 {
            let y_pos: (f32, f32);
            if dirction.1 == Direction::Down {
                y_pos = (player_start.y + y_count, player_end.y + y_count);

                check_positions.push((Position { x: x_pos.0, y: y_pos.1 }, Position { x: x_pos.1, y: y_pos.1 }));
            } else {
                y_pos = (player_start.y - y_count, player_end.y - y_count);

                check_positions.push((Position { x: x_pos.0, y: y_pos.0 }, Position { x: x_pos.1, y: y_pos.0 }));
            }
            
            if dirction.0 == Direction::Right {
                check_positions.push((Position { x: x_pos.1, y: y_pos.0 }, Position { x: x_pos.1, y: y_pos.1 }));
            } else {
                check_positions.push((Position { x: x_pos.0, y: y_pos.0 }, Position { x: x_pos.0, y: y_pos.1 }));
            }

            y_count += value;
        }

        x_count += value;
    }
    
    let mut player_body_check_positions = (false, false);

    let is_in_object = | position: &Position, obj_start: &Position, obj_end: &Position | {
        (position.x >= obj_start.x && position.x <= obj_end.x)
            && (position.y >= obj_start.y && position.y <= obj_end.y)
    };

    for wall in walls {
        let (wall_start, wall_end) = wall.get_calc_position();

        for (start_check_position, end_check_position) in check_positions.iter() {
            if !player_body_check_positions.0 && is_in_object(start_check_position, &wall_start, &wall_end) {
                player_body_check_positions.0 = true;
            }

            if !player_body_check_positions.1 && is_in_object(end_check_position, &wall_start, &wall_end) {
                player_body_check_positions.1 = true;
            }
        }
    }
    
    for door in doors {
        let (door_start, door_end) = door.get_calc_position();

        for (start_check_position, end_check_position) in check_positions.iter() {
            if is_in_object(start_check_position, &door_start, &door_end) {
                player_body_check_positions.0 = door.is_closed();
            }

            if is_in_object(end_check_position, &door_start, &door_end) {
                player_body_check_positions.1 = door.is_closed();
            }
        }
    }

    if player_body_check_positions.0 && player_body_check_positions.1 {
        return true;
    }
    
    false
} 
