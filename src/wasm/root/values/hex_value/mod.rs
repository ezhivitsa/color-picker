use crate::agents::current_color_agent::{CurrentColorAgent, Request, Response};
use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

use crate::root::values::color_input::ColorInput;

pub enum Msg {
  NewMessage(Response),
  ValueChanged(String),
}

pub struct HexValue {
  hex_value: String,
  last_hex_value: String,
  link: ComponentLink<HexValue>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
  _current_color_agent: Dispatcher<CurrentColorAgent>,
}

impl HexValue {
  fn handle_value_change(&mut self, value: String) {
    self
      ._current_color_agent
      .send(Request::HexColorChangeMsg(value));
  }
}

impl Component for HexValue {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::NewMessage);

    let _current_color_agent = CurrentColorAgent::dispatcher();
    let _producer = CurrentColorAgent::bridge(callback);

    HexValue {
      hex_value: String::from(""),
      last_hex_value: String::from(""),
      link,
      _producer,
      _current_color_agent,
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
            {"HEX"}
          </span>
          <ColorInput
            class="hex-color__input"
            value={&self.hex_value}
            on_change={self.link.callback(|value: String| Msg::ValueChanged(value))}
          />
        </div>
    }
  }
}
