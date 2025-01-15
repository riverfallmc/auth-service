#![allow(dead_code)]

use data_encoding::BASE32;
use sha2::{Digest, Sha256};
use rand::{distributions::Alphanumeric, Rng};

const MIN_CODE: u64 = 1000000000;
const MAX_CODE: u64 = 9999999999;

pub struct HasherService;

impl HasherService {
  // генерирует SHA-256 хэш-сумму
  pub fn sha256(value: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value);
    let result = hasher.finalize();
    // конвертим в 16-иричную строку
    hex::encode(result)
  }

  // генерирует соль для пароля
  pub fn generate_salt() -> String {
    rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(16) // 16 символов
      .map(char::from)
      .collect()
  }

  // генерирует случайное число
  // которое может использоваться как айди
  pub fn generate_code() -> u64 {
    let mut rng = rand::thread_rng();

    // ну.. эта запись выглядит не очень
    rng.gen_range(MIN_CODE..=MAX_CODE)
  }

  pub fn generate_2fa_secret() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 20];
    rng.fill(&mut bytes);
    BASE32.encode(&bytes).replace("=", "")
  }
}
