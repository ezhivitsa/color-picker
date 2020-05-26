use crate::agents::current_color_agent::{CurrentColorAgent, Request, Response};
use yew::agent::{Dispatched, Dispatcher};
use yew::html::InputData;
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
  NewMessage(Response),
  ValueChanged(InputData),
}

pub struct HslValue {
  color: String,
  link: ComponentLink<HslValue>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
  _current_color_agent: Dispatcher<CurrentColorAgent>,
}

impl HslValue {
  fn handle_value_change(&mut self, e: InputData) {
    self
      ._current_color_agent
      .send(Request::RgbColorChangeMsg(e.value));
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
      color: "".to_string(),
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
        self.color = response.hex;
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
          <input
            class="value-color__input"
            value={&self.color}
            oninput=self.link.callback(|e: InputData| Msg::ValueChanged(e))
          />
        </div>
    }
  }
}
