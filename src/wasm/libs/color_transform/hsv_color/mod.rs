use crate::constants::{HSV_REG_EXP, MAX_RGB, MAX_SVL};
use crate::libs::color_transform::hsl_color::HSL;
use crate::libs::color_transform::rgb_color::RGB;

pub struct HSV {
  hue: f32,
  saturation: f32,
  value: f32,
}

impl HSV {
  pub fn new(hsv: String) -> HSV {
    let mut hue: f32 = 0.0;
    let mut saturation: f32 = 0.0;
    let mut value: f32 = 0.0;

    for cap in HSV_REG_EXP.captures_iter(&hsv) {
      hue = cap[1].parse::<f32>().unwrap();
      saturation = cap[2].parse::<f32>().unwrap();
      value = cap[3].parse::<f32>().unwrap();
    }

    HSV {
      hue,
      saturation,
      value,
    }
  }

  // h = [0,360], s = [0,100], v = [0,100]
  pub fn from_values(h: f32, s: f32, v: f32) -> HSV {
    HSV {
      hue: h,
      saturation: s,
      value: v,
    }
  }

  pub fn from_rgb(rgb: &RGB) -> HSV {
    let r_norm = rgb.get_red() / MAX_RGB;
    let g_norm = rgb.get_green() / MAX_RGB;
    let b_norm = rgb.get_blue() / MAX_RGB;

    // h, s, v = hue, saturation, value
    let cmax = r_norm.max(g_norm.max(b_norm)); // maximum of r, g, b
    let cmin = r_norm.min(g_norm.min(b_norm)); // minimum of r, g, b
    let diff = cmax - cmin; // diff of cmax and cmin.

    let mut h: f32 = -1.0;

    // if cmax and cmax are equal then h = 0
    if cmax == cmin {
      h = 0.0;
    } else if cmax == r_norm {
      // if cmax equal r then compute h
      h = (60.0 * ((g_norm - b_norm) / diff) + 360.0).round() % 360.0;
    } else if cmax == g_norm {
      // if cmax equal g then compute h
      h = (60.0 * ((b_norm - r_norm) / diff) + 120.0).round() % 360.0;
    } else if cmax == b_norm {
      // if cmax equal b then compute h
      h = (60.0 * ((r_norm - g_norm) / diff) + 240.0).round() % 360.0;
    }

    // if cmax equal zero
    let s = if cmax == 0.0 {
      0.0
    } else {
      (diff / cmax) * 100.0
    };

    // compute v
    let v: f32 = cmax * 100.0;

    HSV {
      hue: h,
      saturation: s,
      value: v,
    }
  }

  pub fn from_hsl(hsl: &HSL) -> HSV {
    let h = hsl.get_hue();

    let l_norm = hsl.get_lightness() * 2.0 / MAX_SVL;
    let mut s_norm = hsl.get_saturation() / MAX_SVL;
    if l_norm <= 1.0 {
      s_norm *= l_norm;
    } else {
      s_norm *= 2.0 - l_norm;
    }

    let v = (l_norm + s_norm) / 2.0;
    let s = (2.0 * s_norm) / (l_norm + s_norm);

    HSV {
      hue: h,
      saturation: (s * MAX_SVL).round(),
      value: (v * MAX_SVL).round(),
    }
  }

  pub fn get_hue(&self) -> f32 {
    self.hue
  }

  pub fn get_saturation(&self) -> f32 {
    self.saturation
  }

  pub fn get_value(&self) -> f32 {
    self.value
  }

  pub fn to_string(&self) -> String {
    format!("{}°, {}%, {}%", self.hue, self.saturation, self.value)
  }
}
