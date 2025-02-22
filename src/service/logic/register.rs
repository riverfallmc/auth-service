use crate::{models::UserRegister, repository::{auth::AuthRepository, user::UserRepository}, service::{authvalidate::AuthValidateService, hasher::HasherService, mail::{mails::register::RegisterMail, service::MailService}, redis::RedisService}};
use dixxxie::{database::{postgres::Postgres, redis::Redis, Database}, response::{HttpError, HttpMessage, HttpResult}};
use reqwest::StatusCode;
use axum::Json;

pub struct RegisterService;

impl RegisterService {
  fn generate_redis_confirm_key(
    key: Option<String>
  ) -> (String, String) {
    let key = key.unwrap_or(HasherService::generate_code());

    (format!("register:{}", key), key)
  }

  pub async fn register(
    redis: &mut Database<Redis>,
    mut user: UserRegister,
  ) -> HttpResult<HttpMessage> {
    AuthValidateService::validate(user.clone())?;
    // оверрайдим значение (по идее оно вообще не должно быть документировано) поля salt
    user.salt = Some(HasherService::generate_salt());
    // ʕ•́ᴥ•̀ʔっ подготавливаем пользователя для хранения в редисе
    // хэшируем пароль
    user.password = HasherService::sha256(user.password + &user.salt.clone().unwrap());

    // Отправляем письмо пользователю
    let (code, reg_id) = Self::generate_redis_confirm_key(None);

    let url = format!("https://serenitymc.ru/api/auth/confirm?id={reg_id}");

    // отправляем письмо
    MailService::send(user.email.clone(), RegisterMail::new(user.username.clone(), url))
      .await?;

    // сохраняем username + hashed password в редисе на 10 минут
    // т.е у юзера есть 10 минут чтобы зайти по ссылке из письма
    // для регистрации
    let jsoned_user = serde_json::to_string(&user)?;

    RedisService::set_temporarily(redis, &code, jsoned_user, 10)?;

    Ok(Json(HttpMessage::new("Подтвердите вашу регистрацию с помощью ссылки, высланной на вашу почту.")))
  }

  pub async fn confirm(
    redis: &mut Database<Redis>,
    db: &mut Database<Postgres>,
    id: String
  ) -> HttpResult<HttpMessage> {
    let (redis_key, _) = Self::generate_redis_confirm_key(Some(id));
    // ищем запись в редисе
    let record = RedisService::get::<String>(redis, &redis_key)
      .map_err(|_| HttpError::new("You are not on the registration confirmation list!", Some(StatusCode::BAD_REQUEST)))?;
    // десериализуем (папа) джонс объект в пользователя
    let user = serde_json::from_str::<UserRegister>(&record)?;

    // добавляем юзера в сервис пользователей
    UserRepository::add(user.clone().into())
      .await?;

    // добавляем юзера в наш бд
    let id = AuthRepository::add(db, &user.clone().into())?;

    Ok(Json(HttpMessage::new(&format!("Пользователь {} с Id {} был успешно зарегистрирован.", user.username, id))))
  }
}