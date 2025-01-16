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
    key: Option<u64>
  ) -> (String, u64) {
    let key = key.unwrap_or(HasherService::generate_code());

    (format!("2fa:{}", key), key)
  }

  // добавляет в редис
  pub fn add_login_attempt(
    redis: &mut RedisPooled,
    user_id: i32
  ) -> HttpResult<Json<HttpMessage>> {
    let (redis_key, code) = Self::generate_redis_2fa_key(Some(user_id as u64));

    let data = TwoFactorRedisData {
      id: user_id,
      code: code.to_string()
    };

    let jsoned_2fa = serde_json::to_string(&data)?;

    RedisService::set_temporarily(redis, &redis_key, jsoned_2fa, 3)?;

    Ok(Json(HttpMessage::new("Подтвердите вход с помощью TOTP кода (2FA).")))
  }

  pub fn get_login_attempt(
    redis: &mut RedisPooled,
    db: &mut DbPooled,
    key: u64
  ) -> HttpResult<User> {
    let (redis_key, _) = Self::generate_redis_2fa_key(Some(key));
    let jsoned_data = RedisService::get::<String>(redis, &redis_key)?;
    let data = serde_json::from_str::<TwoFactorRedisData>(&jsoned_data)?;

    AuthRepository::find(db, data.id)
  }

  pub fn remove_login_attempt(
    redis: &mut RedisPooled,
    key: u64
  ) -> HttpResult<()> {
    let (redis_key, _) = Self::generate_redis_2fa_key(Some(key));

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