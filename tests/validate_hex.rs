use color_picker::libs::color_transform::Color;
use color_picker::libs::color_validate;

#[test]
fn should_validate_hex_string() {
  let valid = color_validate::is_valid_hex("#ABC123");
  assert_eq!(valid, true);
}

#[test]
fn should_determine_invalid_hex_string() {
  let valid = color_validate::is_valid_hex("#ABT123");
  assert_eq!(valid, false);
}

#[test]
fn should_calculate_rgb_from_hex() {
  let color = Color::from_hex(String::from("123456"));
  assert_eq!(color.rgb_value(), "18, 52, 86");
}
