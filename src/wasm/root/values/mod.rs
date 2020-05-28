mod cmyk_value;
mod color_input;
mod hex_value;
mod hsl_value;
mod hsv_value;
mod rgb_value;

use yew::{html, Html};

use cmyk_value::CmykValue;
use hex_value::HexValue;
use hsl_value::HslValue;
use hsv_value::HsvValue;
use rgb_value::RgbValue;

pub fn view() -> Html {
  html! {
    <div class="values">
      <HexValue />

      <div class="values-container">
        <RgbValue />
        <CmykValue />
        <HsvValue />
        <HslValue />
      </div>
    </div>
  }
}
