#![allow(dead_code)]

use std::env;
use adjust::response::NonJsonHttpResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::email::Email;

lazy_static::lazy_static! {
  static ref CLIENT: Client = Client::new();
  static ref MAIL_SERVICE_URL: String = env::var("MAIL_URL")
    .expect("The URL_MAIL environment variable was not found!");
  static ref MAIL_URL: String = format!("http://{}/send", *MAIL_SERVICE_URL);
}

#[derive(Serialize, Deserialize)]
pub struct MailData {
  pub to: String,
  pub subject: String,
  pub body: String,
}

pub struct MailService;

impl MailService {
  pub async fn send<T>(
    recipient: String,
    mail: T
  ) -> NonJsonHttpResult<()>
  where
    T: TryInto<Email, Error = anyhow::Error>,
  {
    let mail: Email = mail.try_into()?;

    CLIENT.post(MAIL_URL.to_string())
      .json(&MailData {
        to: recipient,
        subject: mail.subject.clone(),
        body: mail.render()?
      })
      .send()
      .await
      .map_err(|e| anyhow::anyhow!("Не получилось отправить запрос на сервис mail: {e}"))?
      .error_for_status()?;

    Ok(())
  }
}