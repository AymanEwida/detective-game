use crate::{game::character::Direction, renderer::{render::Size, vertice::Position}};

pub fn length_of_line(start: &Position, end: &Position) -> f32 {
    ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt()
}

pub fn calc_control_point(start: &Position, end: &Position) -> Position {
    let x_middle = (start.x + end.x) / 2.0;
    let y_middle = start.y.abs() + end.y.abs();

    Position { x: x_middle, y: y_middle }
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

pub fn convert_path(path: &str) -> Vec<(u32, Direction)> {
    let path = path.to_lowercase();

    path.split(' ').map(| move_path | {
        let mut chars = move_path.chars();

        let move_number = chars.next().unwrap().to_string().parse::<u32>().unwrap();

        let move_direction = match chars.next().unwrap() {
            'u' => Direction::Up,
            'd' => Direction::Down,
            'l' => Direction::Left,
            'r' => Direction::Right,
            _ => Direction::Down
        };

        (move_number, move_direction)
    }).collect()
}