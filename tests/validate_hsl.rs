use color_picker::libs::color_transform::Color;

#[test]
fn should_calculate_rgb_from_hsl() {
  let color = Color::from_hsl(String::from("300°, 50%, 20%"));
  assert_eq!(color.rgb_value(), "77, 25, 77");
}
