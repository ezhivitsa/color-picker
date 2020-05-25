use serde::{Deserialize, Serialize};
use yew::worker::{Agent, AgentLink, HandlerId, Context};
use std::collections::HashSet;

use crate::libs::color_transform::Color;

use crate::constants::{
  MAX_S,
  MAX_V
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  CurrentColorMsg(i32, i32),
  HexColorChangeMsg(String)
}

#[derive(Serialize, Deserialize)]
pub struct Response {
  pub hex: String,
  pub top_right_corner: String
}

impl Response {
  fn new(color: &Color) -> Response {
    let top_right_color = Color::from_hsv(
      color.hsv.get_hue(),
      MAX_S,
      MAX_V
    );
    Response {
      hex: color.hex_value(),
      top_right_corner: top_right_color.hex_value()
    }
  }
}

pub struct CurrentColorAgent {
  color: Color,
  link: AgentLink<CurrentColorAgent>,
  subscribers: HashSet<HandlerId>,
}

impl CurrentColorAgent {
  fn handle_current_color_change(&mut self, saturation: i32, value: i32) {
    self.color = Color::from_hsv(50, saturation, value);
    self.send_to_subscribers();
  }

  fn handle_hex_value_change(&mut self, value: String) {

  }

  fn send_to_subscribers(&mut self) {
    for sub in self.subscribers.iter() {
      let response = Response::new(&self.color);
      self.link.respond(*sub, response);
    }
  }
}

impl Agent for CurrentColorAgent {
  type Reach = Context;
  type Message = ();
  type Input = Request;
  type Output = Response;

  fn create(link: AgentLink<Self>) -> Self {
    // default color
    let color = Color::from_hsv(50, 20, 40);

    CurrentColorAgent {
        color,
        link,
        subscribers: HashSet::new(),
    }
  }

  fn update(&mut self, _: Self::Message) {}

  fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
    match msg {
      Request::CurrentColorMsg(saturation, value) => {
        self.handle_current_color_change(
          saturation,
          value
        );
      },

      Request::HexColorChangeMsg(hex) => {
        self.handle_hex_value_change(hex);
      }
    }
  }

  fn connected(&mut self, id: HandlerId) {
    self.subscribers.insert(id);

    let initial_value = Response::new(&self.color);
    self.link.respond(id, initial_value);
  }

  fn disconnected(&mut self, id: HandlerId) {
    self.subscribers.remove(&id);
  }
}
