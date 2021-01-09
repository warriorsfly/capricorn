use crate::config::{Config, CONFIG};
use actix_web::web;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

use redis::{Client, RedisResult};

/// Database connection pool
pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(config: Config) -> Result<DatabasePool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    Pool::builder().build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = init_pool(CONFIG.clone()).expect("Failed to create connection pool");
    cfg.data(pool);
}

/// RedisPool,real redis client
pub type RedisPool = Client;

/// create redis client, with the Config.redis_url
fn init_redis(config: Config) -> RedisResult<RedisPool> {
    let client = RedisPool::open(config.redis_url)?;
    Ok(client)
}

/// add redis client to actix web server
pub fn add_redis(cfg: &mut web::ServiceConfig) {
    let cache = init_redis(CONFIG.clone()).expect("Failed to connect to the redis url");
    cfg.data(cache);
}
