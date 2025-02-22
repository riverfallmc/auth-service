#![allow(dead_code)]

use anyhow::{anyhow, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dixxxie::database::{postgres::Postgres, Database};
use crate::{models::{User, UserAdd, UserPasswordUpdate}, schema::users};

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