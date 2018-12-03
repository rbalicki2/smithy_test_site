#![feature(proc_macro_hygiene, slice_patterns)]

use wasm_bindgen::prelude::*;

use js_sys::{
  global,
  Object,
};
use std::mem::transmute;
use web_sys::{
  Document,
  Element,
  Window,
};

use smithy::{
  smd,
  types::Component,
};

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

  struct AppState {
    click_count: u32,
  }

  let mut app_state = AppState { click_count: 0 };

  let app = smd!(
    <h1 on_click={|_| app_state.click_count = app_state.click_count + 1}>
      hello kerry
    </h1>
    <div>
      I have clicked { format!("{}", app_state.click_count) } times
    </div>
  );

  smithy::mount(app, root_element);
}
