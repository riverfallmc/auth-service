use controller::{auth::AuthController, recovery::RecoveryController, register::RegisterController, tfa::TFAController};
use anyhow::Result;
use dixxxie::{controllers, database::{postgres::Postgres, redis::Redis, Pool, PoolBuilder}, server::WebServer, service::Service};
use dixxxie::controller::Controller;

mod repository;
mod controller;
mod service;
mod models;
mod schema;
mod misc;

#[allow(unused)]
#[derive(Clone)]
pub struct AppState {
  postgres: Pool<Postgres>,
  redis: Pool<Redis>
}

#[tokio::main]
async fn main() -> Result<()> {
  WebServer::enviroment();

  let service = Service {
    name: "Auth",
    controllers: controllers![AuthController, RecoveryController, RegisterController, TFAController],
    state: AppState {
      postgres: Postgres::create_pool()?,
      redis: Redis::create_pool()?
    },
    port: Some(1337)
  };

  service.run()
    .await
}