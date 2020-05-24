mod hex_value;

use yew::{
  Html,
  html
};

use hex_value::HexValue;

pub fn view() -> Html {
  html! {
    <div class="values">
      <HexValue />
    </div>
  }
}
