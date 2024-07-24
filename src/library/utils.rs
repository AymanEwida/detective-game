use crate::renderer::vertice::Position;

pub fn template_literal(string: &str, inputs: Option<&[&str]>) -> String {
    match inputs {
        Some(inputs) => {
            let mut parts = Vec::new();
        
            let mut count = 0 as usize;
        
            let string_parts: Vec<&str> = string.split("{}").collect();
        
            if string_parts.len() <= 1 {
                return string.to_string();
            }
        
            for string_part in string_parts {
                parts.push(string_part);
        
                if count < inputs.len() {
                    parts.push(inputs[count]);
                    
                    count += 1;
                }
            }
        
            let full_string = parts.join("");
        
            full_string
        },
        None => string.to_string()
    }
}

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
