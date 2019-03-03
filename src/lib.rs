#![feature(proc_macro_hygiene, slice_patterns, custom_attribute)]

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
  types::{
    Component,
    DomRef,
    PromiseState,
    UnwrappedPromise,
  },
};
use wasm_bindgen::{
  JsCast,
  JsValue,
};

mod fetch_posts;
use self::fetch_posts::{
  fetch_posts,
  Post,
};

// mod home_page;
mod input;
// mod user_page;

pub struct UserInfo {
  pub id: i32,
  pub name: String,
}

fn get_window() -> Window {
  unsafe { transmute::<Object, Window>(global()) }
}

fn get_document() -> Document {
  get_window().document().unwrap()
}

pub fn log(s: &String) {
  log_1(&JsValue::from_str(s));
}

#[derive(Copy, Clone)]
enum Page {
  Home,
  UserDetailView(i32),
}

impl Page {
  pub fn set(&mut self, page: Page) {
    *self = page;
    let _ = get_window().location().set_hash(&self.get_hash());
  }

  fn get_hash(&self) -> String {
    match self {
      Page::Home => "".into(),
      Page::UserDetailView(id) => id.to_string(),
    }
  }
}

struct RouterState {
  pub current_page: Page,
  pub user_list: Vec<UserInfo>,
  pub unwrapped_posts: UnwrappedPromise<Post, ()>,
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
    let posts_future = fetch_posts(1);
    let unwrapped_posts = smithy::unwrapped_promise_from_future(posts_future);

    let current_page = if let Some(user_id) = get_current_user_id_from_hash() {
      Page::UserDetailView(user_id)
    } else {
      Page::Home
    };

    RouterState {
      current_page,
      unwrapped_posts,
      user_list: vec![
        UserInfo {
          id: 1,
          name: "Robert".into(),
        },
        UserInfo {
          id: 2,
          name: "Kerry".into(),
        },
      ],
    }
  }
}

#[wasm_bindgen]
pub fn start(div_id: String) {
  let doc: Document = get_document();
  let root_element: Element = doc.get_element_by_id(&div_id).unwrap();

  // let mut app_state = RouterState::new();
  // let app_2 = smd!(
  //   on_hash_change={|_| {
  //     web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("outer hash change"));
  //     app_state.handle_hash_change();
  //   }};
  //   post_render={|_| {
  //     web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("outer post render"));
  //   }};
  //   {
  //     // TODO figure out a way to avoid cloning current_page
  //     let current_page_for_match = app_state.current_page.clone();
  //     let user_list = &app_state.user_list;
  //     let current_page = &mut app_state.current_page;
  //     match current_page_for_match {
  //       Page::Home => home_page::home_page(
  //         &user_list,
  //         move |id| {
  //           current_page.set(Page::UserDetailView(id));
  //         },
  //       ),
  //       Page::UserDetailView(id) => {
  //         if let Some(ref user_info) = app_state.user_list.iter().find(
  //           |item| item.id == id
  //         ) {
  //           user_page::user_page(
  //             user_info,
  //             move || current_page.set(Page::Home)
  //           )
  //         } else {
  //           user_page::user_not_found_page()
  //         }
  //       }
  //     }
  //   }
  //   <div>
  //     <h1>Fetching post like:</h1>
  //     {
  //       match *(*app_state.unwrapped_posts).borrow() {
  //         PromiseState::Pending => smd!(
  //           <div>
  //             pending
  //           </div>
  //         ),
  //         PromiseState::Success(ref post) => {
  //           smd!(<div>fetched a post with title <b>{ &post.title }</b></div>)
  //         },
  //         PromiseState::Error(_) => smd!(<div>
  //           error
  //         </div>),
  //       }
  //     }
  //   </div>
  // );

  let mut outer_input_str = "hello".to_string();

  let mut dom_ref: DomRef = DomRef::new();
  let mut dom_ref_outer_2: DomRef = DomRef::new();

  let mut inner_input_str = "inner".to_string();
  let inner_input_str = std::rc::Rc::new(std::cell::RefCell::new(inner_input_str));
  let inner_1 = inner_input_str.clone();
  let inner_2 = inner_input_str.clone();
  let mut inner_input = input::render_3(inner_2);

  let app_2 = smd!(
    post_render={|node_list: &Vec<web_sys::Node>| {
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("outer post render in input {} 2:{}", dom_ref.get().is_some(), dom_ref_outer_2.get().is_some())));
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("inner input str val {}", inner_1.borrow())));

      if let Some(el) = dom_ref.get() {
        let el: &web_sys::HtmlInputElement = el.unchecked_ref();
        el.set_value(&outer_input_str);
      }
    }};
    <input
      ref={&mut dom_ref}
      value={(&outer_input_str).to_string()}
      on_input={|e: &web_sys::InputEvent| {
        let target = e.target().unwrap();
        let target: web_sys::HtmlInputElement = target.unchecked_into();
        outer_input_str = target.value().chars().take(10).collect();
      }}
    />
    <div ref={&mut dom_ref_outer_2} />
    { &mut inner_input }
  );

  smithy::mount(Box::new(app_2), root_element);
}
