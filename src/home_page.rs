use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};

pub struct UserInfo {
  pub id: i32,
  pub name: String,
  pub navigate_to_user_profile: Box<Fn()>,
}

pub fn home_page(user_infos: &mut Vec<UserInfo>) -> SmithyComponent {
  smd!(
    Welcome to home_page
    {
      user_infos.iter().map(|user_info: &UserInfo| {
        smd!(<div on_click={|_| (user_info.navigate_to_user_profile)()}>
          user id={ format!("{} ", user_info.id) } - name={ format!("{}", user_info.name) }
        </div>)
      }).collect::<Vec<SmithyComponent>>()
    }
  )
}
