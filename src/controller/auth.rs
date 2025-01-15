use axum::{extract::{Query, State}, http::HeaderMap, routing::{get, post}, Json};
use dixxxie::{controller::Controller, response::{HttpMessage, HttpResult}};
use serde::Deserialize;
use crate::{models::{UserLogin, UserRegister}, service::auth::AuthService, ServerState};

pub struct AuthController;

#[derive(Deserialize)]
pub struct IdQuery {
  id: u64
}

impl AuthController {
  pub async fn login(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Json(user): Json<UserLogin>,
  ) -> HttpResult<Json<serde_json::Value>>{
    let mut redis = state.redis.get()?;
    let mut db = state.postgres.get()?;
    let user_agent = headers
      .get("user-agent")
      .and_then(|v| v.to_str().ok())
      .unwrap_or("n/a");

    AuthService::login(&mut redis, &mut db, user, user_agent)
      .await
  }

  pub async fn confirm(
    State(state): State<ServerState>,
    Query(params): Query<IdQuery>,
  ) -> HttpResult<Json<HttpMessage>> {
    let mut redis = state.redis.get()?;
    let mut db = state.postgres.get()?;

    AuthService::confirm(&mut redis, &mut db, params.id)
      .await
  }

  pub async fn registration(
    State(state): State<ServerState>,
    Json(body): Json<UserRegister>,
  ) -> HttpResult<Json<HttpMessage>> {
    let mut redis = state.redis.get()?;

    AuthService::register(&mut redis, body)
      .await
  }
}

impl Controller<ServerState> for AuthController {
  fn register(&self, router: axum::Router<ServerState>) -> axum::Router<ServerState> {
    router
      .route("/login", post(Self::login)) // логин
      .route("/register", post(Self::registration)) // регистрация
      .route("/confirm", get(Self::confirm)) // подтверждение регистрации
      .route("/2fa/add", post(Self::confirm)) // 2fa - добавление
      .route("/2fa/confirm", post(Self::confirm)) // 2fa - подтверждение авторизации
  }
}