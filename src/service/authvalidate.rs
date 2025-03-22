use adjust::response::{HttpError, NonJsonHttpResult};
use reqwest::StatusCode;

use crate::models::UserRegister;

pub struct AuthValidateService {}

const MIN_USERNAME: usize = 5;
const MAX_USERNAME: usize = 16;

const MIN_PASSWORD: usize = 8;
const MAX_PASSWORD: usize = 32;

impl AuthValidateService {
  fn validate_spell(
    text: String,
    kind: &str
  ) -> NonJsonHttpResult<()> {
    if text.is_empty() ||
      !text.chars().all(|c| c.is_alphanumeric() || c == '_') ||
      !text.chars().any(|c| c.is_alphabetic())
    {
      let msg = format!("{kind} должен содержать хотя бы одну букву или цифру, и может содержать только буквы (a-Z), цифры (0-9) и подчёркивания (_)");

      return Err(HttpError(anyhow::anyhow!(msg), Some(StatusCode::BAD_REQUEST)));
    }

    Ok(())
  }

  pub fn validate_username(
    username: String
  ) -> NonJsonHttpResult<()> {
    if !(MIN_USERNAME..=MAX_USERNAME).contains(&username.len()) {
      return Err(HttpError::new("Никнейм должен быть больше 4 и меньше 17 символов", Some(StatusCode::BAD_REQUEST)))
    }

    Self::validate_spell(username, "Никнейм")
  }

  pub fn validate_password(
    password: String
  ) -> NonJsonHttpResult<()> {
    if !(MIN_PASSWORD..=MAX_PASSWORD).contains(&password.len()) {
      return Err(HttpError::new("Пароль должен быть больше 7 и меньше 33 символов", Some(StatusCode::BAD_REQUEST)))
    }

    Self::validate_spell(password, "Пароль")
  }

  pub fn validate(
    user: UserRegister
  ) -> NonJsonHttpResult<()> {
    Self::validate_username(user.username.clone())?;
    Self::validate_password(user.password.clone())?;

    Ok(())
  }
}