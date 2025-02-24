use crate::{controller::tfa::TFAAddBody, models::User, repository::auth::AuthRepository, service::{hasher::HasherService, jwt::JWTService, redis::RedisService, session::SessionService }};
use adjust::{database::{postgres::Postgres, redis::Redis, Database}, response::{HttpError, HttpMessage, HttpResult, NonJsonHttpResult}};
use serde::{Deserialize, Serialize};
use reqwest::StatusCode;
use totp_rs::TOTP;
use axum::Json;

#[derive(Serialize, Deserialize)]
pub struct TwoFactorResponse {
  pub secret: String,
  pub qr: String
}

pub struct TFAService;

impl TFAService {
  /// Генерирует TFA Secret
  pub fn add(
    db: &mut Database<Postgres>,
    token: String
  ) -> HttpResult<TFAAddBody> {
    let id = JWTService::is_active(token)?;

    // TODO @ Въебашить редис

    let user = AuthRepository::find(db, id.parse()?)?;

    if user.totp_secret.is_some() {
      return Err(HttpError::new("К вашему аккаунту уже привязана двуфакторная аутентификация!", Some(StatusCode::BAD_REQUEST)))
    }

    let axum::Json((secret, totp)) = Self::generate_2fa(user.username, None)?;

    Ok(Json(TFAAddBody {
      secret,
      qr: format!("data:image/png;base64,{}", totp.get_qr_base64().unwrap_or_default())
    }))
  }

  /// Привязка 2FA к профилю
  pub fn link(
    db: &mut Database<Postgres>,
    token: String,
    code: String,
    secret: String
  ) -> HttpResult<HttpMessage> {
    let id = JWTService::is_active(token)?;
    let user = AuthRepository::find(db, id.parse()?)?;

    let axum::Json((_, totp)) = TFAService::generate_2fa(user.username.clone(), Some(secret.clone()))?;

    if !totp.check_current(&code)? {
      return Err(HttpError::new("Неверный код", Some(StatusCode::UNAUTHORIZED)));
    }

    AuthRepository::update_totp(db, user.id, secret)?;

    Ok(Json(HttpMessage::new("Двуфакторная аутентификация была привязана к вашему профилю")))
  }

  /// Вход в аккаунт
  pub async fn login(
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
    username: String,
    code: String,
    user_agent: String
  ) -> HttpResult<crate::models::Session> {
    let user = TFAService::get_login_attempt(redis, db, username.clone())
      .map_err(|_| HttpError::new("Запрос на авторизацию не найден (возможно, вы не успели)", Some(StatusCode::UNAUTHORIZED)))?;

    let axum::Json((_, totp)) = TFAService::generate_2fa(user.username.clone(), user.totp_secret.clone())?;

    if !totp.check_current(&code)? {
      return Err(HttpError::new("Неверный код", Some(StatusCode::UNAUTHORIZED)));
    }

    TFAService::remove_login_attempt(redis, username)?;

    SessionService::get(db, (*user).clone(), &user_agent)
  }

  // Вспомогательные функции

  fn generate_redis_2fa_key(
    key: String
  ) -> String {
    format!("2fa:{}", key)
  }

  // добавляет в редис
  pub fn add_login_attempt(
    redis: &mut Database<Redis>,
    username: String
  ) -> HttpResult<HttpMessage> {
    let redis_key = Self::generate_redis_2fa_key(username);

    RedisService::set_temporarily(redis, &redis_key, 1, 5)?;

    Ok(Json(HttpMessage::new("Подтвердите вход с помощью TOTP кода (2FA).")))
  }

  fn get_login_attempt(
    redis: &mut Database<Redis>,
    db: &mut Database<Postgres>,
    username: String
  ) -> HttpResult<User> {
    let redis_key = Self::generate_redis_2fa_key(username.clone());

    RedisService::get::<String>(redis, &redis_key)?;

    Ok(Json(AuthRepository::find_by_username(db, &username)?))
  }

  fn remove_login_attempt(
    redis: &mut Database<Redis>,
    username: String
  ) -> NonJsonHttpResult<()> {
    let redis_key = Self::generate_redis_2fa_key(username);

    Ok(RedisService::remove(redis, &redis_key)?)
  }

  fn generate_2fa(
    username: String,
    secret: Option<String>
  ) -> HttpResult<(String, TOTP)> {
    let secret = secret.unwrap_or(HasherService::generate_2fa_secret());

    let totp = TOTP::new(
      totp_rs::Algorithm::SHA1,
      6,
      1,
      30,
      secret.as_bytes().to_vec(),
      Some(String::from("serenitymc.ru")),
      username
    )?;

    Ok(axum::Json((secret, totp)))
  }
}