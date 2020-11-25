use std::sync::Arc;

use actix_web::web;
use chrono::format::format;
use redis::{Client as RedisClient, RedisError, RedisResult};

use crate::config::{Config, CONFIG};

#[derive(Clone)]
pub struct RedisCache {
    client: RedisClient,
}

impl RedisCache {
    pub fn new(connection: RedisClient) -> Self {
        Self { client: connection }
    }
}
fn init_cache(config: Config) -> RedisResult<RedisCache> {
    let client = RedisClient::open(config.redis_url)?;
    Ok(RedisCache { client })
}

pub fn add_cache(cfg: &mut web::ServiceConfig) {
    let cache = init_cache(CONFIG.clone()).expect("Failed to connect to the redis url");
    cfg.data(cache);
}
