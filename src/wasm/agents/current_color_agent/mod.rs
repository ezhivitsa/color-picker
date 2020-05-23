use serde::{Deserialize, Serialize};
use yew::worker::{Agent, AgentLink, HandlerId, Context};
use std::collections::HashSet;

use crate::libs::color_transform;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  CurrentColorMsg(i32, i32),
}

pub struct CurrentColorAgent {
  link: AgentLink<CurrentColorAgent>,
  subscribers: HashSet<HandlerId>,
}

impl Agent for CurrentColorAgent {
  type Reach = Context;
  type Message = ();
  type Input = Request;
  type Output = String;

  fn create(link: AgentLink<Self>) -> Self {
    CurrentColorAgent {
        link,
        subscribers: HashSet::new(),
    }
  }

  fn update(&mut self, _: Self::Message) {}

  fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
    match msg {
      Request::CurrentColorMsg(saturation, value) => {
        let rgb = color_transform::hsv_to_rgb(50, saturation, value);
        let hex = color_transform::rgb_to_hex(rgb.0, rgb.1, rgb.2);

        for sub in self.subscribers.iter() {
          self.link.respond(*sub, hex.to_string());
        }
      }
    }
  }

  fn connected(&mut self, id: HandlerId) {
    self.subscribers.insert(id);
  }

  fn disconnected(&mut self, id: HandlerId) {
    self.subscribers.remove(&id);
  }
}
