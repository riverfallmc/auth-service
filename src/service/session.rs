#![allow(dead_code)]

use axum::{http::StatusCode, Json};
use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, HttpResult, NonJsonHttpResult}};
use crate::{models::{Session, SessionCreate, User}, repository::session::SessionRepository, service::jwt::JWTService};
use super::time::TimeService;

pub struct SessionService;

impl SessionService {
  // создает запись сессии
  pub fn create(
    db: &mut Database<Postgres>,
    user: User,
    user_agent: &str
  ) -> HttpResult<Session> {
    let session = SessionCreate {
      user_id: user.id,
      useragent: user_agent.to_owned(),
      jwt: JWTService::generate(user.id)?,
      refresh_token: JWTService::generate_refresh(user.id)?,
      last_activity: TimeService::get_current_time(),
    };

    Ok(Json(SessionRepository::add(db, session)?))
  }

  // обновляет jwt сессии
  pub fn update(
    db: &mut Database<Postgres>,
    session_id: i32,
    jwt: &str
  ) -> NonJsonHttpResult<()> {
    Ok(SessionRepository::update_jwt(db, session_id, jwt.to_string())?)
  }

  // ищет сессию по jwt
  pub fn get_by_jwt(
    db: &mut Database<Postgres>,
    jwt: String,
    check_active: bool
  ) -> HttpResult<Session> {
    let session = SessionRepository::find_by_jwt(db, jwt.clone())?;

    if check_active && (!session.is_active || JWTService::is_active(jwt).is_err()) {
      return Err(HttpError::new("Сессия не была найдена", Some(StatusCode::BAD_REQUEST)));
    }

    Ok(Json(session))
  }

  // ищет сессию по refresh токену
  pub fn get_by_refresh(
    db: &mut Database<Postgres>,
    refresh: String,
    check_active: bool
  ) -> HttpResult<Session> {
    let session = SessionRepository::find_by_refresh(db, refresh.clone())?;

    if check_active && (!session.is_active || JWTService::is_active(refresh).is_err()) {
      return Err(HttpError::new("Сессия не была найдена", Some(StatusCode::BAD_REQUEST)));
    }

    Ok(Json(session))
  }

  pub fn delete(
    db: &mut Database<Postgres>,
    id: i32
  ) -> NonJsonHttpResult<()> {
    Ok(SessionRepository::delete(db, id)?)
  }

  // ищет сессию по user_id и useragent
  // и если её нет, то создает и возвращает её
  pub fn get(
    db: &mut Database<Postgres>,
    user: User,
    user_agent: &str,
  ) -> HttpResult<Session> {
    let session_result = SessionRepository::get(db, user.id, user_agent);

    match session_result {
      Ok(mut session) => {
        if JWTService::is_active(session.refresh_token.clone()).is_err() {
          Self::delete(db, session.id)?;
          return Self::create(db, user, user_agent);
        }

        if JWTService::is_active(session.jwt.clone()).is_err() {
          let new_jwt = JWTService::generate(session.user_id)?;

          SessionService::update(db, session.id, &new_jwt)?;

          session.jwt = new_jwt;
        }

        Ok(Json(session))
      },
      Err(diesel::NotFound) => Self::create(db, user, user_agent),
      Err(e) => Err(e.into()),
    }
  }
}