#![allow(dead_code)]

use axum::Json;
use crate::{models::{BaseUserInfo, Session, UserLogin}, repository::{auth::AuthRepository, session::SessionRepository}, service::jwt::JWTService};
use super::{hasher::HasherService, logic::tfa::TFAService, session::SessionService};
use dixxxie::{connection::{DbPooled, RedisPooled}, response::{HttpError, HttpResult}};
use reqwest::StatusCode;

pub struct AuthService;

impl AuthService {
  pub fn get_owner(
    db: &mut DbPooled,
    token: String
  ) -> HttpResult<Json<BaseUserInfo>> {
    let session = SessionService::get_by_jwt(db, token, true)?;
    let user_id = session.user_id;
    let user = AuthRepository::find(db, user_id)?;

    Ok(Json(BaseUserInfo {
      id: user_id,
      username: user.username
    }))
  }

  // авторизация
  pub async fn login(
    redis: &mut RedisPooled,
    db: &mut DbPooled,
    credentials: UserLogin,
    user_agent: &str
  ) -> HttpResult<Json<serde_json::Value>> {
    // ищем юзера по нику
    let user = AuthRepository::find_by_username(db, &credentials.username)?;
    let password = HasherService::sha256(credentials.password + &user.salt);

    // проверяем пароль на валидность
    if user.password != password {
      return Err(HttpError::new("Неверный пароль!", Some(StatusCode::UNAUTHORIZED)));
    }

    // если у игрока привязан 2fa
    // то добавляем в редис запись на 3 минуты
    // и ждем пока игрок авторизируется
    if user.totp_secret.is_some() {
      let data = TFAService::add_login_attempt(redis, user.username)?;

      return Ok(Json(serde_json::to_value(data.0)?));
    }

    // создаем сессию в любом случае
    let session = SessionService::get(db, user, user_agent)?;

    Ok(Json(serde_json::to_value(session)?))
  }

  // обновляет и возвращает jwt
  // с помощью refresh токена
  pub async fn refresh(
    db: &mut DbPooled,
    refresh_token: String
  ) -> HttpResult<Json<Session>> {
    let session = SessionService::get_by_refresh(db, refresh_token, true)?;
    let token = JWTService::generate(session.user_id)?;

    Ok(Json(SessionRepository::update(db, session.id, token)?))
  }
}