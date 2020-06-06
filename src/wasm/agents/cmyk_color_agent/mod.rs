use serde::{Deserialize, Serialize};
use yew::agent::{Dispatcher, Agent, Context, AgentLink, Dispatched, HandlerId};

use crate::agents::current_color_agent::{CurrentColorAgent, CurrentColorRequest};
use crate::libs::color_transform::Color;

use crate::libs::color_validate;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  CmykColorChangeMsg(String)
}

pub struct CmykColorAgent {
  current_color_dispatcher: Dispatcher<CurrentColorAgent>,
}

impl Agent for CmykColorAgent {
  type Reach = Context;
  type Message = ();
  type Input = Request;
  type Output = ();

  fn create(_: AgentLink<Self>) -> Self {
    CmykColorAgent {
      current_color_dispatcher: CurrentColorAgent::dispatcher(),
    }
  }

  fn update(&mut self, _: Self::Message) {}

  fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
    match msg {
      Request::CmykColorChangeMsg(cmyk) => {
        if color_validate::is_valid_cmyk(&cmyk) {
          let color = Color::from_cmyk(cmyk);
          self.current_color_dispatcher.send(CurrentColorRequest::UpdateColor(color));
        }
      }
    }
  }
}
