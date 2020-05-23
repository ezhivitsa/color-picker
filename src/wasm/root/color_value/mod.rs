use yew::{
  Component,
  Bridge,
  ComponentLink,
  ShouldRender,
  Html,
  html,
  Bridged
};

use crate::agents::current_color_agent::CurrentColorAgent;

pub enum Msg {
  NewMessage(String),
}

pub struct ColorValue {
  color: String,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
}

impl Component for ColorValue {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::NewMessage);
    let _producer = CurrentColorAgent::bridge(callback);

    ColorValue {
        color: "#ffffff".to_string(),
        _producer,
    }
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
          Msg::NewMessage(s) => self.color = s,
      }
      true
  }

  fn view(&self) -> Html {
      html! {
          <div
            class="colors__current"
            style=format!("background-color: {};", &self.color)
          >
            {&self.color}
          </div>
      }
  }
}
