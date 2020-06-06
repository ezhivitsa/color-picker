use serde::{Deserialize, Serialize};

use crate::constants::{HSV_REG_EXP, MAX_SVL};
use crate::libs::color_transform::hsv_color::HSV;

#[derive(Serialize, Deserialize)]
pub struct HSL {
  hue: f32,
  saturation: f32,
  lightness: f32,
}

impl HSL {
  pub fn new(hsl: String) -> HSL {
    let mut hue: f32 = 0.0;
    let mut saturation: f32 = 0.0;
    let mut lightness: f32 = 0.0;

    for cap in HSV_REG_EXP.captures_iter(&hsl) {
      hue = cap[1].parse::<f32>().unwrap();
      saturation = cap[2].parse::<f32>().unwrap();
      lightness = cap[3].parse::<f32>().unwrap();
    }

    HSL {
      hue,
      saturation,
      lightness,
    }
  }

  pub fn from_hsv(hsv: &HSV) -> HSL {
    let s_norm = hsv.get_saturation() / MAX_SVL;
    let v_norm = hsv.get_value() / MAX_SVL;

    let lightness: f32 = (2.0 - s_norm) * v_norm / 2.0;

    let mut saturation: f32 = s_norm;

    if lightness != 0.0 {
      if lightness == 1.0 {
        saturation = 0.0;
      } else if lightness < 0.5 {
        saturation = s_norm * v_norm / (lightness * 2.0);
      } else {
        saturation = s_norm * v_norm / (2.0 - lightness * 2.0);
      }
    }

    HSL {
      hue: hsv.get_hue(),
      saturation: (saturation * MAX_SVL).round(),
      lightness: (lightness * MAX_SVL).round(),
    }
  }

  pub fn get_hue(&self) -> f32 {
    self.hue
  }

  pub fn get_saturation(&self) -> f32 {
    self.saturation
  }

  pub fn get_lightness(&self) -> f32 {
    self.lightness
  }

  pub fn to_string(&self) -> String {
    format!("{}°, {}%, {}%", self.hue, self.saturation, self.lightness)
  }
}
