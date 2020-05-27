use crate::agents::current_color_agent::{CurrentColorAgent, Request, Response};
use yew::agent::{Dispatched, Dispatcher};
use yew::html::InputData;
use yew::web_sys::FocusEvent;
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
  NewMessage(Response),
  ValueChanged(InputData),
  Blur,
  Focus,
}

pub struct HexValue {
  hex_value: String,
  last_hex_value: String,
  focused: bool,
  link: ComponentLink<HexValue>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
  _current_color_agent: Dispatcher<CurrentColorAgent>,
}

impl HexValue {
  fn handle_value_change(&mut self, e: InputData) {
    self.hex_value = e.value.to_string();

    self
      ._current_color_agent
      .send(Request::HexColorChangeMsg(e.value));
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
      focused: false,
      link,
      _producer,
      _current_color_agent,
    }
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::NewMessage(response) => {
        self.hex_value = response.hex.to_string();
        self.last_hex_value = response.hex;

        !self.focused
      }

      Msg::ValueChanged(e) => {
        self.handle_value_change(e);
        true
      }

      Msg::Blur => {
        self.hex_value = self.last_hex_value.to_string();
        self.focused = false;
        true
      }

      Msg::Focus => {
        self.focused = true;
        false
      }
    }
  }

  fn view(&self) -> Html {
    html! {
        <div class="hex-color">
          <span class="hex-color__title">
            {"HEX"}
          </span>
          <input
            class="hex-color__input"
            value={&self.hex_value}
            oninput=self.link.callback(|e: InputData| Msg::ValueChanged(e))
            onfocus={self.link.callback(|_: FocusEvent| Msg::Focus)}
            onblur={self.link.callback(|_: FocusEvent| Msg::Blur)}
          />
        </div>
    }
  }
}
