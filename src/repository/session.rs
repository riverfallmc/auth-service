#![allow(dead_code)]

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dixxxie::{connection::DbPooled, response::HttpResult};
use crate::{models::{Session, SessionCreate, SessionUpdateJwt}, schema::sessions};

pub struct SessionRepository;

impl SessionRepository {
  pub fn update_jwt(
    db: &mut DbPooled,
    session_id: i32,
    jwt: String
  ) -> HttpResult<()> {
    diesel::update(sessions::table.filter(sessions::columns::id.eq(session_id)))
      .set(SessionUpdateJwt {jwt})
      .execute(db)?;

    Ok(())
  }

  pub fn update(
    db: &mut DbPooled,
    session_id: i32,
    jwt: String
  ) -> HttpResult<Session> {
    Ok(diesel::update(sessions::table.filter(sessions::columns::id.eq(session_id)))
      .set(SessionUpdateJwt {jwt})
      .get_result::<Session>(db)?)
  }

  pub fn add(
    db: &mut DbPooled,
    session: SessionCreate
  ) -> HttpResult<Session> {
    Ok(diesel::insert_into(sessions::table)
      .values(&session)
      .get_result::<Session>(db)?)
  }

  pub fn find_by_refresh(
    db: &mut DbPooled,
    refresh: String
  ) -> HttpResult<Session> {
    Ok(sessions::table
      .filter(sessions::columns::refresh_token.eq(refresh))
      .first::<Session>(db)?)
  }

  pub fn find_by_jwt(
    db: &mut DbPooled,
    jwt: String
  ) -> HttpResult<Session> {
    Ok(sessions::table
      .filter(sessions::columns::jwt.eq(jwt))
      .first::<Session>(db)?)
  }

  pub fn delete(
    db: &mut DbPooled,
    id: i32
  ) -> HttpResult<()> {
    diesel::update(sessions::table.filter(sessions::id.eq(id)))
      .set(sessions::is_active.eq(false))
      .execute(db)?;

    Ok(())
  }

  pub fn get(
    db: &mut DbPooled,
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