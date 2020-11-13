#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod awc;
mod config;
mod database;
mod handlers;
mod routes;
mod schema;
mod schemas;
mod server;
mod state;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::serv().await
}
