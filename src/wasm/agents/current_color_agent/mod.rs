use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::{Agent, AgentLink, Context, HandlerId};

use crate::libs::color_transform::Color;
use crate::libs::color_validate;

use crate::constants::{MAX_SVL};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  CurrentHueMsg(i32),
  CurrentColorMsg(i32, i32),
  HexColorChangeMsg(String),
  RgbColorChangeMsg(String),
  CmykColorChangeMsg(String),
  HsvColorChangeMsg(String),
  HslColorChangeMsg(String)
}

#[derive(Serialize, Deserialize)]
pub struct Response {
  pub hex: String,
  pub rgb: String,
  pub cmyk: String,
  pub hsl: String,
  pub hsv: String,
  pub top_right_corner: String,
  pub hue: i32,
  pub saturation: i32,
  pub value: i32,
}

impl Response {
  fn new(color: &Color) -> Response {
    let top_right_color = Color::from_hsv_values(color.get_hue(), MAX_SVL, MAX_SVL);

    Response {
      hex: color.hex_value(),
      rgb: color.rgb_value(),
      cmyk: color.cmyk_value(),
      hsl: color.hsl_value(),
      hsv: color.hsv_value(),
      top_right_corner: top_right_color.hex_value(),
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
  fn handle_current_hue_change(&mut self, hue: i32) {
    self.color = Color::from_hsv_values(
      hue,
      self.color.get_saturation(),
      self.color.get_value()
    );
    self.send_to_subscribers();
  }

  fn handle_current_color_change(&mut self, saturation: i32, value: i32) {
    self.color = Color::from_hsv_values(self.color.get_hue(), saturation, value);
    self.send_to_subscribers();
  }

  fn handle_hex_value_change(&mut self, value: String) {
    if color_validate::is_valid_hex(&value) {
      self.color = Color::from_hex(value);
      self.send_to_subscribers();
    }
  }

  fn handle_rgb_value_change(&mut self, value: String) {
    if color_validate::is_valid_rgb(&value) {
      self.color = Color::from_rgb(value);
      self.send_to_subscribers();
    }
  }

  fn handle_cmyk_value_change(&mut self, value: String) {
    if color_validate::is_valid_cmyk(&value) {
      self.color = Color::from_cmyk(value);
      self.send_to_subscribers();
    }
  }

  fn handle_hsv_value_change(&mut self, value: String) {
    if color_validate::is_valid_hsv(&value) {
      self.color = Color::from_hsv(value);
      self.send_to_subscribers();
    }
  }

  fn handle_hsl_value_change(&mut self, value: String) {
    if color_validate::is_valid_hsv(&value) {
      self.color = Color::from_hsl(value);
      self.send_to_subscribers();
    }
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
    let color = Color::from_hsv_values(50, 20, 40);

    CurrentColorAgent {
      color,
      link,
      subscribers: HashSet::new(),
    }
  }

  fn update(&mut self, _: Self::Message) {}

  fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
    match msg {
      Request::CurrentHueMsg(hue) => {
        self.handle_current_hue_change(hue);
      }

      Request::CurrentColorMsg(saturation, value) => {
        self.handle_current_color_change(saturation, value);
      }

      Request::HexColorChangeMsg(hex) => {
        self.handle_hex_value_change(hex);
      }

      Request::RgbColorChangeMsg(rgb) => {
        self.handle_rgb_value_change(rgb);
      }

      Request::CmykColorChangeMsg(cmyk) => {
        self.handle_cmyk_value_change(cmyk);
      }

      Request::HsvColorChangeMsg(hsv) => {
        self.handle_hsv_value_change(hsv);
      }

      Request::HslColorChangeMsg(hsl) => {
        self.handle_hsl_value_change(hsl);
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
