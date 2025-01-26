use axum::{extract::State, http::HeaderMap, routing::post, Json};
use dixxxie::{controller::Controller, response::HttpResult};
use serde::{Deserialize, Serialize};
use crate::{misc::UserAgent, models::{BaseUserInfo, UserLogin}, service::auth::AuthService, ServerState};

pub struct AuthController;

#[derive(Deserialize)]
pub struct RefreshToken {
  refresh_jwt: String
}

#[derive(Serialize, Deserialize)]
pub struct JsonWebToken {
  pub token: String
}

impl AuthController {
  pub async fn login(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Json(user): Json<UserLogin>,
  ) -> HttpResult<Json<serde_json::Value>>{
    let user_agent = headers.get_user_agent();
    let mut redis = state.redis.get()?;
    let mut db = state.postgres.get()?;

    AuthService::login(&mut redis, &mut db, user, &user_agent)
      .await
  }

  pub async fn get_token_owner(
    State(state): State<ServerState>,
    Json(body): Json<JsonWebToken>
  ) -> HttpResult<Json<BaseUserInfo>> {
    let mut db = state.postgres.get()?;

    AuthService::get_owner(&mut db, body.token)
  }

  pub async fn refresh(
    State(state): State<ServerState>,
    Json(body): Json<RefreshToken>,
  ) -> HttpResult<Json<JsonWebToken>> {
    let mut db = state.postgres.get()?;

    AuthService::refresh(&mut db, body.refresh_jwt)
      .await
  }
}

impl Controller<ServerState> for AuthController {
  fn register(&self, router: axum::Router<ServerState>) -> axum::Router<ServerState> {
    router
      .route("/login", post(Self::login)) // логин
      .route("/refresh", post(Self::refresh)) // обновление токена
      .route("/owner", post(Self::get_token_owner)) // возвращает владельца токена
  }
}