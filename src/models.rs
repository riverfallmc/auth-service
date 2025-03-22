#![allow(dead_code)]

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{sessions, users};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  #[diesel(sql_type = Integer)]
  pub id: i32,
  #[diesel(sql_type = Integer)]
  pub user_id: i32,
  #[diesel(sql_type = Text)]
  pub username: String,
  #[diesel(sql_type = Text)]
  pub password: String,
  #[diesel(sql_type = Text)]
  pub salt: String,
  #[diesel(sql_type = Nullable<String>)]
  pub totp_secret: Option<String>,
  #[diesel(sql_type = Nullable<Jsonb>)]
  pub backup_codes: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UserInUserService {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub friends: Vec<i32>,
  pub rank: String,
  pub registered_at: NaiveDateTime
}

#[derive(Serialize)]
pub struct UserPasswordUpdate {
  pub salt: String,
  pub password: String
}

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = users)]
pub struct UserAdd {
  /// Айди пользователя из сервиса user
  pub user_id: Option<i32>,
  pub username: String,
  pub password: String,
  pub salt: String
}

#[derive(Serialize, Deserialize)]
pub struct UserCreate {
  pub username: String,
  pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct BaseUserInfo {
  pub id: i32,
  pub username: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRegister {
  pub username: String,
  pub password: String,
  pub email: String,
  // значение этого поля в любом случае будет стёрто
  // при выполнении запроса на эндпоинт /register
  pub salt: Option<String>
}

impl From<UserRegister> for UserCreate {
  fn from(value: UserRegister) -> Self {
    UserCreate {
      username: value.username,
      email: value.email,
    }
  }
}

impl From<UserRegister> for UserAdd {
  fn from(value: UserRegister) -> Self {
    UserAdd {
      user_id: None,
      username: value.username,
      password: value.password,
      // использовать unwrap в этом случае - не безответственно
      // поток из-за него не свалится))
      //
      // потому-что нет сценариев, при которых
      // salt равнялся бы None
      //
      // в любом случае, использовать unwrap
      // не круто
      salt: value.salt.unwrap()
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
  pub username: String,
  pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginedData {
  pub username: String,
  /// jwt
  pub token: String
}

// Sessions

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
  #[diesel(sql_type = Integer)]
  pub id: i32,
  #[diesel(sql_type = Integer)]
  pub user_id: i32,
  #[diesel(sql_type = Integer)]
  pub global_id: i32,
  #[diesel(sql_type = Text)]
  pub useragent: String,
  #[diesel(sql_type = Text)]
  pub jwt: String,
  #[diesel(sql_type = Text)]
  pub refresh_token: String,
  #[diesel(sql_type = Boolean)]
  pub is_active: bool,
  #[diesel(sql_type = Timestamp)]
  pub last_activity: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionCreate {
  #[diesel(sql_type = Integer)]
  pub user_id: i32,
  #[diesel(sql_type = Integer)]
  pub global_id: i32,
  #[diesel(sql_type = Text)]
  pub useragent: String,
  #[diesel(sql_type = Text)]
  pub jwt: String,
  #[diesel(sql_type = Text)]
  pub refresh_token: String,
  #[diesel(sql_type = Timestamp)]
  pub last_activity: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionUpdateJwt {
  #[diesel(sql_type = Text)]
  pub jwt: String,
}