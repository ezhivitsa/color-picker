use std::fmt;

use gloo::events::EventListener;
use web_sys::{Event, MouseEvent};
use yew::utils::document;

use wasm_bindgen::JsCast;

use yew::callback::Callback;

/// A service that fires events when a specific element is clicked.
#[derive(Debug)]
pub struct MouseService {
  event: String,
}

/// A handle to the event listener for click events.
#[must_use]
#[allow(dead_code)]
pub struct MouseTask {
  handle: EventListener,
}

impl fmt::Debug for MouseTask {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("MouseTask")
  }
}

impl MouseService {
  /// Creates a new ClickService.
  pub fn new(event: String) -> MouseService {
    MouseService { event }
  }

  /// Register a callback that will be called when the user clicks the element.
  pub fn register(&mut self, callback: Callback<MouseEvent>) -> MouseTask {
    let callback = move |event: &Event| {
      let mouse_event = event
        .dyn_ref::<MouseEvent>()
        .expect("wrong event type")
        .clone();

      callback.emit(mouse_event);
    };

    let handle = EventListener::new(
      &document().body().unwrap().into(),
      self.event.to_string(),
      callback,
    );
    MouseTask { handle }
  }
}
