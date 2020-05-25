use yew::{
  Component,
  ComponentLink,
  Html,
  html,
  ShouldRender,
  Bridge,
  Bridged
};

use crate::agents::current_color_agent::{
  CurrentColorAgent,
  Response
};

pub enum Msg {
  CurrentColorMessage(Response)
}

pub struct ColorSlider {
  color: String,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
}

impl Component for ColorSlider {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> ColorSlider {
    let callback = link.callback(Msg::CurrentColorMessage);
    let _producer = CurrentColorAgent::bridge(callback);

    ColorSlider {
      color: "".to_string(),
      _producer
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::CurrentColorMessage(response) => {
        self.color = response.top_right_corner;
        true
      }
    }
  }

  fn view(&self) -> Html {
    html! {
      <div class="slider">
        <div class="slider__hue" />
        <div
          class="slider__selector"
          style={format!("background-color: {}", self.color)}
        />
      </div>
    }
  }
}
