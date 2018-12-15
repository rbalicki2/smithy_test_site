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

// use wasm_bindgen::JsValue;
// use web_sys::console::log_1;

fn get_window() -> Window {
  unsafe { transmute::<Object, Window>(global()) }
}

fn get_document() -> Document {
  get_window().document().unwrap()
}

struct AppState {
  click_count: i32,
}

// fn foo<'a, 'b>(count: u32, mut update_count: impl FnMut(u32) -> () + 'a) -> SmithyComponent<'b>
// where
//   'a: 'b,
// {
//   smd!(<h1 on_click={|_| update_count(4)}>{ format!("click = {}", count) }</h1>)
// }

#[wasm_bindgen]
pub fn start(div_id: String) {
  let doc: Document = get_document();
  let root_element: Element = doc.get_element_by_id(&div_id).unwrap();

  let mut app_state = AppState { click_count: 0 };

  let app_2 = smd!(
    on_hash_change={|_| app_state.click_count = 0};
    <div>
      { format!("{}", app_state.click_count) }
      <h1
        on_click={|_| app_state.click_count = app_state.click_count + 1}
      >
        MOAR
      </h1>
      <h2 on_click={|_| app_state.click_count = app_state.click_count - 1}>
        LESS
      </h2>
    </div>
  );

  smithy::mount(Box::new(app_2), root_element);
}
