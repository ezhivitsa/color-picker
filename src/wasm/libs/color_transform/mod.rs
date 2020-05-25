use regex::Regex;

use crate::constants::{
  MAX_S,
  MAX_V
};

lazy_static! {
  static ref HEX_SHORT: Regex = Regex::new(r"^#?(?P<r>[\dA-F])(?P<g>[\dA-F])(?P<b>[\dA-F])$").unwrap();
  static ref HEX_LONG: Regex = Regex::new(r"^#?([\dA-F]{2})([\dA-F]{2})([\dA-F]{2})$").unwrap();
}

pub struct HSV {
  hue: i32,
  saturation: i32,
  value: i32,
}

struct RGB {
  red: i32,
  green: i32,
  blue: i32
}

struct Hex {
  value: String
}

pub struct Color {
  hex: Hex,
  pub hsv: HSV,
  rgb: RGB
}

impl HSV {
  // h = [0,360], s = [0,100], v = [0,100]
  fn new(h: i32, s: i32, v: i32) -> HSV {
    HSV {
      hue: h,
      saturation: s,
      value: v
    }
  }

  fn from_rgb(rgb: &RGB) -> HSV {
    HSV {
      hue: 0,
      saturation: 0,
      value: 0
    }
  }

  pub fn get_hue(&self) -> i32 {
    self.hue
  }
}

impl RGB {
  fn values_to_rgb(r: f32, g: f32, b: f32) -> RGB {
    RGB {
      red: (r * 255.0).round() as i32,
      green: (g * 255.0).round() as i32,
      blue: (b * 255.0).round() as i32
    }
  }

  fn from_hsv(hsv: &HSV) -> RGB {
    let s_norm = hsv.saturation as f32 / MAX_S as f32;
    let v_norm = hsv.value as f32 / MAX_V as f32;

    if hsv.saturation == 0 {
      return RGB::values_to_rgb(v_norm, v_norm, v_norm);
    }

    let h_sector: f32 = hsv.hue as f32 / 60.0; // sector 0 to 5
    let i = h_sector.floor();
    let f = h_sector - i;			// factorial part of h
    let p: f32 = v_norm * (1.0 - s_norm);
    let q: f32 = v_norm * (1.0 - s_norm * f);
    let t: f32 = v_norm * (1.0 - s_norm * (1.0 - f));
    
    if i == 0.0 {
      return RGB::values_to_rgb(v_norm, t, p);
    } else if i == 1.0 {
      return RGB::values_to_rgb(q, v_norm, p);
    } else if i == 2.0 {
      return RGB::values_to_rgb(p, v_norm, t);
    } else if i == 3.0 {
      return RGB::values_to_rgb(p, q, v_norm);
    } else if i == 4.0 {
      return RGB::values_to_rgb(t, p, v_norm);
    } else {
      return RGB::values_to_rgb(v_norm, p, q);
    }
  }

  fn from_hex(hex: &Hex) -> RGB {
    let mut hex_value = if HEX_SHORT.is_match(&hex.value) {
      HEX_SHORT
        .replace_all(&hex.value, "$r$r$g$g$b$b")
        .to_string()
    } else {
      hex.value
        .to_string()
    };
    hex_value.make_ascii_lowercase();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for cap in HEX_LONG.captures_iter(&hex_value) {
      red = i32::from_str_radix(&cap[1], 16).unwrap();
      green = i32::from_str_radix(&cap[2], 16).unwrap();
      blue = i32::from_str_radix(&cap[3], 16).unwrap();
    }

    return RGB {
      red,
      green,
      blue
    }
  }
}

impl Hex {
  fn new(value: String) -> Hex {
    Hex {
      value
    }
  }

  fn value_to_hex_part(value: i32) -> String {
    let part = format!("{:X}", value);
    if part.len() == 1 {
      return format!("0{}", part)
    }

    part
  }

  fn from_rgb(rgb: &RGB) -> Hex {
    let r_part = Hex::value_to_hex_part(rgb.red);
    let g_part = Hex::value_to_hex_part(rgb.green);
    let b_part = Hex::value_to_hex_part(rgb.blue);

    let value = format!("#{}{}{}", r_part, g_part, b_part);
    Hex {
      value
    }
  }
}

impl Color {
  pub fn from_hsv(h: i32, s: i32, v: i32) -> Color {
    let hsv = HSV::new(h, s, v);
    let rgb = RGB::from_hsv(&hsv);
    let hex = Hex::from_rgb(&rgb);
 
    Color {
      hex,
      hsv,
      rgb
    }
  }

  pub fn from_hex(value: String) -> Color {
    let hex = Hex::new(value);
    let rgb = RGB::from_hex(&hex);
    let hsv = HSV::from_rgb(&rgb);

    Color {
      hex,
      rgb,
      hsv
    }
  }

  pub fn hex_value(&self) -> String {
    let mut value = self.hex.value.to_string();
    value.make_ascii_lowercase();
    value
  }
}
