use axum::{extract::{Query, State}, routing::{get, post}, Json};
use adjust::{controller::Controller, response::{HttpMessage, HttpResult}};
use serde::Deserialize;
use crate::{models::UserRegister, service::logic::register::RegisterService, AppState};

#[derive(Deserialize)]
pub struct IdQuery {
  id: String
}

pub struct RegisterController;

impl RegisterController {
  async fn registration(
    State(state): State<AppState>,
    Json(body): Json<UserRegister>,
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;
    let mut redis = state.redis.get()?;

    RegisterService::register(&mut db, &mut redis, body)
      .await
  }

  async fn confirm(
    State(state): State<AppState>,
    Query(params): Query<IdQuery>,
  ) -> HttpResult<HttpMessage> {
    let mut redis = state.redis.get()?;
    let mut db = state.postgres.get()?;

    RegisterService::confirm(&mut redis, &mut db, params.id)
      .await
  }
}

impl Controller<AppState> for RegisterController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/register", post(Self::registration)) // регистрация
      .route("/confirm", get(Self::confirm)) // подтверждение регистрации
  }
}