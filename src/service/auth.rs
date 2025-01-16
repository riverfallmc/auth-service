#![allow(dead_code)]

use crate::{controller::auth::TokenRefreshed, models::{UserAdd, UserLogin, UserLoginedData, UserRegister}, repository::{auth::AuthRepository, user::UserRepository}, schema::users, service::{jwt::JWTService, tfa::TFAService}};
use super::{hasher::HasherService, mail::{email::Email, service::MailService}, redis::RedisService, session::service::SessionService, tfa::TwoFactorResponse};
use axum::Json;
use diesel::RunQueryDsl;
use dixxxie::{connection::{DbPooled, RedisPooled}, response::{HttpError, HttpMessage, HttpResult}};
use hashbrown::HashMap;
use reqwest::StatusCode;

pub struct AuthService;

impl AuthService {
  fn generate_redis_confirm_key(
    key: Option<u64>
  ) -> (String, u64) {
    let key = key.unwrap_or(HasherService::generate_code());

    (format!("register:{}", key), key)
  }

  // привязка 2FA (TOTP) к профилю пользователя
  pub fn add_2fa(
    db: &mut DbPooled,
    jwt: String
  ) -> HttpResult<Json<TwoFactorResponse>> {
    let session = SessionService::get_by_jwt(db, jwt, true)?;
    let user = AuthRepository::find(db, session.id)?;

    if user.totp_secret.is_some() {
      return Err(HttpError::new("К вашему профилю уже привязана двуфакторная аутентификация.", Some(StatusCode::CONFLICT)))
    }

    let (secret, totp) = TFAService::generate_2fa(user.username)?;

    AuthRepository::update_totp(db, user.id, secret.clone())?;

    Ok(Json(TwoFactorResponse {
        secret,
        qr_url: totp.get_qr_base64().unwrap_or_default(), // todo ETO CHTO NAHOOI
    }))
  }

  // продолжение авторизации (/login): ввод кода от 2FA (TOTP)
  pub async fn confirm_2fa(
    db: &mut DbPooled,
    redis: &mut RedisPooled,
    code: u64,
  ) -> HttpResult<UserLoginedData> {
    let _user = TFAService::get_login_attempt(redis, db, code)
      .map_err(|_| HttpError::new("", Some(StatusCode::BAD_REQUEST)))?;

    todo!()
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
      let data = TFAService::add_login_attempt(redis, user.id, user_agent)?;

      return Ok(Json(serde_json::to_value(data.0)?));
    }

    // создаем сессию в любом случае
    let session = SessionService::get(db, user, user_agent, true, true)?;

    Ok(Json(serde_json::to_value(session)?))
  }

  // подтверждение регистрации пользователя
  pub async fn confirm(
    redis: &mut RedisPooled,
    db: &mut DbPooled,
    id: u64
  ) -> HttpResult<Json<HttpMessage>> {
    let (redis_key, _) = Self::generate_redis_confirm_key(Some(id));
    // ищем запись в редисе
    let record = RedisService::get::<String>(redis, &redis_key)?;
      // .map_err(|_| Err(HttpError::new("You are not on the registration confirmation list!", Some(StatusCode::BAD_REQUEST))))?; // todo
    // десериализуем джон объект в пользователя
    let user = serde_json::from_str::<UserRegister>(&record)?;

    // добавляем юзера в сервис пользователей
    UserRepository::add(user.clone().into())
      .await?;

    // добавляем юзера в наш бд
    let id = diesel::insert_into(users::table)
      .values::<UserAdd>(user.clone().into())
      .execute(db)?;

    Ok(Json(HttpMessage::new(&format!("Пользователь {} с Id {} был успешно зарегистрирован.", user.username, id))))
  }

  pub async fn refresh(
    db: &mut DbPooled,
    refresh_token: String
  ) -> HttpResult<Json<TokenRefreshed>> {
    let session = SessionService::get_by_refresh(db, refresh_token, true)?;
    let jwt = JWTService::generate(session.user_id)?;

    SessionService::update(db, session.id, &jwt)?;

    todo!()
  }

  // регистрация пользователя
  pub async fn register(
    redis: &mut RedisPooled,
    mut user: UserRegister,
  ) -> HttpResult<Json<HttpMessage>> {
    // оверрайдим значение (по идее оно вообще не должно быть документировано) поля salt
    user.salt = Some(HasherService::generate_salt());
    // ʕ•́ᴥ•̀ʔっ подготавливаем пользователя для хранения в редисе
    // хэшируем пароль
    user.password = HasherService::sha256(user.password + &user.salt.clone().unwrap());

    // Отправляем письмо пользователю
    let (code, reg_id) = Self::generate_redis_confirm_key(None);

    // todo: сделать структуру для каждого письма

    let mail = Email::new(String::from("data/templates/register.html"), String::from("Регистрация"), {
      let mut hashmap = HashMap::new();
      hashmap.insert(String::from("username"), user.username.clone());
      hashmap.insert(String::from("confirm_url"), format!("https://serenitymc.ru/api/auth/confirm?id={reg_id}"));
      hashmap
    })?;

    let sended = MailService::send(user.email.clone(), mail).await?;
    let jsoned_user = serde_json::to_string(&user)?;

    // сохраняем username + hashed password в редисе на 10 минут
    // т.е у юзера есть 10 минут чтобы зайти по ссылке из письма
    // для регистрации
    RedisService::set_temporarily(redis, &code, jsoned_user, 10)?;

    Ok(sended)
  }
}