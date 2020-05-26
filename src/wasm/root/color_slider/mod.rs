use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

use crate::agents::current_color_agent::{CurrentColorAgent, Response};

use crate::constants::MAX_H;

pub enum Msg {
  CurrentColorMessage(Response),
}

pub struct ColorSlider {
  color: String,
  hue: i32,
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
      hue: 0,
      _producer,
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::CurrentColorMessage(response) => {
        self.color = response.top_right_corner;
        self.hue = response.hue;
        true
      }
    }
  }

  fn view(&self) -> Html {
    let left = self.hue as f32 / MAX_H * 100.0;

    html! {
      <div class="slider">
        <div class="slider__hue" />
        <div
          class="slider__selector"
          style={format!("background-color: {}; left: {}%;", self.color, left)}
        />
      </div>
    }
  }
}
