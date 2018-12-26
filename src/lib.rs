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

use futures::{
  Future,
  Map,
};
use smithy::{
  smd,
  types::Component,
  UnwrappedPromise,
};
use std::{
  cell::RefCell,
  rc::Rc,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{
  future_to_promise,
  JsFuture,
};

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
  pub current_page: Page,
  pub promise: Rc<RefCell<UnwrappedPromise<i32, ()>>>,
  // pub future: Box<dyn Future<Item = (), Error = ()>>,
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
    // if let Some(user_id) = get_current_user_id_from_hash() {
    let future = smithy::future_from_timeout(300).map(|_| 3);
    let unwrapped_promise = UnwrappedPromise::from_future(future);
    // let data = Rc::new(RefCell::new(UnwrappedPromise::Pending));
    // let data_1 = data.clone();

    // let future = Box::new(
    //   future
    //     .map(move |s| {
    //       log_1(&JsValue::from_str("future cb"));
    //       *data_1.borrow_mut() = UnwrappedPromise::Success(s);
    //       smithy::rerender();
    //       JsValue::NULL
    //     })
    //     .map_err(|_| JsValue::NULL),
    // );
    // let future = future_to_promise(future);
    // std::mem::forget(future);

    RouterState {
      current_page: Page::UserDetailView(0),
      // promise: UnwrappedPromise::from_future(smithy::future_from_timeout(300)),
      // promise: Box::new(smithy::future_from_timeout(1000)),
      promise: unwrapped_promise,
      // future,
    }
    // } else {
    // RouterState {
    //   current_page: Page::Home,
    //   promise: smithy::promise_from_timeout(1_000),
    // }
    // }
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
            <li><a href="#1">user id 1 byah</a></li>
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
    {
      match *app_state.promise.borrow() {
        UnwrappedPromise::Pending => "pending",
        UnwrappedPromise::Success(ref s) => {
          // s.as_string().unwrap(),
          log_1(&JsValue::from_str(&format!("{}", s)));
          "success"
        }
        _ => "err"
      }
    }
  );

  smithy::mount(Box::new(app_2), root_element);
}
