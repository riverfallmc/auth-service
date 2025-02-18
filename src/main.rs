use controller::{auth::AuthController, register::RegisterController, tfa::TFAController};
use anyhow::Result;
use dixxxie::{
  axum::{self, Router}, connection::{establish_connection, establish_redis_connection, DbPool, RedisPool}, controller::ApplyControllerOnRouter, setup
};
use controller::recovery::RecoveryController;

mod repository;
mod controller;
mod service;
mod models;
mod schema;
mod misc;

#[allow(unused)]
#[derive(Clone)]
pub struct ServerState {
  postgres: DbPool,
  redis: RedisPool
}

#[tokio::main]
async fn main() -> Result<()> {
  setup()?;

  let state = ServerState {
    postgres: establish_connection()?,
    redis: establish_redis_connection()?
  };

  let router = Router::new()
    .apply_controller(AuthController)
    .apply_controller(RecoveryController)
    .apply_controller(RegisterController)
    .apply_controller(TFAController)
    .with_state(state);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
    .await?;

  Ok(axum::serve(listener, router).await?)
}