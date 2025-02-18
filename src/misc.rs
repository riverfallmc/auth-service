use anyhow::{anyhow, bail, Context, Result};
use axum::http::HeaderMap;

pub trait AuthorizationBearer {
  fn get_bearer(&self) -> Result<String>;
}

impl AuthorizationBearer for HeaderMap {
  fn get_bearer(&self) -> Result<String> {
    let splitted = self.get("authorization")
      .context(anyhow!("Заголовок Authorization не был предоставлен"))?
      .to_str()?
      .split("Bearer ")
      .collect::<Vec<&str>>();

    if splitted.len() != 2 {
      bail!("Неверный формат значения заголовка Authorization")
    }

    Ok(splitted.get(1)
      .unwrap()
      .to_string()) // здесь мы можем себе позволить анврап, потому-что выше проверка на длинну
  }
}

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