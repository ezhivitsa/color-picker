use crate::agents::current_color_agent::{CurrentColorAgent, Request, Response};
use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

use crate::root::values::color_input::ColorInput;

pub enum Msg {
  NewMessage(Response),
  ValueChanged(String),
}

pub struct CmykValue {
  cmyk_value: String,
  link: ComponentLink<CmykValue>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
  _current_color_agent: Dispatcher<CurrentColorAgent>,
}

impl CmykValue {
  fn handle_value_change(&mut self, value: String) {
    self
      ._current_color_agent
      .send(Request::CmykColorChangeMsg(value));
  }
}

impl Component for CmykValue {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::NewMessage);

    let _current_color_agent = CurrentColorAgent::dispatcher();
    let _producer = CurrentColorAgent::bridge(callback);

    CmykValue {
      cmyk_value: String::from(""),
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
        self.cmyk_value = response.cmyk;
        true
      }
      Msg::ValueChanged(value) => {
        self.handle_value_change(value);
        true
      }
    }
  }

  fn view(&self) -> Html {
    html! {
        <div class="value-color">
          <span class="value-color__title">
            {"CMYK"}
          </span>
          <ColorInput
            class="value-color__input"
            value={&self.cmyk_value}
            on_change={self.link.callback(|value: String| Msg::ValueChanged(value))}
          />
        </div>
    }
  }
}
