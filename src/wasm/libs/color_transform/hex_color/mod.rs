use serde::{Deserialize, Serialize};

use crate::libs::color_transform::rgb_color::RGB;

use crate::constants::HEX_SHORT_REG_EXP;

#[derive(Serialize, Deserialize)]
pub struct Hex {
  value: String,
}

impl Hex {
  pub fn new(value: String) -> Hex {
    Hex { value }
  }

  fn value_to_hex_part(value: i32) -> String {
    let part = format!("{:X}", value);
    if part.len() == 1 {
      return format!("0{}", part);
    }

    part
  }

  pub fn from_rgb(rgb: &RGB) -> Hex {
    let r_part = Hex::value_to_hex_part(rgb.get_red() as i32);
    let g_part = Hex::value_to_hex_part(rgb.get_green() as i32);
    let b_part = Hex::value_to_hex_part(rgb.get_blue() as i32);

    let value = format!("#{}{}{}", r_part, g_part, b_part);
    Hex { value }
  }

  pub fn get_value(&self) -> String {
    self.value.to_string()
  }

  pub fn to_string(&self) -> String {
    let mut value = if HEX_SHORT_REG_EXP.is_match(&self.value) {
      HEX_SHORT_REG_EXP
        .replace_all(&self.value, "#$r$r$g$g$b$b")
        .to_string()
    } else {
      self.value.to_string()
    };

    value.make_ascii_lowercase();
    value
  }
}
