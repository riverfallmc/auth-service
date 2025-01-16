#![allow(dead_code)]
// use anyhow::Result;

use axum::http::StatusCode;
use chrono::{NaiveDateTime, Utc};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dixxxie::{connection::DbPooled, response::{HttpError, HttpResult}};
use crate::{models::{Session, SessionCreate, User}, repository::session::SessionRepository, schema::sessions, service::jwt::JWTService};

pub struct SessionService;

impl SessionService {
  // возвращает текущее время, не зависимое от временной зоны
  fn get_current_time() -> NaiveDateTime {
    Utc::now().naive_utc()
  }

  // создает запись сессии
  pub fn create(
    db: &mut DbPooled,
    user: User,
    user_agent: &str
  ) -> HttpResult<Session> {
    let session = SessionCreate {
      user_id: user.id,
      useragent: user_agent.to_owned(),
      jwt: JWTService::generate(user.id)?,
      refresh_token: JWTService::generate_refresh(user.id)?,
      last_activity: Self::get_current_time(),
    };

    Ok(diesel::insert_into(sessions::table)
      .values(&session)
      .get_result::<Session>(db)?)
  }

  // обновляет jwt сессии
  pub fn update(
    db: &mut DbPooled,
    session_id: i32,
    jwt: &str
  ) -> HttpResult<()> {
    SessionRepository::update_jwt(db, session_id, jwt.to_string())
  }

  // ищет сессию по jwt
  pub fn get_by_jwt(
    db: &mut DbPooled,
    jwt: String,
    check_active: bool
  ) -> HttpResult<Session> {
    let session = sessions::table
      .filter(sessions::columns::jwt.eq(jwt))
      .first::<Session>(db)?;

    if check_active && !session.is_active {
      return Err(HttpError::new("Сессия не была найдена", Some(StatusCode::BAD_REQUEST)));
    }

    Ok(session)
  }

  // ищет сессию по refresh токену
  pub fn get_by_refresh(
    db: &mut DbPooled,
    refresh: String,
    check_active: bool
  ) -> HttpResult<Session> {
    let session = sessions::table
      .filter(sessions::columns::refresh_token.eq(refresh))
      .first::<Session>(db)?;

    if check_active && !session.is_active {
      return Err(HttpError::new("Сессия не была найдена", Some(StatusCode::BAD_REQUEST)));
    }

    Ok(session)
  }

  // ищет сессию по user_id и useragent
  // и если её нет, то создает и возвращает её
  pub fn get(
    db: &mut DbPooled,
    user: User,
    user_agent: &str,
    check_active: bool,
    create_if_not_exists: bool
  ) -> HttpResult<Session> {
    let session = sessions::table
      .filter(sessions::columns::user_id.eq(user.id))
      .filter(sessions::columns::useragent.eq(user_agent))
      .first::<Session>(db)?; // todo @ я хз, если записи нет оно вернёт Err или Ok. Хуй знает - нужно проверить

    if check_active && !session.is_active {
      if create_if_not_exists {
        return Self::create(db, user, user_agent);
      }

      return Err(HttpError::new("Сессия не была найдена", Some(StatusCode::BAD_REQUEST)));
    }

    Ok(session)
  }
}