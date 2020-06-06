use crate::agents::current_color_agent::{CurrentColorAgent, Response};
use crate::agents::hsv_color_agent::{HsvColorAgent, Request};
use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

use crate::root::values::color_input::ColorInput;
use crate::texts::TEXTS;

pub enum Msg {
  NewMessage(Response),
  ValueChanged(String),
}

pub struct HsvValue {
  hsv_value: String,
  link: ComponentLink<HsvValue>,
  hsv_color_agent: Dispatcher<HsvColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
}

impl HsvValue {
  fn handle_value_change(&mut self, value: String) {
    self
      .hsv_color_agent
      .send(Request::HsvColorChangeMsg(value));
  }
}

impl Component for HsvValue {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::NewMessage);

    let hsv_color_agent = HsvColorAgent::dispatcher();
    let _producer = CurrentColorAgent::bridge(callback);

    HsvValue {
      hsv_value: String::from(""),
      link,
      hsv_color_agent,
      _producer,
    }
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::NewMessage(response) => {
        self.hsv_value = response.hsv;
        true
      }
      Msg::ValueChanged(value) => {
        self.handle_value_change(value);
        false
      }
    }
  }

  fn view(&self) -> Html {
    html! {
        <div class="value-color">
          <span class="value-color__title">
            {TEXTS.hsv}
          </span>
          <ColorInput
            class="value-color__input"
            value={&self.hsv_value}
            on_change={self.link.callback(|value: String| Msg::ValueChanged(value))}
          />
        </div>
    }
  }
}
