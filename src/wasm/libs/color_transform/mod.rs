use regex::Regex;

use crate::constants::{MAX_CMYK, MAX_RGB, MAX_SVL};

lazy_static! {
  static ref HEX_SHORT_REG_EXP: Regex =
    Regex::new(r"^#?(?P<r>[\dA-F])(?P<g>[\dA-F])(?P<b>[\dA-F])$").unwrap();
  static ref HEX_LONG_REG_EXP: Regex =
    Regex::new(r"^#?([\dA-F]{2})([\dA-F]{2})([\dA-F]{2})$").unwrap();
  static ref RGB_REG_EXP: Regex = Regex::new(r"^(\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})$").unwrap();
  static ref CMYK_REG_EXP: Regex =
    Regex::new(r"^(\d{1,3})%,\s*(\d{1,3})%,\s*(\d{1,3})%,\s*(\d{1,3})%$").unwrap();
  static ref HSV_REG_EXP: Regex = Regex::new(r"(\d{1,3})°,\s*(\d{1,3})%,\s*(\d{1,3})%").unwrap();
}

pub struct HSV {
  hue: f32,
  saturation: f32,
  value: f32,
}

pub struct RGB {
  red: f32,
  green: f32,
  blue: f32,
}

pub struct Hex {
  value: String,
}

pub struct CMYK {
  black: f32,
  cyan: f32,
  magenta: f32,
  yellow: f32,
}

pub struct HSL {
  hue: f32,
  saturation: f32,
  lightness: f32,
}

pub struct Color {
  hex: Hex,
  hsv: HSV,
  rgb: RGB,
  cmyk: CMYK,
  hsl: HSL,
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
    let r_norm = rgb.red / MAX_RGB;
    let g_norm = rgb.green / MAX_RGB;
    let b_norm = rgb.blue / MAX_RGB;

    // h, s, v = hue, saturation, value
    let cmax = r_norm.max(g_norm.max(b_norm)); // maximum of r, g, b
    let cmin = r_norm.min(g_norm.min(b_norm)); // minimum of r, g, b
    let diff = cmax - cmin; // diff of cmax and cmin.

    let mut h: f32 = -1.0;
    let mut s: f32 = -1.0;

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
    if cmax == 0.0 {
      s = 0.0;
    } else {
      s = (diff / cmax) * 100.0;
    }

    // compute v
    let v: f32 = cmax * 100.0;

    HSV {
      hue: h,
      saturation: s,
      value: v,
    }
  }

  pub fn from_hsl(hsl: &HSL) -> HSV {
    let h = hsl.hue;

    let l_norm = hsl.lightness * 2.0 / MAX_SVL;
    let mut s_norm = hsl.saturation / MAX_SVL;
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

impl RGB {
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
    let c_norm: f32 = cmyk.cyan / MAX_CMYK;
    let m_norm: f32 = cmyk.magenta / MAX_CMYK;
    let y_norm: f32 = cmyk.yellow / MAX_CMYK;
    let k_norm: f32 = cmyk.black / MAX_CMYK;

    let red = (1.0 - c_norm) * (1.0 - k_norm);
    let green = (1.0 - m_norm) * (1.0 - k_norm);
    let blue = (1.0 - y_norm) * (1.0 - k_norm);

    RGB {
      red: (red * MAX_RGB).round(),
      green: (green * MAX_RGB).round(),
      blue: (blue * MAX_RGB).round(),
    }
  }

  fn values_to_rgb(r: f32, g: f32, b: f32) -> RGB {
    RGB {
      red: (r * MAX_RGB).round(),
      green: (g * MAX_RGB).round(),
      blue: (b * MAX_RGB).round(),
    }
  }

  fn from_hsv(hsv: &HSV) -> RGB {
    let s_norm = hsv.saturation / MAX_SVL;
    let v_norm = hsv.value / MAX_SVL;

    if hsv.saturation == 0.0 {
      return RGB::values_to_rgb(v_norm, v_norm, v_norm);
    }

    let h_sector: f32 = hsv.hue / 60.0; // sector 0 to 5
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

    return RGB {
      red: red as f32,
      green: green as f32,
      blue: blue as f32
    };
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
    let r_part = Hex::value_to_hex_part(rgb.red as i32);
    let g_part = Hex::value_to_hex_part(rgb.green as i32);
    let b_part = Hex::value_to_hex_part(rgb.blue as i32);

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
    let r_norm = rgb.red / MAX_RGB;
    let g_norm = rgb.green / MAX_RGB;
    let b_norm = rgb.blue / MAX_RGB;

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

  pub fn to_string(&self) -> String {
    format!(
      "{}%, {}%, {}%, {}%",
      self.cyan, self.magenta, self.yellow, self.black
    )
  }
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
    let s_norm = hsv.saturation / MAX_SVL;
    let v_norm = hsv.value / MAX_SVL;

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
      saturation: (saturation * MAX_SVL).round(),
      lightness: (lightness * MAX_SVL).round(),
    }
  }

  pub fn to_string(&self) -> String {
    format!("{}°, {}%, {}%", self.hue, self.saturation, self.lightness)
  }
}

impl Color {
  pub fn from_hsv_values(h: f32, s: f32, v: f32) -> Color {
    let hsv = HSV::from_values(h, s, v);
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

  pub fn from_cmyk(value: String) -> Color {
    let cmyk = CMYK::new(value);
    let rgb = RGB::from_cmyk(&cmyk);
    let hex = Hex::from_rgb(&rgb);
    let hsv = HSV::from_rgb(&rgb);
    let hsl = HSL::from_hsv(&hsv);

    Color {
      hex,
      rgb,
      hsv,
      cmyk,
      hsl,
    }
  }

  pub fn from_hsv(value: String) -> Color {
    let hsv = HSV::new(value);
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

  pub fn from_hsl(value: String) -> Color {
    let hsl = HSL::new(value);
    let hsv = HSV::from_hsl(&hsl);
    let rgb = RGB::from_hsv(&hsv);
    let hex = Hex::from_rgb(&rgb);
    let cmyk = CMYK::from_rgb(&rgb);

    Color {
      hex,
      hsv,
      rgb,
      cmyk,
      hsl,
    }
  }

  pub fn get_hue(&self) -> f32 {
    self.hsv.hue
  }

  pub fn get_saturation(&self) -> f32 {
    self.hsv.saturation
  }

  pub fn get_value(&self) -> f32 {
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
