use std::f32::consts::PI;

use crate::{game::character::Direction, renderer::{render::Size, vertice::Position}};

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
    let half_width = size.width as f32 / 2.0;
    let half_height = size.height as f32 / 2.0;

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

pub fn convert_path(path: &str) -> Vec<(u32, Direction, u64)> {
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
