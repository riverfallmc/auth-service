use dixxxie::{controller::Controller, response::{HttpMessage, HttpResult}};
use axum::{extract::{Query, State}, http::HeaderMap, routing::post, Json};
use serde::{Deserialize, Serialize};
use crate::{misc::{AuthorizationBearer, UserAgent}, models::Session, service::logic::tfa::TFAService, ServerState};

#[derive(Deserialize, Serialize)]
pub struct TFAAddBody {
  pub secret: String,
  pub qr: String
}

#[derive(Deserialize)]
pub struct TFAQuery {
  username: String,
}

#[derive(Deserialize)]
pub struct TFALoginBody {
  code: String
}

#[derive(Deserialize, Serialize)]
pub struct TFALinkBody {
  code: String,
  secret: String
}

pub struct TFAController;

impl TFAController {
  /// Генерирует 2FA Secret и отправляет его в виде Json'а
  async fn add(
    headers: HeaderMap,
    State(state): State<ServerState>,
  ) -> HttpResult<Json<TFAAddBody>> {
    let mut db = state.postgres.get()?;
    let token = headers.get_bearer()?;

    TFAService::add(&mut db, token)
  }

  /// Привязывает 2FA Secret к профилю
  async fn link(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Json(body): Json<TFALinkBody>
  ) -> HttpResult<Json<HttpMessage>> {
    let mut db = state.postgres.get()?;
    let token = headers.get_bearer()?;

    TFAService::link(&mut db, token, body.code, body.secret)
  }

  /// Входит в аккаунт
  async fn login(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Query(params): Query<TFAQuery>,
    Json(body): Json<TFALoginBody>
  ) -> HttpResult<Json<Session>> {
    let user_agent = headers.get_user_agent();
    let mut redis = state.redis.get()?;
    let mut db = state.postgres.get()?;

    TFAService::login(&mut db, &mut redis, params.username, body.code, user_agent)
      .await
  }
}

impl Controller<ServerState> for TFAController {
  fn register(&self, router: axum::Router<ServerState>) -> axum::Router<ServerState> {
    router
      .route("/2fa/add", post(Self::add))
      .route("/2fa/link", post(Self::link))
      .route("/2fa/login", post(Self::login))
  }
}