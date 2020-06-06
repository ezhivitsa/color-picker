use serde::{Deserialize, Serialize};
use yew::agent::{Dispatcher, Agent, Context, AgentLink, Dispatched, HandlerId, Bridge, Bridged};

use crate::agents::current_color_agent::{CurrentColorAgent, CurrentColorRequest, Response};
use crate::libs::color_transform::Color;

use crate::libs::color_validate;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  HsvColorChangeMsg(String),
  HueChangedMsg(f32),
  SaturationValueChangedMsg(f32, f32),
}

pub enum Msg {
  CurrentColorChange(Response)
}

pub struct HsvColorAgent {
  hue: f32,
  saturation: f32,
  value: f32,
  current_color_dispatcher: Dispatcher<CurrentColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>
}

impl Agent for HsvColorAgent {
  type Reach = Context;
  type Message = Msg;
  type Input = Request;
  type Output = ();

  fn create(link: AgentLink<Self>) -> Self {
    let callback = link.callback(Msg::CurrentColorChange);
    let _producer = CurrentColorAgent::bridge(callback);

    HsvColorAgent {
      hue: 0.0,
      saturation: 0.0,
      value: 0.0,
      current_color_dispatcher: CurrentColorAgent::dispatcher(),
      _producer
    }
  }

  fn update(&mut self, msg: Self::Message) {
    match msg {
      Msg::CurrentColorChange(response) => {
        self.hue = response.hue;
        self.saturation = response.saturation;
        self.value = response.value;
      }
    }
  }

  fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
    match msg {
      Request::HsvColorChangeMsg(hsv) => {
        if color_validate::is_valid_hsv(&hsv) {
          let color = Color::from_hsv(hsv);
          self.current_color_dispatcher.send(CurrentColorRequest::UpdateColor(color));
        }
      }

      Request::HueChangedMsg(hue) => {
        let color = Color::from_hsv_values(
          hue,
          self.saturation,
          self.value
        );
        self.current_color_dispatcher.send(CurrentColorRequest::UpdateColor(color));
      }

      Request::SaturationValueChangedMsg(saturation, value) => {
        let color = Color::from_hsv_values(
          self.hue,
          saturation,
          value
        );
        self.current_color_dispatcher.send(CurrentColorRequest::UpdateColor(color));
      }
    }
  }
}
