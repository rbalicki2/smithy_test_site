#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;
use web_sys::console::log_1;

use smithy::smd;

#[wasm_bindgen]
pub fn start() {
  let div = smd!(<h1>HIIII</h1>);

  let foo = smithy::mount();
  log_1(&JsValue::from_str(&format!("foo={:?}", foo)));
}
