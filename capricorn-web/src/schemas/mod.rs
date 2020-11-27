use actix_web::web;

use self::root::init_schema;

pub mod message_type;
pub mod root;
pub mod service_application;
pub mod service_provider;

use crate::{cache::RedisCache, database::DatabasePool};

pub struct DataSource {
    pub database: DatabasePool,
    pub cache: RedisCache,
}

impl juniper::Context for DataSource {}

pub fn add_graphql(cfg: &mut web::ServiceConfig) {
    let schema = init_schema();
    cfg.data(schema);
}
