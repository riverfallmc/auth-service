#![allow(dead_code)]
// use anyhow::Result;

use axum::http::StatusCode;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dixxxie::{connection::DbPooled, response::{HttpError, HttpResult}};

use crate::{models::Session, schema::sessions};

pub struct SessionService;

impl SessionService {
  // Ищет сессию по айди
  pub fn get_by_jwt(
    db: &mut DbPooled,
    jwt: String,
    check_active: bool
  ) -> HttpResult<Session> {
    let session = sessions::table
      .filter(sessions::columns::token.eq(jwt))
      .first::<Session>(db)?;

    if check_active && !session.is_active {
      return Err(HttpError::new("Сессия не была найдена", Some(StatusCode::BAD_REQUEST)));
    }

    Ok(session)
  }
}