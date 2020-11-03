#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate sqlx;

mod awc;
mod cache;
mod config;
mod database;
mod errors;
mod handlers;
mod schemas;
mod server;
mod state;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::serv().await
}
