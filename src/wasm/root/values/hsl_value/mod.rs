use crate::agents::current_color_agent::{CurrentColorAgent, Request, Response};
use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

use crate::root::values::color_input::ColorInput;

pub enum Msg {
  NewMessage(Response),
  ValueChanged(String),
}

pub struct HslValue {
  hsl_value: String,
  link: ComponentLink<HslValue>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
  _current_color_agent: Dispatcher<CurrentColorAgent>,
}

impl HslValue {
  fn handle_value_change(&mut self, value: String) {
    self
      ._current_color_agent
      .send(Request::HslColorChangeMsg(value));
  }
}

impl Component for HslValue {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::NewMessage);

    let _current_color_agent = CurrentColorAgent::dispatcher();
    let _producer = CurrentColorAgent::bridge(callback);

    HslValue {
      hsl_value: String::from(""),
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
        self.hsl_value = response.hsl;
        true
      }
      Msg::ValueChanged(e) => {
        self.handle_value_change(e);
        false
      }
    }
  }

  fn view(&self) -> Html {
    html! {
        <div class="value-color">
          <span class="value-color__title">
            {"HSL"}
          </span>
          <ColorInput
            class="value-color__input"
            value={&self.hsl_value}
            on_change={self.link.callback(|value: String| Msg::ValueChanged(value))}
          />
        </div>
    }
  }
}
