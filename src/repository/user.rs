#![allow(unused)]

use std::env;
use crate::{models::{User, UserCreate}, schema::users};
use anyhow::{bail, Result};
use dixxxie::{connection::{DbPool, DbPooled}};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use reqwest::Client;
use serde::Deserialize;

lazy_static::lazy_static! {
  static ref CLIENT: Client = Client::new();
  static ref USER_URL: String = env::var("USER_URL")
    .expect("The USER_URL environment variable was not found!");
}

pub struct UserRepository;

#[derive(Deserialize)]
struct Response {
  is_error: Option<bool>,
  message: String
}

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
}