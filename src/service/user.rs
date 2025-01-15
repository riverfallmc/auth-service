#![allow(unused)]

use crate::{models::{User, UserCreate}, repository::user::UserRepository};
use anyhow::Result;
use dixxxie::connection::DbPool;

pub struct UserService;

impl UserService {
  /// Добавляет игрока в базу данных
  pub async fn add_user(
    user: UserCreate
  ) -> Result<()> {
    UserRepository::add(user)
      .await
  }

  pub async fn get_user(
    id: i32
  ) -> Result<User> {
    UserRepository::get(id)
      .await
  }
}