use color_picker::libs::color_validate;
use color_picker::libs::color_transform::Color;

#[test]
fn should_validate_rgb_string() {
  let valid = color_validate::is_valid_rgb("255,  0,100");
  assert_eq!(valid, true);
}

#[test]
fn should_determine_invalid_rgb_string() {
  let valid = color_validate::is_valid_rgb("256,0,0");
  assert_eq!(valid, false);
}

#[test]
fn should_calculate_hex_from_rgb() {
  let color = Color::from_rgb(String::from("12, 34, 56"));
  assert_eq!(color.hex_value(), "#0c2238");
}
