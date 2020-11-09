use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use serde::Deserialize;

pub fn get_all_users<'a>() -> Result<Vec<User>, UserFetchError<'a>> {
  let json = include_str!("../../data/users.json");

  match serde_json::from_str::<Vec<User>>(json) {
    Ok(users) => Ok(users),
    Err(_) => Err(UserFetchError::DataJsonLoadingFailed()),
  }
}

pub fn get_user_by_username(username: &str) -> Result<User, UserFetchError> {
  match get_all_users() {
    Ok(users) => match users.into_iter().find(|user| user.username == username) {
      Some(user) => Ok(user),
      _ => Err(UserFetchError::UserNotFound(username)),
    },
    _ => Err(UserFetchError::DataJsonLoadingFailed()),
  }
}

pub fn get_followers_by_username(username: &str) -> Result<Vec<User>, UserFetchError> {
  let relationships_json = include_str!("../../data/user_relationships.json");

  match (get_all_users(), get_user_by_username(username), serde_json::from_str::<HashMap<String, UserRelationshipShape>>(relationships_json)) {
    (Ok(users), Ok(user), Ok(user_relationships)) => {
      let mut vec: Vec<User> = vec![];

      let follower_usernames = match user_relationships.get(&user.username) {
        Some(rel) => HashSet::from_iter(rel.followers.iter().cloned()),
        None => HashSet::new(),
      };

      for u in users {
        if follower_usernames.contains(&u.username) {
          vec.push(u);
        }
      }

      Ok(vec)
    },
    (Ok(_), Err(_), Ok(_)) => Err(UserFetchError::UserNotFound(username)),
    _ => Err(UserFetchError::DataJsonLoadingFailed()),
  }
}

#[derive(Deserialize)]
pub struct User {
  pub username: String,
  pub name: String,
  pub email: String,
}

pub enum UserFetchError<'a> {
  DataJsonLoadingFailed(),

  UserNotFound(&'a str),
}

#[derive(Deserialize)]
pub struct UserRelationshipShape {
  pub followers: Vec<String>,
}
