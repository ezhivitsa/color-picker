use yew::callback::Callback;
use yew::html::InputData;
use web_sys::FocusEvent;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub enum Msg {
  ValueChanged(InputData),
  Blur,
  Focus,
}

pub struct ColorInput {
  class: String,
  on_change: Callback<String>,
  value: String,
  last_value: String,
  focused: bool,
  link: ComponentLink<ColorInput>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub value: String,
  pub class: String,
  pub on_change: Callback<String>,
}

impl Component for ColorInput {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    ColorInput {
      class: props.class,
      on_change: props.on_change,
      value: props.value.to_string(),
      last_value: props.value.to_string(),
      focused: false,
      link,
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.last_value = props.value.to_string();

    if !self.focused {
      self.value = props.value.to_string();
    }

    !self.focused
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::ValueChanged(e) => {
        self.value = e.value.to_string();
        self.on_change.emit(e.value);
        true
      }

      Msg::Blur => {
        self.value = self.last_value.to_string();
        self.focused = false;

        true
      }

      Msg::Focus => {
        self.focused = true;
        false
      }
    }
  }

  fn view(&self) -> Html {
    html! {
      <input
        class={&self.class}
        value={&self.value}
        oninput=self.link.callback(|e: InputData| Msg::ValueChanged(e))
        onfocus={self.link.callback(|_: FocusEvent| Msg::Focus)}
        onblur={self.link.callback(|_: FocusEvent| Msg::Blur)}
      />
    }
  }
}
