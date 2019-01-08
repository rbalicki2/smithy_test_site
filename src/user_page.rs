use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};
use web_sys::MouseEvent;

pub fn user_page<'a>(
  user_info: &'a crate::UserInfo,
  mut navigate_home: impl FnMut() + 'a,
) -> SmithyComponent<'a> {
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
  smd!(<div>user not found</div>)
}
