use detective_game::library::utils::*;

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
