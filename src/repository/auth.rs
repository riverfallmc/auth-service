#![allow(dead_code)]

use axum::http::StatusCode;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dixxxie::{connection::DbPooled, response::{HttpResult, HttpError}};
use crate::{models::{User, UserAdd, UserPasswordUpdate}, schema::users};

pub struct AuthRepository;

impl AuthRepository {
  pub fn add(
    db: &mut DbPooled,
    user: &UserAdd
  ) -> diesel::result::QueryResult<usize> {
    diesel::insert_into(users::table)
      .values(user)
      .execute(db)
  }

  pub fn find(
    db: &mut DbPooled,
    id: i32
  ) -> HttpResult<User> {
    users::table
      .filter(users::columns::id.eq(id))
      .first::<User>(db)
      .map_err(|_| HttpError::new("Пользователь не был найден", Some(StatusCode::UNAUTHORIZED)))
  }

  pub fn find_by_username(
    db: &mut DbPooled,
    username: &String
  ) -> HttpResult<User> {
    users::table
      .filter(users::columns::username.eq(username))
      .first::<User>(db)
      .map_err(|_| HttpError::new("Пользователь не был найден", Some(StatusCode::UNAUTHORIZED)))
  }

  pub fn update(
    db: &mut DbPooled,
    id: i32,
    data: UserPasswordUpdate
  ) -> HttpResult<()> {
    diesel::update(users::table.filter(users::id.eq(id)))
      .set((
        users::columns::salt.eq(data.salt),
        users::columns::password.eq(data.password)
      ))
      .execute(db)?;

    Ok(())
  }

  pub fn update_totp(
    db: &mut DbPooled,
    id: i32,
    secret: String
  ) -> HttpResult<()> {
    diesel::update(users::table.filter(users::columns::id.eq(id)))
      .set(users::columns::totp_secret.eq(secret))
      .execute(db)?;

    Ok(())
  }
}