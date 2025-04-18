use std::sync::Arc;
use controller::{auth::AuthController, recovery::RecoveryController, register::RegisterController, sessions::SessionsController, tfa::TFAController};
use adjust::{main, controllers, database::{postgres::Postgres, redis::Redis, Pool}, controller::Controller, service::Service};

mod repository;
mod controller;
mod service;
mod models;
mod schema;
mod misc;

#[allow(unused)]
#[derive(Clone, Default)]
pub struct AppState {
  postgres: Arc<Pool<Postgres>>,
  redis: Pool<Redis>
}

#[main]
async fn main() -> Service<'_, AppState> {
  Service {
    name: "Auth",
    controllers: controllers![AuthController, SessionsController, RecoveryController, RegisterController, TFAController],
    ..Default::default()
  }
}