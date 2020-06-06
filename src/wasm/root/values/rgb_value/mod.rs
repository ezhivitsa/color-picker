use crate::agents::current_color_agent::{CurrentColorAgent, Response};
use crate::agents::rgb_color_agent::{RgbColorAgent, Request};
use yew::agent::{Dispatched, Dispatcher};

use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

use crate::root::values::color_input::ColorInput;
use crate::texts::TEXTS;

pub enum Msg {
  NewMessage(Response),
  ValueChanged(String),
}

pub struct RgbValue {
  rgb_value: String,
  link: ComponentLink<RgbValue>,
  rgb_color_agent: Dispatcher<RgbColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
}

impl RgbValue {
  fn handle_value_change(&mut self, value: String) {
    self
      .rgb_color_agent
      .send(Request::RgbColorChangeMsg(value));
  }
}

impl Component for RgbValue {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::NewMessage);

    let rgb_color_agent = RgbColorAgent::dispatcher();
    let _producer = CurrentColorAgent::bridge(callback);

    RgbValue {
      rgb_value: String::from(""),
      link,
      rgb_color_agent,
      _producer,
    }
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::NewMessage(response) => {
        self.rgb_value = response.rgb;
        true
      }
      Msg::ValueChanged(v) => {
        self.handle_value_change(v);
        true
      }
    }
  }

  fn view(&self) -> Html {
    html! {
        <div class="value-color">
          <span class="value-color__title">
            {TEXTS.rgb}
          </span>
          <ColorInput
            class="value-color__input"
            value={&self.rgb_value}
            on_change={self.link.callback(|value: String| Msg::ValueChanged(value))}
          />
        </div>
    }
  }
}
