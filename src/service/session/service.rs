#![allow(dead_code)]

use axum::http::StatusCode;
use chrono::{NaiveDateTime, Utc};
use dixxxie::{connection::DbPooled, response::{HttpError, HttpResult}};
use crate::{models::{Session, SessionCreate, User}, repository::session::SessionRepository, service::jwt::JWTService};

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

    SessionRepository::add(db, session)
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
    let session = SessionRepository::find_by_jwt(db, jwt)?;

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
    let session = SessionRepository::find_by_refresh(db, refresh)?;

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
  ) -> HttpResult<Session> {
    let session = SessionRepository::get(db, user.id, user_agent);

    match session {
      Ok(session) => Ok(session),
      Err(diesel::NotFound) => Self::create(db, user, user_agent),
      Err(e) => Err(e.into()),
    }
  }
}