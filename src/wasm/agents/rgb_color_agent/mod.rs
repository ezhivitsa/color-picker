use serde::{Deserialize, Serialize};
use yew::agent::{Agent, AgentLink, Context, Dispatched, Dispatcher, HandlerId};

use crate::agents::current_color_agent::{CurrentColorAgent, CurrentColorRequest};
use crate::libs::color_transform::Color;

use crate::libs::color_validate;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  RgbColorChangeMsg(String),
}

pub struct RgbColorAgent {
  current_color_dispatcher: Dispatcher<CurrentColorAgent>,
}

impl Agent for RgbColorAgent {
  type Reach = Context;
  type Message = ();
  type Input = Request;
  type Output = ();

  fn create(_: AgentLink<Self>) -> Self {
    RgbColorAgent {
      current_color_dispatcher: CurrentColorAgent::dispatcher(),
    }
  }

  fn update(&mut self, _: Self::Message) {}

  fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
    match msg {
      Request::RgbColorChangeMsg(rgb) => {
        if color_validate::is_valid_rgb(&rgb) {
          let color = Color::from_rgb(rgb);
          self
            .current_color_dispatcher
            .send(CurrentColorRequest::UpdateColor(color));
        }
      }
    }
  }
}
