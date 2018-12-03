#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;

use js_sys::{
  global,
  Object,
};
use std::mem::transmute;
use wasm_bindgen::JsValue;
use web_sys::{
  console::log_1,
  Document,
  Element,
  Window,
};

use smithy::smd;

fn get_window() -> Window {
  unsafe { transmute::<Object, Window>(global()) }
}

fn get_document() -> Document {
  get_window().document().unwrap()
}

#[wasm_bindgen]
pub fn start(div_id: String) {
  let doc: Document = get_document();
  let root_element: Element = doc.get_element_by_id(&div_id).unwrap();

  let app = smd!(<h1>HIIII</h1>);

  smithy::mount(app, root_element);
}
