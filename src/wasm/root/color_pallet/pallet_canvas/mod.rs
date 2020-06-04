use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, NodeRef, ShouldRender};

use crate::agents::current_color_agent::{CurrentColorAgent, Response};

use crate::constants::MAX_SVL;
use crate::libs::color_transform::{HSV, RGB};

pub enum Msg {
  CurrentColorMessage(Response),
}

pub struct PalletCanvas {
  canvas_ref: NodeRef,
  hue: f32,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
}

impl PalletCanvas {
  fn redraw_canvas(&self) {
    let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

    let width = MAX_SVL as i32 + 1;
    let height = MAX_SVL as i32 + 1;

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let ctx: CanvasRenderingContext2d = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into()
      .unwrap();

    for i in 0..height {
      for j in 0..width {
        let hsv = HSV::from_values(self.hue, j as f32, (height - i) as f32);
        let rgb = RGB::from_hsv(&hsv);

        ctx.set_fill_style(&JsValue::from_str(&format!("rgb({})", rgb.to_string())));
        ctx.fill_rect(j as f64, i as f64, 1.0, 1.0);
      }
    }
  }
}

impl Component for PalletCanvas {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::CurrentColorMessage);
    let _producer = CurrentColorAgent::bridge(callback);

    PalletCanvas {
      canvas_ref: NodeRef::default(),
      hue: 0.0,
      _producer,
    }
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::CurrentColorMessage(response) => {
        if self.hue != response.hue {
          self.hue = response.hue;
          self.redraw_canvas();
        }

        false
      }
    }
  }

  fn view(&self) -> Html {
    html! {
      <canvas
        class="pallet__canvas"
        ref={self.canvas_ref.clone()}
      />
    }
  }
}
