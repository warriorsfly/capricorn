#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod awc;
mod config;
mod constants;
mod datasource;
mod handlers;
mod middlewares;
mod routes;
mod schema;
mod schemas;
mod server;
mod services;
mod state;
mod tests;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::serv().await
}
