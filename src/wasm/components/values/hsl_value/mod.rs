use crate::agents::current_color_agent::{CurrentColorAgent, Response};
use crate::agents::hsl_color_agent::{HslColorAgent, Request};
use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

use crate::components::values::color_input::ColorInput;
use crate::texts::TEXTS;

pub enum Msg {
  NewMessage(Response),
  ValueChanged(String),
}

pub struct HslValue {
  hsl_value: String,
  link: ComponentLink<HslValue>,
  hsl_color_agent: Dispatcher<HslColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
}

impl HslValue {
  fn handle_value_change(&mut self, value: String) {
    self.hsl_color_agent.send(Request::HslColorChangeMsg(value));
  }
}

impl Component for HslValue {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::NewMessage);

    let hsl_color_agent = HslColorAgent::dispatcher();
    let _producer = CurrentColorAgent::bridge(callback);

    HslValue {
      hsl_value: String::from(""),
      link,
      hsl_color_agent,
      _producer,
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
            {TEXTS.hsl}
          </span>
          <ColorInput
            class="value-color__input"
            value={self.hsl_value.clone()}
            on_change={self.link.callback(|value: String| Msg::ValueChanged(value))}
          />
        </div>
    }
  }
}
