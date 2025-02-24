use anyhow::Result;
use axum::Json;
use adjust::{database::{postgres::Postgres, redis::Redis, Database}, response::{HttpError, HttpMessage, HttpResult}};
use reqwest::StatusCode;
use crate::{models::UserPasswordUpdate, repository::{auth::AuthRepository, user::UserRepository}, service::{authvalidate::AuthValidateService, hasher::HasherService, mail::{mails::recovery::RecoveryMail, service::MailService}, redis::RedisService}};

pub struct RecoveryService;

impl RecoveryService {
  fn get_record_key(
    code: &String
  ) -> String {
    format!("recovery:{code}")
  }

  async fn add_record(
    redis: &mut Database<Redis>,
    email: &String
  ) -> Result<String> {
    let code = HasherService::generate_code();

    RedisService::set_temporarily(redis, &Self::get_record_key(&code), email, 5)?;
    RedisService::set_temporarily(redis, &Self::get_record_key(email), &code, 5)?;

    Ok(code)
  }

  // восстановление пароля
  pub async fn recovery(
    redis: &mut Database<Redis>,
    email: String
  ) -> HttpResult<HttpMessage> {
    if Self::exist_email(redis, &email).is_ok() {
      return Err(HttpError::new("Вы уже имеете запрос на сброс пароля", Some(StatusCode::BAD_REQUEST)));
    }

    let user = UserRepository::find_by_email(email.clone())
      .await?;

    let code = Self::add_record(redis, &email)
      .await?;

    MailService::send(email, RecoveryMail::new(user.username, code))
      .await?;

    Ok(Json(HttpMessage::new("На вашу почту было отправленно письмо с ссылкой для сброса пароля")))
  }

  pub async fn get_record(
    redis: &mut Database<Redis>,
    code: String
  ) -> HttpResult<String> {
    Ok(Json(RedisService::get::<String>(redis, &Self::get_record_key(&code))
        .map_err(|_| HttpError::new("Запись не найдена", Some(StatusCode::BAD_REQUEST)))?))
  }

  pub fn exist(
    redis: &mut Database<Redis>,
    code: &String
  ) -> HttpResult<HttpMessage> {
    RedisService::get::<String>(redis, &Self::get_record_key(code))
      .map_err(|_| HttpError::new("Запись не найдена", Some(StatusCode::BAD_REQUEST)))?;

    Ok(Json(HttpMessage::new("Запись существует")))
  }

  fn exist_email(
    redis: &mut Database<Redis>,
    email: &String
  ) -> HttpResult<HttpMessage> {
    RedisService::get::<String>(redis, &Self::get_record_key(email))
      .map_err(|_| HttpError::new("", Some(StatusCode::BAD_GATEWAY)))?;

    Ok(Json(HttpMessage::new("Запись существует")))
  }

  fn remove_record(
    redis: &mut Database<Redis>,
    email: String
  ) -> Result<()> {
    let code = RedisService::get::<String>(redis, &Self::get_record_key(&email))?;

    RedisService::remove(redis, &Self::get_record_key(&code))
  }

  pub async fn confirm(
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
    code: String,
    password: String
  ) -> HttpResult<HttpMessage> {
    let email = Self::get_record(redis, code)
      .await?;

    AuthValidateService::validate_password(password.clone())?;

    let userdata = UserRepository::find_by_email(email.to_string())
      .await?;

    let user = AuthRepository::find(db, userdata.id)?;
    let salt = HasherService::generate_salt();
    let password = HasherService::sha256(password + &salt);

    AuthRepository::update(db, user.id, UserPasswordUpdate { salt, password })?;

    Self::remove_record(redis, email.to_string())?;

    Ok(Json(HttpMessage::new("Пароль был сменён!")))
  }
}