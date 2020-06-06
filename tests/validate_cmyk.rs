use color_picker::libs::color_validate;
use color_picker::libs::color_transform::Color;

#[test]
fn should_validate_cmyk_string() {
  let valid = color_validate::is_valid_cmyk("50%,  0%, 50%,100%");
  assert_eq!(valid, true);
}

#[test]
fn should_determine_invalid_cmyk_string() {
  let valid = color_validate::is_valid_cmyk("50%,0%, 50%,101%");
  assert_eq!(valid, false);
}

#[test]
fn should_calculate_rgb_from_cmyk() {
  let color = Color::from_cmyk(String::from("40%, 30%, 20%, 10%"));
  assert_eq!(color.rgb_value(), "138, 161, 184");
}
