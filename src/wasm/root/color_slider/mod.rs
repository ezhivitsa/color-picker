use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender, NodeRef};
use web_sys::{MouseEvent,HtmlElement};
use yew::agent::{Dispatched, Dispatcher};

use yew::utils::document;

use crate::agents::current_color_agent::{CurrentColorAgent, Response, Request};
use crate::services::mouse_move::{MouseService, MouseTask};

use crate::constants::MAX_H;

pub enum Msg {
  CurrentColorMessage(Response),
  MouseDown(MouseEvent),
  MouseMove(MouseEvent),
  MouseUp(MouseEvent),
  MouseOut(MouseEvent)
}

struct Tasks {
  _mouse_move: MouseTask,
  _mouse_up: MouseTask,
  _mouse_out: MouseTask
}

struct SliderData {
  hue: i32,
  start: i32
}

impl Tasks {
  fn new(
    _mouse_move: MouseTask,
    _mouse_up: MouseTask,
    _mouse_out: MouseTask
  ) -> Tasks {
    Tasks {
      _mouse_move,
      _mouse_up,
      _mouse_out
    }
  }
}

pub struct ColorSlider {
  color: String,
  hue: i32,
  link: ComponentLink<ColorSlider>,
  current_color_agent: Dispatcher<CurrentColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
  start_data: Option<SliderData>,
  _tasks: Tasks,
  slider_ref: NodeRef
}

impl ColorSlider {
  fn handle_mouse_down(&mut self, event: MouseEvent) {
    self.start_data = Some(SliderData {
      start: event.screen_x(),
      hue: self.hue
    });
  }

  fn handle_mouse_move(&mut self, event: MouseEvent) {
    if let Some(start_data) = &self.start_data {
      let diff = event.screen_x() - start_data.start;
      let slider_width = self.slider_ref.cast::<HtmlElement>().unwrap().offset_width();

      let hue_diff = diff as f32 / slider_width as f32 * MAX_H as f32;
      let hue_diff = hue_diff.round() as i32;

      let hue = start_data.hue + hue_diff;
      let hue = (hue.max(0)).min(MAX_H);

      self
          .current_color_agent
          .send(Request::CurrentHueMsg(hue));
    }
  }

  fn handle_mouse_up(&mut self, _: MouseEvent) {
    self.start_data = None;
  }

  fn handle_mouse_out(&mut self, _: MouseEvent) {
    self.start_data = None;
  }
}

impl Component for ColorSlider {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> ColorSlider {
    let callback = link.callback(Msg::CurrentColorMessage);

    let current_color_agent = CurrentColorAgent::dispatcher();
    let _producer = CurrentColorAgent::bridge(callback);

    let move_callback = link.callback(|e: MouseEvent| Msg::MouseMove(e));
    let up_callback = link.callback(|e: MouseEvent| Msg::MouseUp(e));
    let out_callback = link.callback(|e: MouseEvent| Msg::MouseOut(e));
    
    let _mousemove_task = MouseService::new(
      document().body().unwrap().into(),
      String::from("mousemove")
    ).register(move_callback);
    let _mouseup_task = MouseService::new(
      document().body().unwrap().into(),
      String::from("mouseup")
    ).register(up_callback);
    let _mouseout_task = MouseService::new(
      document().body().unwrap().into(),
      String::from("mouseleave")
    ).register(out_callback);

    let _tasks = Tasks::new(
      _mousemove_task,
      _mouseup_task,
      _mouseout_task
    );

    ColorSlider {
      color: String::from(""),
      hue: 0,
      link,
      current_color_agent,
      _producer,
      _tasks,
      start_data: None,
      slider_ref: NodeRef::default()
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::CurrentColorMessage(response) => {
        self.color = response.top_right_corner;
        self.hue = response.hue;
        true
      }

      Msg::MouseDown(event) => {
        self.handle_mouse_down(event);
        false
      }

      Msg::MouseMove(event) => {
        self.handle_mouse_move(event);
        false
      }

      Msg::MouseUp(event) => {
        self.handle_mouse_up(event);
        false
      }

      Msg::MouseOut(event) => {
        self.handle_mouse_out(event);
        false
      }
    }
  }

  fn view(&self) -> Html {
    let left = self.hue as f32 / MAX_H as f32 * 100.0;

    html! {
      <div class="slider">
        <div
          class="slider__hue"
          ref={self.slider_ref.clone()}
        />
        <div
          class="slider__selector"
          style={format!("background-color: {}; left: {}%;", self.color, left)}
          onmousedown={self.link.callback(|e: MouseEvent| Msg::MouseDown(e))}
        />
      </div>
    }
  }
}
