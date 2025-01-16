#![allow(unused)]

use std::env;

use crate::{models::{User, UserCreate}, schema::users};
use anyhow::Result;
use dixxxie::{connection::{DbPool, DbPooled}};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use reqwest::Client;

lazy_static::lazy_static! {
  static ref CLIENT: Client = Client::new();
  static ref USER_URL: String = env::var("USER_URL")
    .expect("The USER_URL environment variable was not found!");
}

pub struct UserRepository;

impl UserRepository {
  pub async fn add(
    user: UserCreate
  ) -> Result<()> {
    CLIENT.post(format!("http://{}/user", *USER_URL))
      .json(&user)
      .send()
      .await
      .map_err(|e| anyhow::anyhow!("Не получилось отправить запрос на сервис user: {e}"))?
      .error_for_status()?;

    Ok(())
  }
}