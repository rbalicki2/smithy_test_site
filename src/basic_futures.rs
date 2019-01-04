use futures::Future;
use js_sys::Promise;
use wasm_bindgen::{
  closure::Closure,
  JsCast,
  JsValue,
};
use wasm_bindgen_futures::JsFuture;

pub fn future_from_timeout(duration: i32) -> impl Future<Item = (), Error = ()> {
  // TODO figure out how to make a future from a setTimeout directly without
  // going through a promise
  let promise = promise_from_timeout(duration);

  JsFuture::from(promise).map(|_| ()).map_err(|_| ())
}

pub fn promise_from_timeout(duration: i32) -> Promise {
  let mut promise_closure = move |resolve: js_sys::Function, _reject| {
    let timeout_closure = Closure::wrap(Box::new(move || {
      let _ = resolve.call0(&JsValue::NULL);
    }) as Box<FnMut()>);
    let window = web_sys::window().unwrap();

    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
      timeout_closure.as_ref().unchecked_ref(),
      duration,
    );
    timeout_closure.forget();
  };
  let promise = Promise::new(&mut promise_closure);
  promise
}
