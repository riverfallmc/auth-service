#![allow(dead_code)]

use anyhow::{anyhow, Result};
use axum::Json;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm, TokenData};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use dixxxie::response::{HttpError, HttpResult};

use super::time::TimeService;

lazy_static::lazy_static! {
  static ref JWT_SECRET: String = std::env::var("JWT_SECRET")
    .expect("The JWT_SECRET environment variable was not found!");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  sub: String,
  exp: usize,
  refresh: bool,
}

pub struct JWTService;

impl JWTService {
  // проверяет валидность JWT токена
  pub fn is_valid(token: &str) -> bool {
    let parts = token.split('.')
      .collect::<Vec<&str>>();

    if parts.len() != 3 {
      println!("3");
      return false; // Неправильный формат JWT
    }

    decode::<Claims>(
      token,
      &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
      &Validation::new(Algorithm::HS256),
    )
    .is_ok()
  }

  pub fn is_active(
    token: String
  ) -> HttpResult<String> {
    if !Self::is_valid(&token) {
      return Err(HttpError::new("Невалидный токен", Some(StatusCode::UNAUTHORIZED)))
    }

    let payload = Self::decode_token(&token)?;

    if payload.claims.exp < (TimeService::get_current_timestamp() as usize) {
      return Err(HttpError::new("Токен истёк", Some(StatusCode::UNAUTHORIZED)))
    }

    Ok(axum::Json(payload.claims.sub.clone()))
  }

  // генерация jwt (действует 1 час)
  pub fn generate(
    user_id: i32
  ) -> Result<String> {
    let claims = Claims {
      sub: user_id.to_string(),
      exp: Self::calculate_exp(60),
      refresh: false,
    };

    encode(
      &Header::new(Algorithm::HS256),
      &claims,
      &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    ).map_err(|_| anyhow!("Не получилось сгенерировать JWT"))
  }

  // генерация refresh токена (действует 7 дней)
  pub fn generate_refresh(
    user_id: i32
  ) -> Result<String> {
    let claims = Claims {
      sub: user_id.to_string(),
      exp: Self::calculate_exp(60 * 24 * 7),
      refresh: true,
    };

    encode(
      &Header::new(Algorithm::HS256),
      &claims,
      &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    ).map_err(|_| anyhow!("Не получилось сгенерировать Refresh токен"))
  }

  // декодирование токена и возврат данных
  pub fn decode_token(
    token: &str
  ) -> HttpResult<TokenData<Claims>> {
    Ok(Json(decode::<Claims>(
      token,
      &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
      &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| HttpError::new("Невалидный или истёкший токен", Some(StatusCode::UNAUTHORIZED)))?))
  }

  // вычисление времени истечения токена (в секундах)
  fn calculate_exp(
    minutes: usize
  ) -> usize {
    let now = TimeService::get_current_timestamp() as usize;
    now + minutes * 60
  }
}