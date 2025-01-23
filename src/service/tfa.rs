// TFA - Two Factor Auth
#![allow(dead_code)]

use axum::Json;
use dixxxie::{connection::{DbPooled, RedisPooled}, response::{HttpMessage, HttpResult}};
use serde::{Deserialize, Serialize};
use totp_rs::TOTP;
use crate::{models::User, repository::auth::AuthRepository};
use super::{hasher::HasherService, redis::RedisService};

pub struct TFAService;

#[derive(Serialize, Deserialize)]
pub struct TwoFactorRedisData {
  pub id: i32,
  pub code: String
}

#[derive(Serialize, Deserialize)]
pub struct TwoFactorResponse {
  pub secret: String,
  pub qr: String
}

impl TFAService {
  fn generate_redis_2fa_key(
    key: String
  ) -> String {
    format!("2fa:{}", key)
  }

  // добавляет в редис
  pub fn add_login_attempt(
    redis: &mut RedisPooled,
    username: String
  ) -> HttpResult<Json<HttpMessage>> {
    let redis_key = Self::generate_redis_2fa_key(username);

    RedisService::set_temporarily(redis, &redis_key, 1, 5)?;

    Ok(Json(HttpMessage::new("Подтвердите вход с помощью TOTP кода (2FA).")))
  }

  pub fn get_login_attempt(
    redis: &mut RedisPooled,
    db: &mut DbPooled,
    username: String
  ) -> HttpResult<User> {
    let redis_key = Self::generate_redis_2fa_key(username.clone());

    RedisService::get::<String>(redis, &redis_key)?;

    AuthRepository::find_by_username(db, &username)
  }

  pub fn remove_login_attempt(
    redis: &mut RedisPooled,
    username: String
  ) -> HttpResult<()> {
    let redis_key = Self::generate_redis_2fa_key(username);

    RedisService::remove(redis, &redis_key)
  }

  pub fn generate_2fa(
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

    Ok((secret, totp))
  }
}