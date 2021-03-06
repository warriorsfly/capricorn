use actix_web::web;

use self::root::init_schema;
pub mod function;
pub mod message;
pub mod provider;
pub mod root;

pub fn add_graphql(cfg: &mut web::ServiceConfig) {
    let schema = init_schema();
    cfg.data(schema);
}
