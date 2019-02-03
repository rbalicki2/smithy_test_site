use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};
use web_sys::{
  MouseEvent,
  Window,
};

fn get_window() -> Window {
  web_sys::window().unwrap()
}

pub fn user_page<'a>(
  user_info: &'a crate::UserInfo,
  mut navigate_home: impl FnMut() + 'a,
) -> SmithyComponent<'a> {
  // smd!(<div id="user"><h1>hoioiuser</h1></div>)
  smd!(<div>
    looking at user profile
    <b>{ format!("{}", user_info.name) }</b>
    <div>
      <a
        href
        on_click={|e: &MouseEvent| {
          navigate_home();
          e.prevent_default();
        }}
      >
        Go home!
      </a>
    </div>
  </div>)
}

pub fn user_not_found_page<'a>() -> SmithyComponent<'a> {
  let x: Option<i32> = None;

  smd!(
    <div>user not found</div>
    { if get_window().get("foo").is_undefined() { smd!(<div>TL1</div>) } else { smd!(<div>TL1<span>NOFUCKINGWAY</span>pchah</div>) } }
    <span />
    <br />
    { if get_window().get("foo").is_undefined() { smd!(<div>TL2</div>) } else { smd!(<div />) } }
    { x }
    { "BYAH "}
  )
}
