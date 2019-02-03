use futures::Future;
use js_sys::Promise;
// TODO figure out why these imports are being marked as unused
use serde_derive::{
  Deserialize,
  Serialize,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
  Request,
  RequestInit,
  RequestMode,
  Response,
};

const POST_URL: &'static str = "https://jsonplaceholder.typicode.com/posts/";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Post {
  pub body: String,
  pub id: i32,
  pub user_id: i32,
  pub title: String,
}

pub fn fetch_posts(id: i32) -> impl Future<Item = Post, Error = ()> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init(&format!("{}{}", POST_URL, id), &opts).unwrap();

  request.headers().set("Accept", "application/json").unwrap();

  let window = web_sys::window().unwrap();
  let request_promise = window.fetch_with_request(&request);

  let future = JsFuture::from(request_promise)
    .and_then(|resp_value| {
      // `resp_value` is a `Response` object.
      assert!(resp_value.is_instance_of::<Response>());
      let resp: Response = resp_value.dyn_into().unwrap();
      resp.json()
    })
    .and_then(|json_value: Promise| {
      // Convert this other `Promise` into a rust `Future`.
      JsFuture::from(json_value)
    })
    .map(|json| {
      // Use serde to parse the JSON into a struct.
      let post: Post = json.into_serde().unwrap();
      post
    })
    .map_err(|_| ());
  future
}
