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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_literal_one_input() {
        let input_string = "My name is {}";
        let actual = template_literal(input_string, Some(&["Ayman"]));
        let expected = "My name is Ayman";

        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_template_literal_multiple_input() {
        let input_string = "My name is {}, and this is {}";
        let actual = template_literal(input_string, Some(&["Ayman", "test"]));
        let expected = "My name is Ayman, and this is test";

        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_template_literal_multiple_with_number_input() {
        let input_string = "My name is {}, and i am {} years old";
        let actual = template_literal(input_string, Some(&["Ayman", 19.to_string().as_str()]));
        let expected = "My name is Ayman, and i am 19 years old";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_template_literal_with_no_input() {
        let input_string = "My name is Adam";
        let actual = template_literal(input_string, None);
        let expected = "My name is Adam";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_template_literal_with_no_template_literal() {
        let input_string = "This a test";
        let actual = template_literal(input_string, Some(&["Name"]));
        let expected = "This a test";

        assert_eq!(actual, expected);
    }
}
