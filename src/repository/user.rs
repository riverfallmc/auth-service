#![allow(unused)]

use std::{env, sync::LazyLock};
use crate::{models::{User, UserCreate, UserInUserService}, schema::users};
use adjust::load_env;
use anyhow::{bail, Result};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use reqwest::Client;
use serde::{Deserialize, Serialize};

static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

load_env!(USER_URL);

#[derive(Deserialize)]
struct Response {
  is_error: Option<bool>,
  message: String
}

pub struct UserRepository;

impl UserRepository {
  pub async fn add(
    user: UserCreate
  ) -> Result<UserInUserService> {
    let res = CLIENT.post(format!("http://{}/user", *USER_URL))
      .json(&user)
      .send()
      .await
      .map_err(|e| anyhow::anyhow!("Не получилось отправить запрос на сервис user: {e}"))?;

    let status = res.status();

    if !status.is_success() {
      let json = res.json::<Response>()
        .await?;

      bail!(json.message);
    }

    let json = res.json::<UserInUserService>()
      .await?;

    Ok(json)
  }

  pub async fn find_by_email(
    email: &str
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