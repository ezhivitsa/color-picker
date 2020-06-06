use crate::agents::current_color_agent::{CurrentColorAgent, Response};
use crate::agents::cmyk_color_agent::{CmykColorAgent, Request};
use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

use crate::root::values::color_input::ColorInput;
use crate::texts::TEXTS;

pub enum Msg {
  NewMessage(Response),
  ValueChanged(String),
}

pub struct CmykValue {
  cmyk_value: String,
  link: ComponentLink<CmykValue>,
  cmyk_color_agent: Dispatcher<CmykColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
}

impl CmykValue {
  fn handle_value_change(&mut self, value: String) {
    self
      .cmyk_color_agent
      .send(Request::CmykColorChangeMsg(value));
  }
}

impl Component for CmykValue {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::NewMessage);

    let cmyk_color_agent = CmykColorAgent::dispatcher();
    let _producer = CurrentColorAgent::bridge(callback);

    CmykValue {
      cmyk_value: String::from(""),
      link,
      cmyk_color_agent,
      _producer,
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
            {TEXTS.cmyk}
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
