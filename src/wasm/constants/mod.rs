use regex::Regex;

pub const MAX_H: f32 = 360.0;
pub const MAX_SVL: f32 = 100.0;
pub const MIN_HSV: f32 = 0.0;

pub const MIN_RGB: f32 = 0.0;
pub const MAX_RGB: f32 = 255.0;

pub const MIN_CMYK: f32 = 0.0;
pub const MAX_CMYK: f32 = 100.0;

lazy_static! {
  pub static ref CMYK_REG_EXP: Regex =
    Regex::new(r"^(\d{1,3})%,\s*(\d{1,3})%,\s*(\d{1,3})%,\s*(\d{1,3})%$").unwrap();
  pub static ref HEX_SHORT_REG_EXP: Regex =
    Regex::new(r"^#?(?P<r>[\dA-Fa-f])(?P<g>[\dA-Fa-f])(?P<b>[\dA-Fa-f])$").unwrap();
  pub static ref HEX_LONG_REG_EXP: Regex =
    Regex::new(r"^#?([\dA-Fa-f]{2})([\dA-Fa-f]{2})([\dA-Fa-f]{2})$").unwrap();
  pub static ref HSV_REG_EXP: Regex =
    Regex::new(r"(\d{1,3})°,\s*(\d{1,3})%,\s*(\d{1,3})%").unwrap();
  pub static ref RGB_REG_EXP: Regex = Regex::new(r"^(\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})$").unwrap();
}
