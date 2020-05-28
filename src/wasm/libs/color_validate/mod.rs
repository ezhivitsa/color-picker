use regex::{Regex, RegexSet};

use crate::constants::{MAX_CMYK, MAX_RGB, MIN_CMYK, MIN_RGB};

lazy_static! {
  static ref HEX: RegexSet =
    RegexSet::new(&[r"^#?([\da-fA-F]{3})$", r"^#?([\da-fA-F]{6})$"]).unwrap();
  static ref RGB: Regex = Regex::new(r"^(\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})$").unwrap();
  static ref CMYK: Regex =
    Regex::new(r"^(\d{1,3})%,\s*(\d{1,3})%,\s*(\d{1,3})%,\s*(\d{1,3})%$").unwrap();
}

pub fn is_valid_hex(hex: &str) -> bool {
  HEX.is_match(hex)
}

pub fn is_valid_rgb(rgb: &str) -> bool {
  if !RGB.is_match(rgb) {
    return false;
  }

  let mut red: i32 = -1;
  let mut green: i32 = -1;
  let mut blue: i32 = -1;

  for cap in RGB.captures_iter(&rgb) {
    red = cap[1].parse::<i32>().unwrap();
    green = cap[2].parse::<i32>().unwrap();
    blue = cap[3].parse::<i32>().unwrap();
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

  let mut cyan: i32 = -1;
  let mut magenta: i32 = -1;
  let mut yellow: i32 = -1;
  let mut key: i32 = -1;

  for cap in RGB.captures_iter(&cmyk) {
    cyan = cap[1].parse::<i32>().unwrap();
    magenta = cap[2].parse::<i32>().unwrap();
    yellow = cap[3].parse::<i32>().unwrap();
    key = cap[4].parse::<i32>().unwrap();
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
