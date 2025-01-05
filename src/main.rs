use anyhow::Result;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<()> {
  if cfg!(debug_assertions) {
    dotenv::dotenv().ok();
  }


  env_logger::init();

  log::info!("Drochi mne hui");

  let router = Router::new()
    .route("/", get(|| async {"ZALUPNIY SHERSHEN'"}));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await?;

  Ok(axum::serve(listener, router).await?)
}