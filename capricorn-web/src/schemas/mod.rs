use actix_web::web;

use self::root::init_schema;

pub mod message_type;
pub mod root;
pub mod service_application;
pub mod service_provider;

pub fn add_graphql(cfg: &mut web::ServiceConfig) {
    let schema = init_schema();
    cfg.data(schema);
}
