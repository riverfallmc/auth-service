#![allow(dead_code)]

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dixxxie::{connection::DbPooled, response::HttpResult};
use crate::{models::SessionUpdateJwt, schema::sessions};

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
}