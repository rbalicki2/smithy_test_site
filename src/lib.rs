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
  types::{
    Component,
    SmithyComponent,
  },
};

use wasm_bindgen::JsValue;
use web_sys::console::log_1;

fn get_window() -> Window {
  unsafe { transmute::<Object, Window>(global()) }
}

fn get_document() -> Document {
  get_window().document().unwrap()
}

struct AppState {
  click_count: u32,
}

fn foo<'a, 'b>(count: u32, mut update_count: impl FnMut(u32) -> () + 'a) -> SmithyComponent<'b>
where
  'a: 'b,
{
  smd!(<h1 on_click={|_| update_count(4)}>{ format!("click = {}", count) }</h1>)
}

#[wasm_bindgen]
pub fn start(div_id: String) {
  let doc: Document = get_document();
  let root_element: Element = doc.get_element_by_id(&div_id).unwrap();

  let mut app_state = AppState { click_count: 0 };

  // TODO log app_state, see if it's changing...

  let app_2 = smd!(<div>
    {format!("{}", app_state.click_count) }
    {
      &mut foo(
        app_state.click_count,
        |new_count| {
          log_1(&JsValue::from(format!("{}", app_state.click_count)));
          app_state.click_count = new_count;
        }
      )
    }
  </div>);

  smithy::mount(Box::new(app_2), root_element);
}
