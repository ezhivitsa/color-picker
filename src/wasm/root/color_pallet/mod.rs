use yew::agent::{Dispatched, Dispatcher};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender, NodeRef};
use web_sys::{HtmlElement, MouseEvent};

use crate::agents::current_color_agent::{CurrentColorAgent, Request, Response};
use crate::services::mouse::{MouseService, MouseTask};

use crate::constants::MAX_SVL;

struct SliderData {
  saturation: i32,
  value: i32,
  start_x: i32,
  start_y: i32
}

struct Tasks {
  _mouse_move: MouseTask,
  _mouse_up: MouseTask,
  _mouse_out: MouseTask,
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

pub struct ColorPallet {
  color: String,
  saturation: i32,
  value: i32,
  link: ComponentLink<ColorPallet>,
  current_color_agent: Dispatcher<CurrentColorAgent>,
  _producer: Box<dyn Bridge<CurrentColorAgent>>,
  pallet_ref: NodeRef,
  start_data: Option<SliderData>,
  _tasks: Tasks
}

pub enum Msg {
  CurrentColorMessage(Response),
  MouseDown(MouseEvent),
  MouseMove(MouseEvent),
  MouseUp(MouseEvent),
  MouseOut(MouseEvent)
}

impl ColorPallet {
  fn handle_mouse_down(&mut self, event: MouseEvent) {
    self.start_data = Some(SliderData {
      start_x: event.screen_x(),
      start_y: event.screen_y(),
      saturation: self.saturation,
      value: self.value
    });
  }

  fn handle_mouse_move(&mut self, event: MouseEvent) {
    if let Some(start_data) = &self.start_data {
      let diff_x = event.screen_x() - start_data.start_x;
      let diff_y = event.screen_y() - start_data.start_y;

      let pallet_element = self
        .pallet_ref
        .cast::<HtmlElement>()
        .unwrap();

      let pallet_width = pallet_element.offset_width();
      let pallet_height = pallet_element.offset_height();

      let saturation_diff = diff_x as f32 / pallet_width as f32 * MAX_SVL as f32;
      let saturation_diff = saturation_diff.round() as i32;

      let value_diff = diff_y as f32 / pallet_height as f32 * MAX_SVL as f32;
      let value_diff = value_diff.round() as i32;

      let saturation = start_data.saturation + saturation_diff;
      let saturation = (saturation.max(0)).min(MAX_SVL);

      let value = start_data.value - value_diff;
      let value = (value.max(0)).min(MAX_SVL);

      self.current_color_agent.send(Request::CurrentColorMsg(saturation, value));
    }
  }

  fn handle_mouse_up(&mut self, _: MouseEvent) {
    self.start_data = None;
  }
}

impl Component for ColorPallet {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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

    ColorPallet {
      color: String::from(""),
      saturation: 0,
      value: 0,
      current_color_agent,
      link,
      _producer,
      pallet_ref: NodeRef::default(),
      _tasks,
      start_data: None
    }
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::CurrentColorMessage(response) => {
        self.color = response.hex;
        self.saturation = response.saturation;
        self.value = response.value;

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

    }
  }

  fn view(&self) -> Html {
    let left = self.saturation as f32 / MAX_SVL as f32 * 100.0;
    let bottom = self.value as f32 / MAX_SVL as f32 * 100.0;

    html! {
      <div class="colors__pallet">
        <div
          class="pallet"
          ref={self.pallet_ref.clone()}
        >
          <div
            class="pallet__selector"
            style={format!("background-color: {}; bottom: {}%; left: {}%;", self.color, bottom, left)}
            onmousedown={self.link.callback(|e: MouseEvent| Msg::MouseDown(e))}
          />
        </div>
      </div>
    }
  }
}
