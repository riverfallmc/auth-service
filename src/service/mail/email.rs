#![allow(dead_code)]

use std::{fs::File, io::Read, path::Path};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use hashbrown::HashMap;

const NO_TEMPLATE: &'static str = "Не получилось найти заготовку для электронного письма.";

#[derive(Serialize, Deserialize)]
pub struct MailData {
  pub to: String,
  pub subject: String,
  pub body: String,
}

#[derive(Clone)]
pub struct Email {
  template: String,
  context: HashMap<String, String>,
  pub subject: String
}

impl Email {
  pub fn new(
    template: String,
    subject: String,
    context: HashMap<String, String>
  ) -> Result<Email> {
    // Проверяем что темплейт существует
    if !Path::new(&template).exists() {
      return Err(anyhow::anyhow!(NO_TEMPLATE));
    }

    Ok(Email {template, subject, context})
  }

  fn read_template(&self) -> Result<String> {
    let mut content = String::new();

    File::open(&self.template)?
      .read_to_string(&mut content)?;

    Ok(content)
  }

  pub fn render(&self) -> Result<String> {
    let template = self.read_template()?;

    let rendered = self.context.iter().fold(template, |r, (k, v)| {
      r.replace(&format!("{{{}}}", k), v)
    });

    Ok(rendered)
  }

}