#![allow(dead_code)]

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm, TokenData};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use dixxxie::response::{HttpError, HttpResult};

lazy_static::lazy_static! {
    static ref JWT_SECRET: String = std::env::var("JWT_SECRET")
        .expect("The JWT_SECRET environment variable was not found!");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  sub: String,
  exp: usize,
  refresh: Option<bool>,
}

pub struct JWTService;

impl JWTService {
  // проверяет валидность JWT токена
  pub fn is_valid(token: String) -> bool {
    let parts = token.split('.')
      .collect::<Vec<&str>>();

    if parts.len() != 3 {
      return false; // Неправильный формат JWT
    }

    decode::<Claims>(
      &token,
      &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
      &Validation::new(Algorithm::HS256),
    )
    .is_ok()
  }

  // генерация jwt (действует 1 час)
  pub fn generate(
    user_id: i32
  ) -> HttpResult<String> {
    let claims = Claims {
      sub: user_id.to_string(),
      exp: Self::calculate_exp(60),
      refresh: None,
    };

    encode(
      &Header::new(Algorithm::HS256),
      &claims,
      &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    ).map_err(|_| HttpError::new("Не получилось сгенерировать Refresh токен", None))
  }

  // генерация refresh токена (действует 7 дней)
  pub fn generate_refresh(
    user_id: i32
  ) -> HttpResult<String> {
    let claims = Claims {
      sub: user_id.to_string(),
      exp: Self::calculate_exp(60 * 24 * 7),
      refresh: Some(true),
    };

    encode(
      &Header::new(Algorithm::HS256),
      &claims,
      &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    ).map_err(|_| HttpError::new("Не получилось сгенерировать Refresh токен", None))
  }

  // декодирование токена и возврат данных
  pub fn decode_token(
    token: &str
  ) -> HttpResult<TokenData<Claims>> {
    decode::<Claims>(
      &token,
      &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
      &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| HttpError::new("Невалидный или истёкший токен", Some(StatusCode::UNAUTHORIZED)))
  }

  // вычисление времени истечения токена (в секундах)
  fn calculate_exp(
    minutes: usize
  ) -> usize {
    let now = chrono::Utc::now().timestamp() as usize;
    now + minutes * 60
  }
}