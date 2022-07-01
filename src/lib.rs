#[macro_use]
pub mod utils;
pub mod app;

use app::App;

use sauron::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
  Program::mount_to_body(App::new());
}
