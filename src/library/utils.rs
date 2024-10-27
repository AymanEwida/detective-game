use std::{f32::consts::PI, fs, io::{Error, ErrorKind}, path::Path};

use rand::Rng;

use crate::{game::character::Direction, renderer::{render::Size, vertice::Position}};

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

    let challenges_vec: Vec<&str> = content.split("\r\n").collect();

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


pub fn round_position_to_full_numbers(position: Position, value: f32) -> Position {
    let decimal_round_position = Position { x: position.x.round(), y: position.y.round() };

    let rounded_x = (decimal_round_position.x / value).floor() * value;
    let rounded_y = (decimal_round_position.y / value).floor() * value;

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