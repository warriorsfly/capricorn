use crate::config::{Config, CONFIG};
use actix_web::web;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

use redis::{Client, RedisResult};

// use futures::StreamExt;
// use rdkafka::{
//     config::RDKafkaLogLevel,
//     consumer::{Consumer, StreamConsumer},
//     error::KafkaError,
//     ClientConfig,
// };

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

// fn init_consumer(brokers: String, topics: &[String]) -> Result<StreamConsumer, KafkaError> {
//     let consumer: StreamConsumer = ClientConfig::new()
//         .set("group.id", "im-capricorn")
//         .set("bootstrap.servers", &brokers)
//         .set("auto.offset.reset", "latest")
//         .set("enable.partition.eof", "true")
//         .set("session.timeout.ms", "6000")
//         .set("enable.auto.commit", "true")
//         .set_log_level(RDKafkaLogLevel::Debug)
//         .create()?;

//     let topics = topics
//         .iter()
//         .map(|topic| topic.as_str())
//         .collect::<Vec<&str>>();

//     consumer.subscribe(topics.as_slice())?;

//     Ok(consumer)
// }

/// Database,Redis,Kafka connection pool all in here
pub struct DataSource {
    pub database: DatabasePool,
    pub redis: RedisPool,
}
