mod color_pallet;
mod color_slider;
mod color_value;
mod values;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use color_pallet::ColorPallet;
use color_slider::ColorSlider;
use color_value::ColorValue;

pub struct Root;

impl Component for Root {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
        <div class="color-picker">
            <h1 class="color-picker__title">
              {"Color picker"}
            </h1>

            <div class="colors">
              <ColorValue />
              <ColorPallet />
            </div>

            <ColorSlider />

            {values::view()}
        </div>
    }
  }
}
