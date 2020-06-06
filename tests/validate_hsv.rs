use color_picker::libs::color_validate;
use color_picker::libs::color_transform::Color;

#[test]
fn should_validate_hsv_string() {
  let valid = color_validate::is_valid_hsv("200°,30%,  70%");
  assert_eq!(valid, true);
}

#[test]
fn should_determine_invalid_hsv_string() {
  let valid = color_validate::is_valid_hsv("361°, 10%, 10%");
  assert_eq!(valid, false);
}

#[test]
fn should_calculate_rgb_from_hsl_string() {
  let color = Color::from_hsv(String::from("250°, 100%, 50%"));
  assert_eq!(color.rgb_value(), "21, 0, 128");
}

#[test]
fn should_calculate_rgb_from_hsl_values() {
  let color = Color::from_hsv_values(250.0, 100.0, 50.0);
  assert_eq!(color.rgb_value(), "21, 0, 128");
}
