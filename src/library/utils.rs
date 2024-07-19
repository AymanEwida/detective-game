pub fn template_literal(string: &str, inputs: &[&str]) -> String {
    let mut parts = Vec::new();
    
    let mut i = 0_usize;

    let _: Vec<_> = string.split("{}").map(| part | {
        parts.push(part);

        if i < inputs.len() {
            parts.push(inputs[i]);
            
            i += 1;
        }
    }).collect();

    let full_string = parts.join("");

    full_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_literal_one_input() {
        let input_string = "My name is {}";
        let actual = template_literal(input_string, &["Ayman"]);
        let expected = "My name is Ayman";

        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_template_literal_multiple_input() {
        let input_string = "My name is {}, and this is {}";
        let actual = template_literal(input_string, &["Ayman", "test"]);
        let expected = "My name is Ayman, and this is test";

        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_template_literal_multiple_with_number_input() {
        let input_string = "My name is {}, and i am {} years old";
        let actual = template_literal(input_string, &["Ayman", 19.to_string().as_str()]);
        let expected = "My name is Ayman, and i am 19 years old";

        assert_eq!(actual, expected);
    }
}
