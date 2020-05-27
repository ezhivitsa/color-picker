use regex::{Regex, RegexSet};

use crate::constants::{MAX_RGB, MIN_RGB};

pub fn is_valid_hex(hex: &str) -> bool {
  lazy_static! {
    static ref HEX: RegexSet =
      RegexSet::new(&[r"^#?([\da-fA-F]{3})$", r"^#?([\da-fA-F]{6})$"]).unwrap();
  }

  HEX.is_match(hex)
}

pub fn is_valid_rgb(rgb: &str) -> bool {
  lazy_static! {
    static ref RGB: Regex = Regex::new(r"^(\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})$").unwrap();
  }

  if !RGB.is_match(rgb) {
    return false;
  }

  let mut red: i32 = -1;
  let mut green: i32 = -1; 
  let mut blue: i32 = -1; 

  for cap in RGB.captures_iter(&rgb) {
    red = i32::from_str_radix(&cap[1], 16).unwrap();
    green = i32::from_str_radix(&cap[2], 16).unwrap();
    blue = i32::from_str_radix(&cap[3], 16).unwrap();
  }

  return MIN_RGB <= red && red <= MAX_RGB &&
    MIN_RGB <= green && green <= MAX_RGB &&
    MIN_RGB <= blue && blue <= MAX_RGB;
}
