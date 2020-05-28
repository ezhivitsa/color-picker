use regex::Regex;

use crate::constants::{MAX_CMYK, MAX_L, MAX_RGB, MAX_S, MAX_V};

lazy_static! {
  static ref HEX_SHORT_REG_EXP: Regex =
    Regex::new(r"^#?(?P<r>[\dA-F])(?P<g>[\dA-F])(?P<b>[\dA-F])$").unwrap();
  static ref HEX_LONG_REG_EXP: Regex =
    Regex::new(r"^#?([\dA-F]{2})([\dA-F]{2})([\dA-F]{2})$").unwrap();
  static ref RGB_REG_EXP: Regex = Regex::new(r"^(\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})$").unwrap();
}

pub struct HSV {
  hue: i32,
  saturation: i32,
  value: i32,
}

struct RGB {
  red: i32,
  green: i32,
  blue: i32,
}

struct Hex {
  value: String,
}

struct CMYK {
  black: i32,
  cyan: i32,
  magenta: i32,
  yellow: i32,
}

struct HSL {
  hue: i32,
  saturation: i32,
  lightness: i32,
}

pub struct Color {
  hex: Hex,
  hsv: HSV,
  rgb: RGB,
  cmyk: CMYK,
  hsl: HSL,
}

impl HSV {
  // h = [0,360], s = [0,100], v = [0,100]
  fn new(h: i32, s: i32, v: i32) -> HSV {
    HSV {
      hue: h,
      saturation: s,
      value: v,
    }
  }

  fn from_rgb(rgb: &RGB) -> HSV {
    let r_norm = rgb.red as f32 / MAX_RGB as f32;
    let g_norm = rgb.green as f32 / MAX_RGB as f32;
    let b_norm = rgb.blue as f32 / MAX_RGB as f32;

    // h, s, v = hue, saturation, value
    let cmax = r_norm.max(g_norm.max(b_norm)); // maximum of r, g, b
    let cmin = r_norm.min(g_norm.min(b_norm)); // minimum of r, g, b
    let diff = cmax - cmin; // diff of cmax and cmin.

    let mut h: i32 = -1;
    let mut s: i32 = -1;

    // if cmax and cmax are equal then h = 0
    if cmax == cmin {
      h = 0;
    } else if cmax == r_norm {
      // if cmax equal r then compute h
      h = (60.0 * ((g_norm - b_norm) / diff) + 360.0).round() as i32 % 360;
    } else if cmax == g_norm {
      // if cmax equal g then compute h
      h = (60.0 * ((b_norm - r_norm) / diff) + 120.0).round() as i32 % 360;
    } else if cmax == b_norm {
      // if cmax equal b then compute h
      h = (60.0 * ((r_norm - g_norm) / diff) + 240.0).round() as i32 % 360;
    }

    // if cmax equal zero
    if cmax == 0.0 {
      s = 0;
    } else {
      s = ((diff / cmax) * 100.0) as i32;
    }

    // compute v
    let v: i32 = (cmax * 100.0) as i32;

    HSV {
      hue: h,
      saturation: s,
      value: v,
    }
  }

  pub fn get_hue(&self) -> i32 {
    self.hue
  }

  pub fn get_saturation(&self) -> i32 {
    self.saturation
  }

  pub fn get_value(&self) -> i32 {
    self.value
  }

  pub fn to_string(&self) -> String {
    format!("{}°, {}%, {}%", self.hue, self.saturation, self.value)
  }
}

impl RGB {
  pub fn new(value: String) -> RGB {
    let mut red: i32 = 0;
    let mut green: i32 = 0;
    let mut blue: i32 = 0;

    for cap in RGB_REG_EXP.captures_iter(&value) {
      red = cap[1].parse::<i32>().unwrap();
      green = cap[2].parse::<i32>().unwrap();
      blue = cap[3].parse::<i32>().unwrap();
    }

    RGB { red, green, blue }
  }

  fn values_to_rgb(r: f32, g: f32, b: f32) -> RGB {
    RGB {
      red: (r * 255.0).round() as i32,
      green: (g * 255.0).round() as i32,
      blue: (b * 255.0).round() as i32,
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
    let f = h_sector - i; // factorial part of h
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
    let mut hex_value = if HEX_SHORT_REG_EXP.is_match(&hex.value) {
      HEX_SHORT_REG_EXP
        .replace_all(&hex.value, "$r$r$g$g$b$b")
        .to_string()
    } else {
      hex.value.to_string()
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

    return RGB { red, green, blue };
  }

  pub fn to_string(&self) -> String {
    format!("{}, {}, {}", self.red, self.green, self.blue)
  }
}

impl Hex {
  fn new(value: String) -> Hex {
    Hex { value }
  }

  fn value_to_hex_part(value: i32) -> String {
    let part = format!("{:X}", value);
    if part.len() == 1 {
      return format!("0{}", part);
    }

    part
  }

  fn from_rgb(rgb: &RGB) -> Hex {
    let r_part = Hex::value_to_hex_part(rgb.red);
    let g_part = Hex::value_to_hex_part(rgb.green);
    let b_part = Hex::value_to_hex_part(rgb.blue);

    let value = format!("#{}{}{}", r_part, g_part, b_part);
    Hex { value }
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

impl CMYK {
  pub fn from_rgb(rgb: &RGB) -> CMYK {
    let r_norm = rgb.red as f32 / MAX_RGB as f32;
    let g_norm = rgb.green as f32 / MAX_RGB as f32;
    let b_norm = rgb.blue as f32 / MAX_RGB as f32;

    let black_norm = 1.0 - (r_norm.max(g_norm.max(b_norm)));
    let cyan_norm = (1.0 - r_norm - black_norm) / (1.0 - black_norm);
    let magenta_norm = (1.0 - g_norm - black_norm) / (1.0 - black_norm);
    let yellow_norm = (1.0 - b_norm - black_norm) / (1.0 - black_norm);

    let black = (black_norm * MAX_CMYK as f32).round() as i32;
    let cyan = (cyan_norm * MAX_CMYK as f32).round() as i32;
    let magenta = (magenta_norm * MAX_CMYK as f32).round() as i32;
    let yellow = (yellow_norm * MAX_CMYK as f32).round() as i32;

    CMYK {
      black,
      cyan,
      magenta,
      yellow,
    }
  }

  pub fn to_string(&self) -> String {
    format!(
      "{}%, {}%, {}%, {}%",
      self.cyan, self.magenta, self.yellow, self.black
    )
  }
}

impl HSL {
  pub fn from_hsv(hsv: &HSV) -> HSL {
    let s_norm = hsv.saturation as f32 / MAX_S as f32;
    let v_norm = hsv.value as f32 / MAX_V as f32;

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
      hue: hsv.hue,
      saturation: (saturation * MAX_S as f32).round() as i32,
      lightness: (lightness * MAX_L as f32).round() as i32,
    }
  }

  pub fn to_string(&self) -> String {
    format!("{}°, {}%, {}%", self.hue, self.saturation, self.lightness)
  }
}

impl Color {
  pub fn from_hsv(h: i32, s: i32, v: i32) -> Color {
    let hsv = HSV::new(h, s, v);
    let rgb = RGB::from_hsv(&hsv);
    let hex = Hex::from_rgb(&rgb);
    let cmyk = CMYK::from_rgb(&rgb);
    let hsl = HSL::from_hsv(&hsv);

    Color {
      hex,
      hsv,
      rgb,
      cmyk,
      hsl,
    }
  }

  pub fn from_hex(value: String) -> Color {
    let hex = Hex::new(value);
    let rgb = RGB::from_hex(&hex);
    let hsv = HSV::from_rgb(&rgb);
    let cmyk = CMYK::from_rgb(&rgb);
    let hsl = HSL::from_hsv(&hsv);

    Color {
      hex,
      rgb,
      hsv,
      cmyk,
      hsl,
    }
  }

  pub fn from_rgb(value: String) -> Color {
    let rgb = RGB::new(value);
    let hex = Hex::from_rgb(&rgb);
    let hsv = HSV::from_rgb(&rgb);
    let cmyk = CMYK::from_rgb(&rgb);
    let hsl = HSL::from_hsv(&hsv);

    Color {
      hex,
      rgb,
      hsv,
      cmyk,
      hsl,
    }
  }

  pub fn from_cmyk(value: String) -> Color {}

  pub fn get_hue(&self) -> i32 {
    self.hsv.hue
  }

  pub fn get_saturation(&self) -> i32 {
    self.hsv.saturation
  }

  pub fn get_value(&self) -> i32 {
    self.hsv.value
  }

  pub fn hex_value(&self) -> String {
    self.hex.to_string()
  }

  pub fn rgb_value(&self) -> String {
    self.rgb.to_string()
  }

  pub fn cmyk_value(&self) -> String {
    self.cmyk.to_string()
  }

  pub fn hsl_value(&self) -> String {
    self.hsl.to_string()
  }

  pub fn hsv_value(&self) -> String {
    self.hsv.to_string()
  }
}
