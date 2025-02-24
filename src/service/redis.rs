#![allow(dead_code)]

use anyhow::Result;
use adjust::{database::{redis::Redis, Database}, redis::{self, Commands}};

pub struct RedisService;

impl RedisService {
  pub fn get<T>(
    redis: &mut Database<Redis>,
    id: &str
  ) -> Result<T>
  where
    T: redis::FromRedisValue,
  {
    Ok(redis.get::<_, T>(id)?)
  }

  pub fn set<V>(
    redis: &mut Database<Redis>,
    id: &str,
    value: V
  ) -> Result<()>
  where
    V: redis::ToRedisArgs,
  {
    Ok(redis.set::<&str, V, ()>(id, value)?)
  }

  pub fn set_temporarily<V>(
    redis: &mut Database<Redis>,
    id: &str,
    value: V,
    mins: u64
  ) -> Result<()>
  where
    V: redis::ToRedisArgs,
  {
    Ok(redis.set_ex::<&str, V, ()>(id, value, mins*60)?)
  }

  pub fn remove(
    redis: &mut Database<Redis>,
    id: &str
  ) -> Result<()> {
    Ok(redis.del::<&str, ()>(id)?)
  }
}