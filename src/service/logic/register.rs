use crate::{models::{UserAdd, UserRegister}, repository::{auth::AuthRepository, user::UserRepository}, service::{authvalidate::AuthValidateService, hasher::HasherService, mail::{mails::register::RegisterMail, service::MailService}, redis::RedisService}};
use adjust::{database::{postgres::Postgres, redis::Redis, Database}, response::{HttpError, HttpMessage, HttpResult}};
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
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
    mut user: UserRegister,
  ) -> HttpResult<HttpMessage> {
    // проверяем что ник написан по правилам
    AuthValidateService::validate(user.clone())?;

    // проверяем что ник/почта не заняты
    AuthRepository::check_userdata_taken(db, &user.username, &user.email)
      .await
      .map_err(|_| anyhow::anyhow!("Никнейм или электронная почта уже занята!"))?;

    // ʕ•́ᴥ•̀ʔっ подготавливаем пользователя для хранения в редисе
    // оверрайдим значение (по идее оно вообще не должно быть документировано) поля salt
    user.salt = Some(HasherService::generate_salt());
    // хэшируем пароль
    user.password = HasherService::sha256(user.password + &user.salt.clone().unwrap());

    // Отправляем письмо пользователю
    let (code, reg_id) = Self::generate_redis_confirm_key(None);

    let url = format!("https://riverfallmc.ru/api/auth/confirm?id={reg_id}");

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
      .map_err(|_| HttpError::new("Вас нет в списке на подтверждение регистрации", Some(StatusCode::BAD_REQUEST)))?;
    // десериализуем (папа) джонс объект в пользователя
    let user = serde_json::from_str::<UserRegister>(&record)?;

    // добавляем юзера в сервис пользователей
    let user_row = UserRepository::add(user.clone().into())
      .await?;

    let mut user_to_add: UserAdd = user.clone().into();
    user_to_add.user_id = Some(user_row.id);

    // добавляем юзера в наш бд
    #[allow(unused)]
    AuthRepository::add(db, &user_to_add)?;

    // чистим запись в редисе
    RedisService::remove(redis, &redis_key)?;

    Ok(Json(HttpMessage::new(&format!("Пользователь {} был успешно зарегистрирован.", user.username))))
  }
}