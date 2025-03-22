#![allow(dead_code)]

use anyhow::{anyhow, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, NonJsonHttpResult}};
use reqwest::StatusCode;
use crate::{models::{User, UserAdd, UserPasswordUpdate}, schema::users};

use super::user::UserRepository;

const USER_EXISTS: &str = "Пользователь уже существует";

pub struct AuthRepository;

impl AuthRepository {
  pub fn add(
    db: &mut Database<Postgres>,
    user: &UserAdd
  ) -> diesel::result::QueryResult<usize> {
    diesel::insert_into(users::table)
      .values(user)
      .execute(db)
  }

  pub fn find(
    db: &mut Database<Postgres>,
    id: i32
  ) -> Result<User> {
    users::table
      .filter(users::columns::id.eq(id))
      .first::<User>(db)
      .map_err(|_| anyhow!("Пользователь не был найден"))
  }

  pub fn find_by_username(
    db: &mut Database<Postgres>,
    username: &String
  ) -> Result<User> {
    users::table
      .filter(users::columns::username.eq(username))
      .first::<User>(db)
      .map_err(|_| anyhow!("Пользователь не был найден"))
  }

  pub async fn check_userdata_taken(
    db: &mut Database<Postgres>,
    username: &str,
    email: &str
  ) -> NonJsonHttpResult<()> {
    // ищем в бд сервиса
    let user = users::table
      .filter(users::username.eq(username))
      .first::<User>(db);

    // если юзера нет в бд, то ищем занята ли почта
    if user.is_err() {
      let user = UserRepository::find_by_email(email)
        .await;

      // ну типо все заебис и почта свободна
      if user.is_err() {
        return Ok(())
      }
    }

    Err(HttpError::new(USER_EXISTS, Some(StatusCode::BAD_REQUEST)))
  }

  pub fn update(
    db: &mut Database<Postgres>,
    id: i32,
    data: UserPasswordUpdate
  ) -> Result<()> {
    diesel::update(users::table.filter(users::id.eq(id)))
      .set((
        users::columns::salt.eq(data.salt),
        users::columns::password.eq(data.password)
      ))
      .execute(db)?;

    Ok(())
  }

  pub fn update_totp(
    db: &mut Database<Postgres>,
    id: i32,
    secret: String
  ) -> Result<()> {
    diesel::update(users::table.filter(users::columns::id.eq(id)))
      .set(users::columns::totp_secret.eq(secret))
      .execute(db)?;

    Ok(())
  }
}