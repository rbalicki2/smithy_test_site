#![feature(proc_macro_hygiene, slice_patterns)]

use wasm_bindgen::prelude::*;

use js_sys::{
  global,
  Object,
};
use std::mem::transmute;
use web_sys::{
  console::log_1,
  Document,
  Element,
  Window,
};

use smithy::{
  smd,
  types::Component,
};
use wasm_bindgen::JsValue;

mod next_tick;

fn get_window() -> Window {
  unsafe { transmute::<Object, Window>(global()) }
}

fn get_document() -> Document {
  get_window().document().unwrap()
}

pub fn log(s: &String) {
  log_1(&JsValue::from_str(s));
}

enum Page {
  Home,
  UserDetailView(i32),
}

struct RouterState {
  current_page: Page,
}

fn get_current_user_id_from_hash() -> Option<i32> {
  get_window()
    .location()
    .hash()
    .ok()
    .map(|hash_with_hash| hash_with_hash.chars().skip(1).collect::<String>())
    .and_then(|hash| hash.parse::<i32>().ok())
}

impl RouterState {
  pub fn handle_hash_change(&mut self) {
    if let Some(user_id) = get_current_user_id_from_hash() {
      self.current_page = Page::UserDetailView(user_id);
    } else {
      self.current_page = Page::Home;
    }
  }

  pub fn new() -> RouterState {
    if let Some(user_id) = get_current_user_id_from_hash() {
      RouterState {
        current_page: Page::UserDetailView(user_id),
      }
    } else {
      RouterState {
        current_page: Page::Home,
      }
    }
  }
}

#[wasm_bindgen]
pub fn start(div_id: String) {
  let doc: Document = get_document();
  let root_element: Element = doc.get_element_by_id(&div_id).unwrap();

  let mut app_state = RouterState::new();

  let app_2 = smd!(
    on_hash_change={|_| {
      app_state.handle_hash_change();
    }};
    {
      match app_state.current_page {
        Page::Home => smd!(<div>
          home
          <ul>
            <li><a href="#1">user id 1</a></li>
          </ul>
          <ul>
            <li><a href="#2">user id 2</a></li>
          </ul>
        </div>),
        Page::UserDetailView(id) => smd!(<div>
          user detail view id = { format!("{}", id) }
          <hr />
          <a href="#">Go home</a>
        </div>),
      }
    }
  );

  smithy::mount(Box::new(app_2), root_element);
}
