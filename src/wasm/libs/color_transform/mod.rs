use serde::{Deserialize, Serialize};

pub mod cmyk_color;
pub mod hex_color;
pub mod hsl_color;
pub mod hsv_color;
pub mod rgb_color;

use cmyk_color::CMYK;
use hex_color::Hex;
use hsl_color::HSL;
use hsv_color::HSV;
use rgb_color::RGB;

#[derive(Serialize, Deserialize)]
pub struct Color {
  hex: Hex,
  hsv: HSV,
  rgb: RGB,
  cmyk: CMYK,
  hsl: HSL,
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
    self.hsv.get_hue()
  }

  pub fn get_saturation(&self) -> f32 {
    self.hsv.get_saturation()
  }

  pub fn get_value(&self) -> f32 {
    self.hsv.get_value()
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
