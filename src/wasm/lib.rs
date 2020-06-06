mod agents;
mod constants;
pub mod libs;
mod root;
mod services;
mod texts;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use root::Root;

#[macro_use]
extern crate lazy_static;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
  App::<Root>::new().mount_to_body();
}
