use crate::constants::{
  CMYK_REG_EXP, HEX_LONG_REG_EXP, HEX_SHORT_REG_EXP, HSV_REG_EXP, MAX_CMYK, MAX_H, MAX_RGB,
  MAX_SVL, MIN_CMYK, MIN_HSV, MIN_RGB, RGB_REG_EXP,
};

pub fn is_valid_hex(hex: &str) -> bool {
  HEX_SHORT_REG_EXP.is_match(hex) || HEX_LONG_REG_EXP.is_match(hex)
}

pub fn is_valid_rgb(rgb: &str) -> bool {
  if !RGB_REG_EXP.is_match(rgb) {
    return false;
  }

  let mut red: f32 = -1.0;
  let mut green: f32 = -1.0;
  let mut blue: f32 = -1.0;

  for cap in RGB_REG_EXP.captures_iter(&rgb) {
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
  if !CMYK_REG_EXP.is_match(cmyk) {
    return false;
  }

  let mut cyan: f32 = -1.0;
  let mut magenta: f32 = -1.0;
  let mut yellow: f32 = -1.0;
  let mut key: f32 = -1.0;

  for cap in CMYK_REG_EXP.captures_iter(&cmyk) {
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
  if !HSV_REG_EXP.is_match(hsv) {
    return false;
  }

  let mut hue: f32 = -1.0;
  let mut saturation: f32 = -1.0;
  let mut value: f32 = -1.0;

  for cap in HSV_REG_EXP.captures_iter(&hsv) {
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
