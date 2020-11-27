use actix_web::web;

use redis::{Client, RedisResult};

use crate::config::{Config, CONFIG};

pub type RedisCache = Client;

/// create redis client, with the Config.redis_url
fn init_cache(config: Config) -> RedisResult<RedisCache> {
    let client = RedisCache::open(config.redis_url)?;
    Ok(client)
}

/// add redis client to actix web server
pub fn add_cache(cfg: &mut web::ServiceConfig) {
    let cache = init_cache(CONFIG.clone()).expect("Failed to connect to the redis url");
    cfg.data(cache);
}