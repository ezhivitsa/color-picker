use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::agents::current_color_agent::{CurrentColorAgent, Request};

pub struct ColorPallet {
  link: ComponentLink<ColorPallet>,
  current_color_agent: Dispatcher<CurrentColorAgent>,
}

pub enum Msg {
  Clicked,
}

impl Component for ColorPallet {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let current_color_agent = CurrentColorAgent::dispatcher();

    ColorPallet {
      current_color_agent,
      link,
    }
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Clicked => {
        self
          .current_color_agent
          .send(Request::CurrentColorMsg(30, 60));
        false
      }
    }
  }

  fn view(&self) -> Html {
    html! {
      <div class="colors__pallet">
        <div class="pallet">
          <button
              onclick=self.link.callback(|_| Msg::Clicked)
          >
              {"PUSH ME"}
          </button>
        </div>
      </div>
    }
  }
}
