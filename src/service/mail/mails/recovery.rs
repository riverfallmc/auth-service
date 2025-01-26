use hashbrown::HashMap;
use crate::service::mail::email::Email;

pub struct RecoveryMail {
  username: String,
  code: String
}

impl RecoveryMail {
  pub fn new(
    username: String,
    code: String
  ) -> Self{
    RecoveryMail { username, code }
  }
}

impl TryFrom<RecoveryMail> for Email {
  type Error = anyhow::Error;

  fn try_from(value: RecoveryMail) -> anyhow::Result<Self> {
    let mut context = HashMap::new();
    context.insert("username".to_string(), value.username);
    context.insert("code".to_string(), value.code);

    Email::new(
      "data/templates/recovery.html".to_string(),
      "Сброс пароля".to_string(),
      context
    )
  }
}