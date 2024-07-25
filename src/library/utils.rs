use crate::renderer::{render::Size, vertice::Position};

pub fn length_of_line(start: &Position, end: &Position) -> f32 {
    if start.x == end.x {
        return end.y - start.y;
    } else if start.y == end.y {
        return end.x - start.x;
    } else {
        ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt()
    }
}

pub fn calc_mid_point(start: &Position, end: &Position) -> Position {
    let x_middle = (start.x + end.x) / 2.0;
    let y_middle = (start.y + end.y) / 2.0;

    Position { x: x_middle, y: y_middle }
}

pub fn convert_coordinates(coordinate: Position, size: Size) -> Position {
    let half_width = size.width as f32 / 2.0;
    let half_height = size.height as f32 / 2.0;

    let new_x = ((coordinate.x - half_width) / half_width).abs();
    let new_y = ((coordinate.y - half_height) / half_height).abs();

    if coordinate.x < half_width {
        if coordinate.y < half_height {
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
        if coordinate.y < half_height {
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
