use crate::common::JsonPayload;
use crate::repository::user::{get_all_users, get_user_by_username, get_followers_by_username, User, UserFetchError};
use serde::Serialize;

#[derive(Responder)]
pub enum UserListResponder {
  #[response(status = 200, content_type = "json")]
  Successful(JsonPayload<Vec<UserView>>),

  #[response(status = 500)]
  InternalError(JsonPayload<Vec<UserView>>),
}

#[get("/")]
pub fn index() -> UserListResponder {
  match get_all_users() {
    Ok(users) => UserListResponder::Successful(JsonPayload::from_result(
      users.into_iter().map(|user| user.into()).collect(),
    )),
    Err(_) => UserListResponder::InternalError(JsonPayload::from_error_message(
      "Something is wrong.".to_string(),
    )),
  }
}

#[derive(Responder)]
pub enum UserResponder {
  #[response(status = 200, content_type = "json")]
  Successful(JsonPayload<UserView>),
  
  #[response(status = 404)]
  NotFound(JsonPayload<UserView>),

  #[response(status = 500)]
  InternalError(JsonPayload<UserView>),
}

#[get("/<username>")]
pub fn get(username: String) -> UserResponder {
  print!("{}", username);

  match get_user_by_username(&username) {
    Ok(user) => UserResponder::Successful(JsonPayload::from_result(user.into())),
    Err(UserFetchError::UserNotFound(username)) => UserResponder::NotFound(JsonPayload::from_error_message(format!(
      "username {} is not found.",
      username
    ))),
    Err(_) => UserResponder::InternalError(JsonPayload::from_error_message(
      "Something is wrong.".to_string(),
    )),
  }
}

#[derive(Responder)]
pub enum UserFollowerListResponder {
  #[response(status = 200, content_type = "json")]
  Successful(JsonPayload<Vec<UserView>>),

  #[response(status = 404)]
  NotFound(JsonPayload<Vec<UserView>>),

  #[response(status = 500)]
  InternalError(JsonPayload<Vec<UserView>>),
}

#[get("/<username>/followers")]
pub fn follower_index(username: String) -> UserFollowerListResponder {
  match get_followers_by_username(&username) {
    Ok(users) => UserFollowerListResponder::Successful(JsonPayload::from_result(
      users.into_iter().map(|user| user.into()).collect(),
    )),
    Err(_) => UserFollowerListResponder::InternalError(JsonPayload::from_error_message(
      "Something is wrong.".to_string(),
    )),
  }
}

#[derive(Clone, Serialize)]
pub struct UserView {
  pub username: String,
  pub name: String,
  pub email: String,
}

impl From<User> for UserView {
  fn from(user: User) -> Self {
    UserView {
      username: user.username,
      name: user.name,
      email: user.email,
    }
  }
}
