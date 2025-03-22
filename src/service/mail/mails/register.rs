use hashbrown::HashMap;
use crate::service::mail::email::Email;

pub struct RegisterMail {
  username: String,
  url: String
}

impl RegisterMail {
  pub fn new(
    username: String,
    url: String
  ) -> Self{
    RegisterMail { username, url }
  }
}

impl TryFrom<RegisterMail> for Email {
  type Error = anyhow::Error;

  fn try_from(value: RegisterMail) -> anyhow::Result<Self> {
    let mut context = HashMap::new();
    context.insert("username".to_string(), value.username);
    context.insert("confirm_url".to_string(), value.url);

    Email::new(
      "data/templates/register.html".to_string(),
      "Подтверждение регистрации".to_string(),
      context
    )
  }
}