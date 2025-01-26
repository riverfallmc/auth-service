use axum::{extract::{Query, State}, routing::{get, post}, Json};
use dixxxie::{controller::Controller, response::{HttpMessage, HttpResult}};
use serde::Deserialize;
use crate::{service::logic::recovery::RecoveryService, ServerState};

#[derive(Deserialize)]
struct EmailBody {
  email: String
}

#[derive(Deserialize)]
struct ConfirmBody {
  code: String,
  password: String
}

#[derive(Deserialize)]
struct RecoveryCode {
  code: String
}

pub struct RecoveryController;

impl RecoveryController {
  async fn recovery(
    State(state): State<ServerState>,
    Json(body): Json<EmailBody>,
  ) -> HttpResult<Json<HttpMessage>> {
    let mut redis = state.redis.get()?;

    RecoveryService::recovery(&mut redis, body.email)
      .await
  }

  async fn exist(
    State(state): State<ServerState>,
    Query(query): Query<RecoveryCode>
  ) -> HttpResult<Json<HttpMessage>> {
    let mut redis = state.redis.get()?;

    RecoveryService::exist(&mut redis, &query.code)
  }

  #[allow(dead_code)]
  async fn confirm_recovery(
    State(state): State<ServerState>,
    Json(body): Json<ConfirmBody>
  ) -> HttpResult<Json<HttpMessage>> {
    let mut db = state.postgres.get()?;
    let mut redis = state.redis.get()?;

    RecoveryService::confirm(&mut db, &mut redis, body.code, body.password)
      .await
  }
}

impl Controller<ServerState> for RecoveryController {
  fn register(&self, router: axum::Router<ServerState>) -> axum::Router<ServerState> {
    router
      .route("/recovery", post(Self::recovery))
      .route("/recoveryExist", get(Self::exist))
      .route("/recoveryConfirm", post(Self::confirm_recovery))
  }
}