use axum::http::HeaderMap;

pub trait UserAgent {
  fn get_user_agent(&self) -> String;
}

impl UserAgent for HeaderMap {
  fn get_user_agent(&self) -> String {
    self
      .get("user-agent")
      .and_then(|v| v.to_str().ok())
      .unwrap_or("n/a").to_owned()
  }
}