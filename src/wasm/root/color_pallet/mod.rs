use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Component, ComponentLink, Html, ShouldRender, Bridge, Bridged};

use crate::agents::current_color_agent::{CurrentColorAgent, Request, Response};

use crate::constants::MAX_SVL;

pub struct ColorPallet {
  color: String,
  saturation: i32,
  value: i32,
  link: ComponentLink<ColorPallet>,
  current_color_agent: Dispatcher<CurrentColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
}

pub enum Msg {
  Clicked,
  CurrentColorMessage(Response),
}

impl Component for ColorPallet {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let current_color_agent = CurrentColorAgent::dispatcher();
    let callback = link.callback(Msg::CurrentColorMessage);
    let _producer = CurrentColorAgent::bridge(callback);

    ColorPallet {
      color: String::from(""),
      saturation: 0,
      value: 0,
      current_color_agent,
      link,
      _producer
    }
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::CurrentColorMessage(response) => {
        self.color = response.hex;
        self.saturation = response.saturation;
        self.value = response.value;

        true
      },
  
      Msg::Clicked => {
        self
          .current_color_agent
          .send(Request::CurrentColorMsg(30, 60));
        false
      }
    }
  }

  fn view(&self) -> Html {
    let left = self.saturation as f32 / MAX_SVL as f32 * 100.0;
    let bottom = self.value as f32 / MAX_SVL as f32 * 100.0;

    html! {
      <div class="colors__pallet">
        <div class="pallet">
          <div
            class="pallet__selector"
            style={format!("background-color: {}; bottom: {}%; left: {}%;", self.color, bottom, left)}
          />
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
