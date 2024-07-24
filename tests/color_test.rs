use detective_game::renderer::color::*;

#[test]
fn test_get_white_in_f32() {
    let color = Color::White;
    let actual = color.get_color_in_f32();
    let expected = (1.0, 1.0, 1.0, 1.0);

    assert_eq!(actual, expected);
}

#[test]
fn test_get_black_in_f32() {
    let color = Color::Black;
    let actual = color.get_color_in_f32();
    let expected = (0.0, 0.0, 0.0, 1.0);

    assert_eq!(actual, expected);
}

#[test]
fn test_get_red_in_f32() {
    let color = Color::Red;
    let actual = color.get_color_in_f32();
    let expected = (1.0, 0.0, 0.0, 1.0);

    assert_eq!(actual, expected);
}

#[test]
fn test_get_green_in_f32() {
    let color = Color::Green;
    let actual = color.get_color_in_f32();
    let expected = (0.0, 1.0, 0.0, 1.0);

    assert_eq!(actual, expected);
}

#[test]
fn test_get_blue_in_f32() {
    let color = Color::Blue;
    let actual = color.get_color_in_f32();
    let expected = (0.0, 0.0, 1.0, 1.0);

    assert_eq!(actual, expected);
}

#[test]
fn test_get_rgb_in_f32() {
    let color = Color::RGB(50, 50, 50);
    let actual = color.get_color_in_f32();
    let expected = (0.19607843, 0.19607843, 0.19607843, 1.0);

    assert_eq!(actual, expected);
}

#[test]
fn test_get_rgba_in_f32() {
    let color = Color::RGBA(255, 255, 0, 255);
    let actual = color.get_color_in_f32();
    let expected = (1.0, 1.0, 0.0, 1.0);

    assert_eq!(actual, expected);
}

#[test]
fn test_get_vertices_white_in_f32() {
    let color = Color::White;
    let actual = color.get_vertices_color_in_f32();
    let expected = [1.0, 1.0, 1.0, 1.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_get_vertices_black_in_f32() {
    let color = Color::Black;
    let actual = color.get_vertices_color_in_f32();
    let expected = [0.0, 0.0, 0.0, 1.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_get_vertices_red_in_f32() {
    let color = Color::Red;
    let actual = color.get_vertices_color_in_f32();
    let expected = [1.0, 0.0, 0.0, 1.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_get_vertices_green_in_f32() {
    let color = Color::Green;
    let actual = color.get_vertices_color_in_f32();
    let expected = [0.0, 1.0, 0.0, 1.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_get_vertices_blue_in_f32() {
    let color = Color::Blue;
    let actual = color.get_vertices_color_in_f32();
    let expected = [0.0, 0.0, 1.0, 1.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_get_vertices_rgb_in_f32() {
    let color = Color::RGB(255, 255, 0);
    let actual = color.get_vertices_color_in_f32();
    let expected = [1.0, 1.0, 0.0, 1.0];

    assert_eq!(actual, expected);
}

#[test]
fn test_get_vertices_defalut_in_f32() {
    let color = Color::RGBA(255, 0, 255, 0);
    let actual = color.get_vertices_color_in_f32();
    let expected = [1.0, 0.0, 1.0, 0.0];

    assert_eq!(actual, expected);
}