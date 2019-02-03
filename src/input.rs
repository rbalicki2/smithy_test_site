use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};
use web_sys::InputEvent;

pub fn render<'a>(
  value: String,
  mut on_change: impl FnMut(&InputEvent) -> () + 'a,
) -> SmithyComponent<'a> {
  smd!(<input
    type="text"
    value={value.clone()}
    on_input={&mut on_change}
  />)
}
