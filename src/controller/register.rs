use axum::{extract::{Query, State}, routing::{get, post}, Json};
use dixxxie::{controller::Controller, response::{HttpMessage, HttpResult}};
use serde::Deserialize;
use crate::{models::UserRegister, service::logic::register::RegisterService, ServerState};

#[derive(Deserialize)]
pub struct IdQuery {
  id: String
}

pub struct RegisterController;

impl RegisterController {
  async fn registration(
    State(state): State<ServerState>,
    Json(body): Json<UserRegister>,
  ) -> HttpResult<Json<HttpMessage>> {
    let mut redis = state.redis.get()?;

    RegisterService::register(&mut redis, body)
      .await
  }

  async fn confirm(
    State(state): State<ServerState>,
    Query(params): Query<IdQuery>,
  ) -> HttpResult<Json<HttpMessage>> {
    let mut redis = state.redis.get()?;
    let mut db = state.postgres.get()?;

    RegisterService::confirm(&mut redis, &mut db, params.id)
      .await
  }
}

impl Controller<ServerState> for RegisterController {
  fn register(&self, router: axum::Router<ServerState>) -> axum::Router<ServerState> {
    router
      .route("/register", post(Self::registration)) // регистрация
      .route("/confirm", get(Self::confirm)) // подтверждение регистрации
  }
}