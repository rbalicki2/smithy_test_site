use crate::Mutable;
use smithy::{
  smd,
  types::{
    Component,
    DomRef,
    SmithyComponent,
  },
};
use std::{
  cell::RefCell,
  rc::Rc,
};
use wasm_bindgen::JsCast;
use web_sys::InputEvent;

pub fn render<'a>(
  value: &'a String,
  mut on_change: impl FnMut(&String) -> () + 'a,
) -> SmithyComponent<'a> {
  smd!(
    post_render={|node_list: &Vec<web_sys::Node>| {
      // N.B. "node_list" here shadows a variable from above it! NOOOO
      // TODO figure out how to make opaque variable names in macros
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("inner post render in input {}", value)));
      for x in node_list.iter() {
        let el: &web_sys::HtmlInputElement = x.unchecked_ref();
        el.set_value(&value);
      }
    }};
    <input
      type="text"
      value={value.clone()}
      on_input={|e: &InputEvent| {
        let target = e.target().unwrap();
        let target: web_sys::HtmlInputElement = target.unchecked_into();
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("on input {} - {}", value, target.value())));
        let new_val = target.value().chars().take(10).collect();
        on_change(&new_val);
      }}
    />
  )
}

pub fn render_2<'a>(value: &'a mut impl Mutable<String>) -> SmithyComponent<'a> {
  let mut dom_ref_inner: DomRef = DomRef::new("inner".to_string());
  smd!(
    post_render={|node_list: &Vec<web_sys::Node>| {
      // N.B. "node_list" here shadows a variable from above it! NOOOO
      // TODO figure out how to make opaque variable names in macros
      // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("inner post render in input {:?}", value.get())));
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("XX-> inner input is some {}", dom_ref_inner.get().is_some())));
      if let Some(el) = dom_ref_inner.get() {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("found an inner dom_ref"));
        let el: &web_sys::HtmlInputElement = el.unchecked_ref();
        el.set_value(&value.get());
      }
    }};
    // fooooo
    <input
      ref={&mut dom_ref_inner}
      type="text"
      value={value.get().to_string()}
      on_input={|e: &InputEvent| {
        let target = e.target().unwrap();
        let target: web_sys::HtmlInputElement = target.unchecked_into();
        let new_val = target.value().chars().take(5).collect();
        value.set(new_val);
      }}
    />
    // this is some heavy shit
  )
}
