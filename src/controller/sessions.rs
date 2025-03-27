use adjust::{controller::Controller, response::HttpResult};
use axum::{extract::{Path, State}, routing::get, Json, Router};
use crate::{models::SessionSafe, repository::session::SessionRepository, AppState};

pub struct SessionsController;

impl SessionsController {
  async fn get_sessions(
    State(state): State<AppState>,
    Path(user_id): Path<i32>
  ) -> HttpResult<Vec<SessionSafe>> {
    let mut db = state.postgres.get()?;

    Ok(Json(SessionRepository::get_sessions(&mut db, user_id)?))
  }
}

impl Controller<AppState> for SessionsController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: Router<AppState>) -> Router<AppState> {
    router
      .nest("/sessions",
        Router::new()
          .route("/{id}", get(Self::get_sessions))
      )
  }
}