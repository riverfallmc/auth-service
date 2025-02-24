use axum::{extract::{Query, State}, routing::{get, post}, Json};
use adjust::{controller::Controller, response::{HttpMessage, HttpResult}};
use serde::Deserialize;
use crate::{service::logic::recovery::RecoveryService, AppState};

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
    State(state): State<AppState>,
    Json(body): Json<EmailBody>,
  ) -> HttpResult<HttpMessage> {
    let mut redis = state.redis.get()?;

    RecoveryService::recovery(&mut redis, body.email)
      .await
  }

  async fn exist(
    State(state): State<AppState>,
    Query(query): Query<RecoveryCode>
  ) -> HttpResult<HttpMessage> {
    let mut redis = state.redis.get()?;

    RecoveryService::exist(&mut redis, &query.code)
  }

  #[allow(dead_code)]
  async fn confirm_recovery(
    State(state): State<AppState>,
    Json(body): Json<ConfirmBody>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;
    let mut redis = state.redis.get()?;

    RecoveryService::confirm(&mut db, &mut redis, body.code, body.password)
      .await
  }
}

impl Controller<AppState> for RecoveryController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/recovery", post(Self::recovery))
      .route("/recoveryExist", get(Self::exist))
      .route("/recoveryConfirm", post(Self::confirm_recovery))
  }
}