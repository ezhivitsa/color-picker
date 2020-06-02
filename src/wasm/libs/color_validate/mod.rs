use regex::{Regex, RegexSet};

use crate::constants::{MAX_CMYK, MAX_H, MAX_RGB, MAX_SVL, MIN_CMYK, MIN_HSV, MIN_RGB};

lazy_static! {
  static ref HEX: RegexSet =
    RegexSet::new(&[r"^#?([\da-fA-F]{3})$", r"^#?([\da-fA-F]{6})$"]).unwrap();
  static ref RGB: Regex = Regex::new(r"^(\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})$").unwrap();
  static ref CMYK: Regex =
    Regex::new(r"^(\d{1,3})%,\s*(\d{1,3})%,\s*(\d{1,3})%,\s*(\d{1,3})%$").unwrap();
  static ref HSV: Regex = Regex::new(r"(\d{1,3})°,\s*(\d{1,3})%,\s*(\d{1,3})%").unwrap();
}

pub fn is_valid_hex(hex: &str) -> bool {
  HEX.is_match(hex)
}

pub fn is_valid_rgb(rgb: &str) -> bool {
  if !RGB.is_match(rgb) {
    return false;
  }

  let mut red: f32 = -1.0;
  let mut green: f32 = -1.0;
  let mut blue: f32 = -1.0;

  for cap in RGB.captures_iter(&rgb) {
    red = cap[1].parse::<f32>().unwrap();
    green = cap[2].parse::<f32>().unwrap();
    blue = cap[3].parse::<f32>().unwrap();
  }

  return MIN_RGB <= red
    && red <= MAX_RGB
    && MIN_RGB <= green
    && green <= MAX_RGB
    && MIN_RGB <= blue
    && blue <= MAX_RGB;
}

pub fn is_valid_cmyk(cmyk: &str) -> bool {
  if !CMYK.is_match(cmyk) {
    return false;
  }

  let mut cyan: f32 = -1.0;
  let mut magenta: f32 = -1.0;
  let mut yellow: f32 = -1.0;
  let mut key: f32 = -1.0;

  for cap in CMYK.captures_iter(&cmyk) {
    cyan = cap[1].parse::<f32>().unwrap();
    magenta = cap[2].parse::<f32>().unwrap();
    yellow = cap[3].parse::<f32>().unwrap();
    key = cap[4].parse::<f32>().unwrap();
  }

  return MIN_CMYK <= cyan
    && cyan <= MAX_CMYK
    && MIN_CMYK <= magenta
    && magenta <= MAX_CMYK
    && MIN_CMYK <= yellow
    && yellow <= MAX_CMYK
    && MIN_CMYK <= key
    && key <= MAX_CMYK;
}

pub fn is_valid_hsv(hsv: &str) -> bool {
  if !HSV.is_match(hsv) {
    return false;
  }

  let mut hue: f32 = -1.0;
  let mut saturation: f32 = -1.0;
  let mut value: f32 = -1.0;

  for cap in HSV.captures_iter(&hsv) {
    hue = cap[1].parse::<f32>().unwrap();
    saturation = cap[2].parse::<f32>().unwrap();
    value = cap[3].parse::<f32>().unwrap();
  }

  return MIN_HSV <= hue
    && hue <= MAX_H
    && MIN_HSV <= saturation
    && saturation <= MAX_SVL
    && MIN_HSV <= value
    && value <= MAX_SVL;
}
