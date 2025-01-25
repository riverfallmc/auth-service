use dixxxie::response::{HttpError, HttpResult};
use reqwest::StatusCode;

use crate::models::UserRegister;

pub struct AuthValidateService {}

const MIN_USERNAME: usize = 5;
const MAX_USERNAME: usize = 16;

const MIN_PASSWORD: usize = 8;
const MAX_PASSWORD: usize = 32;

impl AuthValidateService {
  pub fn validate(
    user: UserRegister
  ) -> HttpResult<()> {
    let username = user.username;

    if !(MIN_USERNAME..=MAX_USERNAME).contains(&username.len()) {
      return Err(HttpError::new("Никнейм должен быть больше 4 и меньше 17 символов", Some(StatusCode::BAD_REQUEST)))
    }

    if !(MIN_PASSWORD..=MAX_PASSWORD).contains(&user.password.len()) {
      return Err(HttpError::new("Пароль должен быть больше 7 и меньше 33 символов", Some(StatusCode::BAD_REQUEST)))
    }

    if username.is_empty() ||
      !username.chars().all(|c| c.is_alphanumeric() || c == '_') ||
      !username.chars().any(|c| c.is_alphanumeric())
    {
      return Err(HttpError::new("Никнейм должен содержать хотя бы одну букву или цифру, и может содержать только буквы (a-Z), цифры (0-9) и подчёркивания (_)", Some(StatusCode::BAD_REQUEST)));
    }

    Ok(())
  }
}