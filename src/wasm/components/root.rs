use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::color_pallet::ColorPallet;
use crate::components::color_slider::ColorSlider;
use crate::components::color_value::ColorValue;

use crate::components::values;

use crate::texts::TEXTS;

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
              {TEXTS.title}
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
