use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};
use std::{
  cell::RefCell,
  rc::Rc,
};

pub struct UserInfo {
  pub id: i32,
  pub name: String,
  // pub navigate_to_user_profile: Box<Fn()>,
}

fn clone_many_times<T>(cell: &T, count: usize) -> Vec<T>
where
  T: Clone,
{
  let mut vec = Vec::with_capacity(count);
  for _ in 0..count {
    vec.push(cell.clone());
  }
  vec
}

pub fn home_page<'a>(
  user_infos: &'a Vec<UserInfo>,
  navigate_to_user_profile: impl FnMut(i32) -> () + 'a,
) -> SmithyComponent<'a> {
  let navigate_to_user_profile = {
    let fn_once = std::cell::RefCell::new(Some(navigate_to_user_profile));
    move |id| (fn_once.borrow_mut().take().unwrap())(id)
  };

  let mut navigate_cell = Rc::new(RefCell::new(navigate_to_user_profile));
  let navigate_vec = clone_many_times(&mut navigate_cell, user_infos.len());

  let zipped_iter = user_infos.iter().zip(navigate_vec.into_iter());
  let mut inner = zipped_iter
    .map(|(user_info, cb)| {
      smd!(<div
        on_click={|_| {
          cb.borrow()(user_info.id);
        }}
      >
        user id={ format!("{} ", user_info.id) } - name={ format!("{}", user_info.name) }
      </div>)
    })
    .collect::<Vec<SmithyComponent>>();

  smd!(
    Welcome to home_page
    { &mut inner }
  )
}
