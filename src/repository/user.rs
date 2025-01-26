#![allow(unused)]

use std::env;
use crate::{models::{User, UserCreate, UserInUserService}, schema::users};
use anyhow::{bail, Result};
use dixxxie::{connection::{DbPool, DbPooled}, response::HttpResult};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};

static CLIENT: Lazy<Client> = Lazy::new(Client::new);
static USER_URL: Lazy<String> = Lazy::new(|| {
  env::var("USER_URL")
    .expect("The USER_URL environment variable was not found!")
});

#[derive(Deserialize)]
struct Response {
  is_error: Option<bool>,
  message: String
}

pub struct UserRepository;

impl UserRepository {
  pub async fn add(
    user: UserCreate
  ) -> Result<()> {
    let res = CLIENT.post(format!("http://{}/user", *USER_URL))
      .json(&user)
      .send()
      .await
      .map_err(|e| anyhow::anyhow!("Не получилось отправить запрос на сервис user: {e}"))?;

    let status = res.status();

    let json = res.json::<Response>()
      .await?;

    if !status.is_success() {
      bail!(json.message);
    }

    Ok(())
  }

  pub async fn find_by_email(
    email: String
  ) -> Result<UserInUserService> {
    let res = CLIENT.get(format!("http://{}/user/0?email={}", *USER_URL, email))
      .send()
      .await
      .map_err(|e| anyhow::anyhow!("Не получилось отправить запрос на сервис user: {e}"))?
      .error_for_status()?;

    let json = res.json::<UserInUserService>()
      .await?;

    Ok(json)
  }
}