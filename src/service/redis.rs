#![allow(dead_code)]

use anyhow::Result;
use dixxxie::{connection::RedisPooled, redis::{self, Commands}, response::HttpResult};

pub struct RedisService;

impl RedisService {
  pub fn get<T>(
    redis: &mut RedisPooled,
    id: &str
  ) -> Result<T>
  where
    T: redis::FromRedisValue,
  {
    Ok(redis.get::<_, T>(id)?)
  }

  pub fn set<V>(
    redis: &mut RedisPooled,
    id: &str,
    value: V
  ) -> Result<()>
  where
    V: redis::ToRedisArgs,
  {
    Ok(redis.set::<&str, V, ()>(id, value)?)
  }

  pub fn set_temporarily<V>(
    redis: &mut RedisPooled,
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
    redis: &mut RedisPooled,
    id: &str
  ) -> HttpResult<()> {
    Ok(redis.del::<&str, ()>(id)?)
  }
}