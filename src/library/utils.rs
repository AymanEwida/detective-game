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
