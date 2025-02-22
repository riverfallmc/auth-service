#![allow(dead_code)]

use anyhow::Result;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dixxxie::database::{postgres::Postgres, Database};
use crate::{models::{Session, SessionCreate, SessionUpdateJwt}, schema::sessions};

pub struct SessionRepository;

impl SessionRepository {
  pub fn update_jwt(
    db: &mut Database<Postgres>,
    session_id: i32,
    jwt: String
  ) -> Result<()> {
    diesel::update(sessions::table.filter(sessions::columns::id.eq(session_id)))
      .set(SessionUpdateJwt {jwt})
      .execute(db)?;

    Ok(())
  }

  pub fn update(
    db: &mut Database<Postgres>,
    session_id: i32,
    jwt: String
  ) -> Result<Session> {
    Ok(diesel::update(sessions::table.filter(sessions::columns::id.eq(session_id)))
      .set(SessionUpdateJwt {jwt})
      .get_result::<Session>(db)?)
  }

  pub fn add(
    db: &mut Database<Postgres>,
    session: SessionCreate
  ) -> Result<Session> {
    Ok(diesel::insert_into(sessions::table)
      .values(&session)
      .get_result::<Session>(db)?)
  }

  pub fn find_by_refresh(
    db: &mut Database<Postgres>,
    refresh: String
  ) -> Result<Session> {
    Ok(sessions::table
      .filter(sessions::columns::refresh_token.eq(refresh))
      .first::<Session>(db)?)
  }

  pub fn find_by_jwt(
    db: &mut Database<Postgres>,
    jwt: String
  ) -> Result<Session> {
    Ok(sessions::table
      .filter(sessions::columns::jwt.eq(jwt))
      .first::<Session>(db)?)
  }

  pub fn delete(
    db: &mut Database<Postgres>,
    id: i32
  ) -> Result<()> {
    diesel::update(sessions::table.filter(sessions::id.eq(id)))
      .set(sessions::is_active.eq(false))
      .execute(db)?;

    Ok(())
  }

  pub fn get(
    db: &mut Database<Postgres>,
    user_id: i32,
    user_agent: &str,
  ) -> diesel::result::QueryResult<Session> {
    let session = sessions::table
      .filter(sessions::user_id.eq(user_id))
      .filter(sessions::useragent.eq(user_agent))
      .filter(sessions::is_active.eq(true))
      .first::<Session>(db)?;

    Ok(session)
  }
}