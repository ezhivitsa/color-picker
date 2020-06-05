pub struct Texts<'a> {
  pub title: &'a str,
  pub cmyk: &'a str,
  pub hex: &'a str,
  pub hsl: &'a str,
  pub hsv: &'a str,
  pub rgb: &'a str,
}

pub const TEXTS: Texts<'static> = Texts {
  title: "Color picker",
  cmyk: "CMYK",
  hex: "HEX",
  hsl: "HSL",
  hsv: "HSV",
  rgb: "RGB",
};
