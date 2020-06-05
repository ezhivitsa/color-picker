use crate::libs::color_transform::cmyk_color::CMYK;
use crate::libs::color_transform::hex_color::Hex;
use crate::libs::color_transform::hsv_color::HSV;

use crate::constants::{
  HEX_LONG_REG_EXP, HEX_SHORT_REG_EXP, MAX_CMYK, MAX_RGB, MAX_SVL, RGB_REG_EXP,
};

pub struct RGB {
  red: f32,
  green: f32,
  blue: f32,
}

impl RGB {
  fn values_to_rgb(r: f32, g: f32, b: f32) -> RGB {
    RGB {
      red: (r * MAX_RGB).round(),
      green: (g * MAX_RGB).round(),
      blue: (b * MAX_RGB).round(),
    }
  }

  pub fn new(value: String) -> RGB {
    let mut red: f32 = 0.0;
    let mut green: f32 = 0.0;
    let mut blue: f32 = 0.0;

    for cap in RGB_REG_EXP.captures_iter(&value) {
      red = cap[1].parse::<f32>().unwrap();
      green = cap[2].parse::<f32>().unwrap();
      blue = cap[3].parse::<f32>().unwrap();
    }

    RGB { red, green, blue }
  }

  pub fn from_cmyk(cmyk: &CMYK) -> RGB {
    let c_norm: f32 = cmyk.get_cyan() / MAX_CMYK;
    let m_norm: f32 = cmyk.get_magenta() / MAX_CMYK;
    let y_norm: f32 = cmyk.get_yellow() / MAX_CMYK;
    let k_norm: f32 = cmyk.get_black() / MAX_CMYK;

    let red = (1.0 - c_norm) * (1.0 - k_norm);
    let green = (1.0 - m_norm) * (1.0 - k_norm);
    let blue = (1.0 - y_norm) * (1.0 - k_norm);

    RGB {
      red: (red * MAX_RGB).round(),
      green: (green * MAX_RGB).round(),
      blue: (blue * MAX_RGB).round(),
    }
  }

  pub fn from_hsv(hsv: &HSV) -> RGB {
    let s_norm = hsv.get_saturation() / MAX_SVL;
    let v_norm = hsv.get_value() / MAX_SVL;

    if hsv.get_saturation() == 0.0 {
      return RGB::values_to_rgb(v_norm, v_norm, v_norm);
    }

    let h_sector: f32 = hsv.get_hue() / 60.0; // sector 0 to 5
    let i = h_sector.floor();
    let f = h_sector - i; // factorial part of h
    let p: f32 = v_norm * (1.0 - s_norm);
    let q: f32 = v_norm * (1.0 - s_norm * f);
    let t: f32 = v_norm * (1.0 - s_norm * (1.0 - f));

    if i == 0.0 || i == 6.0 {
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

  pub fn from_hex(hex: &Hex) -> RGB {
    let mut hex_value = if HEX_SHORT_REG_EXP.is_match(&hex.get_value()) {
      HEX_SHORT_REG_EXP
        .replace_all(&hex.get_value(), "$r$r$g$g$b$b")
        .to_string()
    } else {
      hex.get_value().to_string()
    };
    hex_value.make_ascii_lowercase();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for cap in HEX_LONG_REG_EXP.captures_iter(&hex_value) {
      red = i32::from_str_radix(&cap[1], 16).unwrap();
      green = i32::from_str_radix(&cap[2], 16).unwrap();
      blue = i32::from_str_radix(&cap[3], 16).unwrap();
    }

    return RGB {
      red: red as f32,
      green: green as f32,
      blue: blue as f32,
    };
  }

  pub fn get_red(&self) -> f32 {
    self.red
  }

  pub fn get_green(&self) -> f32 {
    self.green
  }

  pub fn get_blue(&self) -> f32 {
    self.blue
  }

  pub fn to_string(&self) -> String {
    format!("{}, {}, {}", self.red, self.green, self.blue)
  }

  pub fn to_color_string(&self) -> String {
    format!("rgb({})", self.to_string())
  }
}
