use crate::agents::current_color_agent::{CurrentColorAgent, Response};
use crate::agents::hex_color_agent::{HexColorAgent, Request};
use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

use crate::components::values::color_input::ColorInput;
use crate::texts::TEXTS;

pub enum Msg {
  NewMessage(Response),
  ValueChanged(String),
}

pub struct HexValue {
  hex_value: String,
  last_hex_value: String,
  link: ComponentLink<HexValue>,
  hex_color_agent: Dispatcher<HexColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
}

impl HexValue {
  fn handle_value_change(&mut self, value: String) {
    self.hex_color_agent.send(Request::HexColorChangeMsg(value));
  }
}

impl Component for HexValue {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::NewMessage);

    let hex_color_agent = HexColorAgent::dispatcher();
    let _producer = CurrentColorAgent::bridge(callback);

    HexValue {
      hex_value: String::from(""),
      last_hex_value: String::from(""),
      link,
      hex_color_agent,
      _producer,
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::NewMessage(response) => {
        self.hex_value = response.hex.to_string();
        self.last_hex_value = response.hex;
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
        <div class="hex-color">
          <span class="hex-color__title">
            {TEXTS.hex}
          </span>
          <ColorInput
            class="hex-color__input"
            value={self.hex_value.clone()}
            on_change={self.link.callback(|value: String| Msg::ValueChanged(value))}
          />
        </div>
    }
  }
}
