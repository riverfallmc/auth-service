use chrono::{NaiveDateTime, Utc};

pub struct TimeService;

impl TimeService {
  // возвращает текущее время, не зависимое от временной зоны
  pub fn get_current_timestamp() -> i64 {
    Utc::now()
      .timestamp()
  }

  pub fn get_current_time() -> NaiveDateTime {
    Utc::now()
      .naive_utc()
  }
}