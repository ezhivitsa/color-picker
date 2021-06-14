use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::{Agent, AgentLink, Context, HandlerId};

use crate::libs::color_transform::Color;

use crate::libs::color_transform::hsv_color::HSV;
use crate::libs::color_transform::rgb_color::RGB;

use crate::constants::{MAX_H, MAX_SVL};

#[derive(Serialize, Deserialize)]
pub enum CurrentColorRequest {
  UpdateColor(Color),
}

#[derive(Serialize, Deserialize)]
pub struct Response {
  pub hex: String,
  pub rgb: String,
  pub cmyk: String,
  pub hsl: String,
  pub hsv: String,
  pub top_right_corner: String,
  pub hue: f32,
  pub saturation: f32,
  pub value: f32,
}

impl Response {
  fn new(color: &Color) -> Response {
    let top_right_hsv = HSV::from_values(color.get_hue(), MAX_SVL, MAX_SVL);
    let top_right_rgb = RGB::from_hsv(&top_right_hsv);

    Response {
      hex: color.hex_value(),
      rgb: color.rgb_value(),
      cmyk: color.cmyk_value(),
      hsl: color.hsl_value(),
      hsv: color.hsv_value(),
      top_right_corner: top_right_rgb.to_color_string(),
      hue: color.get_hue(),
      saturation: color.get_saturation(),
      value: color.get_value(),
    }
  }
}

pub struct CurrentColorAgent {
  color: Color,
  link: AgentLink<CurrentColorAgent>,
  subscribers: HashSet<HandlerId>,
}

impl CurrentColorAgent {
  fn send_to_subscribers(&mut self, color: &Color) {
    for sub in self.subscribers.iter() {
      let response = Response::new(color);
      self.link.respond(*sub, response);
    }
  }
}

impl Agent for CurrentColorAgent {
  type Reach = Context<Self>;
  type Message = ();
  type Input = CurrentColorRequest;
  type Output = Response;

  fn create(link: AgentLink<Self>) -> Self {
    // default color
    let mut rng = rand::thread_rng();
    let hue: f32 = rand::random::<f32>() * MAX_H;
    let saturation: f32 = rng.gen::<f32>() * MAX_SVL;
    let value: f32 = rng.gen::<f32>() * MAX_SVL;

    let color = Color::from_hsv_values(hue.round(), saturation.round(), value.round());

    CurrentColorAgent {
      color,
      link,
      subscribers: HashSet::new(),
    }
  }

  fn update(&mut self, _: Self::Message) {}

  fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
    match msg {
      CurrentColorRequest::UpdateColor(color) => {
        self.send_to_subscribers(&color);
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
