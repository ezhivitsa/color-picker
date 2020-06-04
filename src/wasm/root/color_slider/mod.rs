use web_sys::{Element, HtmlElement, MouseEvent};
use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, NodeRef, ShouldRender};

use crate::agents::current_color_agent::{CurrentColorAgent, Request, Response};
use crate::services::mouse::{MouseService, MouseTask};

use crate::constants::{MAX_H, MIN_HSV};

pub enum Msg {
  CurrentColorMessage(Response),
  MouseDown(MouseEvent),
  MouseMove(MouseEvent),
  MouseUp(MouseEvent),
  MouseOut(MouseEvent),
  SliderClick(MouseEvent),
}

struct Tasks {
  _mouse_move: MouseTask,
  _mouse_up: MouseTask,
  _mouse_out: MouseTask,
}

struct SliderData {
  hue: f32,
  start: i32,
}

impl Tasks {
  fn new(_mouse_move: MouseTask, _mouse_up: MouseTask, _mouse_out: MouseTask) -> Tasks {
    Tasks {
      _mouse_move,
      _mouse_up,
      _mouse_out,
    }
  }
}

pub struct ColorSlider {
  color: String,
  hue: f32,
  link: ComponentLink<ColorSlider>,
  current_color_agent: Dispatcher<CurrentColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
  start_data: Option<SliderData>,
  _tasks: Tasks,
  slider_ref: NodeRef,
}

impl ColorSlider {
  fn handle_mouse_down(&mut self, event: MouseEvent) {
    self.start_data = Some(SliderData {
      start: event.screen_x(),
      hue: self.hue,
    });
  }

  fn handle_mouse_move(&mut self, event: MouseEvent) {
    if let Some(start_data) = &self.start_data {
      let diff = event.screen_x() - start_data.start;
      let slider_width = self
        .slider_ref
        .cast::<HtmlElement>()
        .unwrap()
        .offset_width();

      let hue_diff = diff as f32 / slider_width as f32 * MAX_H;
      let hue_diff = hue_diff.round();

      let hue = start_data.hue + hue_diff;
      let hue = (hue.max(MIN_HSV)).min(MAX_H);

      self.current_color_agent.send(Request::CurrentHueMsg(hue));
    }
  }

  fn handle_mouse_up(&mut self, _: MouseEvent) {
    self.start_data = None;
  }

  fn handle_slider_click(&mut self, event: MouseEvent) {
    let x = event.client_x();
    let left = self
      .slider_ref
      .cast::<Element>()
      .unwrap()
      .get_bounding_client_rect()
      .left() as i32;

    let pallet_width = self
      .slider_ref
      .cast::<HtmlElement>()
      .unwrap()
      .offset_width();

    let hue = (x - left) as f32 / pallet_width as f32 * MAX_H;
    let hue = hue.round();

    self.current_color_agent.send(Request::CurrentHueMsg(hue));
    self.start_data = Some(SliderData { start: x, hue });
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

    let _mousemove_task = MouseService::new(String::from("mousemove")).register(move_callback);
    let _mouseup_task = MouseService::new(String::from("mouseup")).register(up_callback);
    let _mouseout_task = MouseService::new(String::from("mouseleave")).register(out_callback);

    let _tasks = Tasks::new(_mousemove_task, _mouseup_task, _mouseout_task);

    ColorSlider {
      color: String::from(""),
      hue: 0.0,
      link,
      current_color_agent,
      _producer,
      _tasks,
      start_data: None,
      slider_ref: NodeRef::default(),
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
        self.handle_mouse_up(event);
        false
      }

      Msg::SliderClick(event) => {
        self.handle_slider_click(event);
        false
      }
    }
  }

  fn view(&self) -> Html {
    let left = self.hue as f32 / MAX_H as f32 * 100.0;

    html! {
      <div class="slider">
        <div
          ref={self.slider_ref.clone()}
          class="slider__hue"
          onmousedown={self.link.callback(|e: MouseEvent| Msg::SliderClick(e))}
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
