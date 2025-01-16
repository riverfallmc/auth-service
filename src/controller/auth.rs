use axum::{extract::{Query, State}, http::HeaderMap, routing::{get, post}, Json};
use dixxxie::{controller::Controller, response::{HttpMessage, HttpResult}};
use serde::{Deserialize, Serialize};
use crate::{models::{Session, UserLogin, UserRegister}, service::{auth::AuthService, tfa::TwoFactorResponse}, ServerState};

pub struct AuthController;

#[derive(Deserialize)]
pub struct IdQuery {
  id: u64
}

#[derive(Deserialize)]
pub struct TFABody {
  code: String
}

#[derive(Deserialize)]
pub struct RefreshToken {
  refresh_jwt: String
}

#[derive(Serialize, Deserialize)]
pub struct JsonWebToken {
  pub jwt: String
}

impl AuthController {
  fn get_user_agent(
    headers: HeaderMap
  ) -> String {
    headers
      .get("user-agent")
      .and_then(|v| v.to_str().ok())
      .unwrap_or("n/a").to_owned()
  }

  pub async fn login(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Json(user): Json<UserLogin>,
  ) -> HttpResult<Json<serde_json::Value>>{
    let user_agent = &Self::get_user_agent(headers);
    let mut redis = state.redis.get()?;
    let mut db = state.postgres.get()?;

    AuthService::login(&mut redis, &mut db, user, user_agent)
      .await
  }

  pub async fn add_2fa(
    State(state): State<ServerState>,
    Json(body): Json<JsonWebToken>,
  ) -> HttpResult<Json<TwoFactorResponse>>{
    let mut db = state.postgres.get()?;

    AuthService::add_2fa(&mut db, body.jwt)
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

  pub async fn confirm_2fa(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Query(params): Query<IdQuery>,
    Json(body): Json<TFABody>,
  ) -> HttpResult<Json<Session>> {
    let user_agent = Self::get_user_agent(headers);
    let mut redis = state.redis.get()?;
    let mut db = state.postgres.get()?;

    AuthService::confirm_2fa(&mut db, &mut redis, params.id, body.code, user_agent)
      .await
  }

  pub async fn refresh(
    State(state): State<ServerState>,
    Json(body): Json<RefreshToken>,
  ) -> HttpResult<Json<JsonWebToken>> {
    let mut db = state.postgres.get()?;

    AuthService::refresh(&mut db, body.refresh_jwt)
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
      .route("/refresh", post(Self::refresh)) // обновление токена
      .route("/2fa/add", post(Self::add_2fa)) // 2fa - добавление
      .route("/2fa/confirm", post(Self::confirm_2fa)) // 2fa - подтверждение авторизации
  }
}