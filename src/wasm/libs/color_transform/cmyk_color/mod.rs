use serde::{Deserialize, Serialize};

use crate::libs::color_transform::rgb_color::RGB;

use crate::constants::{CMYK_REG_EXP, MAX_CMYK, MAX_RGB};

#[derive(Serialize, Deserialize)]
pub struct CMYK {
  black: f32,
  cyan: f32,
  magenta: f32,
  yellow: f32,
}

impl CMYK {
  pub fn new(value: String) -> CMYK {
    let mut cyan: f32 = 0.0;
    let mut magenta: f32 = 0.0;
    let mut yellow: f32 = 0.0;
    let mut black: f32 = 0.0;

    for cap in CMYK_REG_EXP.captures_iter(&value) {
      cyan = cap[1].parse::<f32>().unwrap();
      magenta = cap[2].parse::<f32>().unwrap();
      yellow = cap[3].parse::<f32>().unwrap();
      black = cap[4].parse::<f32>().unwrap();
    }

    CMYK {
      cyan,
      magenta,
      yellow,
      black,
    }
  }

  pub fn from_rgb(rgb: &RGB) -> CMYK {
    let r_norm = rgb.get_red() / MAX_RGB;
    let g_norm = rgb.get_green() / MAX_RGB;
    let b_norm = rgb.get_blue() / MAX_RGB;

    let black_norm = 1.0 - (r_norm.max(g_norm.max(b_norm)));
    let cyan_norm = (1.0 - r_norm - black_norm) / (1.0 - black_norm);
    let magenta_norm = (1.0 - g_norm - black_norm) / (1.0 - black_norm);
    let yellow_norm = (1.0 - b_norm - black_norm) / (1.0 - black_norm);

    let black = (black_norm * MAX_CMYK).round();
    let cyan = (cyan_norm * MAX_CMYK).round();
    let magenta = (magenta_norm * MAX_CMYK).round();
    let yellow = (yellow_norm * MAX_CMYK).round();

    CMYK {
      black,
      cyan,
      magenta,
      yellow,
    }
  }

  pub fn get_cyan(&self) -> f32 {
    self.cyan
  }

  pub fn get_magenta(&self) -> f32 {
    self.magenta
  }

  pub fn get_yellow(&self) -> f32 {
    self.yellow
  }

  pub fn get_black(&self) -> f32 {
    self.black
  }

  pub fn to_string(&self) -> String {
    format!(
      "{}%, {}%, {}%, {}%",
      self.cyan, self.magenta, self.yellow, self.black
    )
  }
}
