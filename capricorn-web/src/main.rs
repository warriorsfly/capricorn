#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;

mod awc;
mod cache;
mod config;
mod errors;
mod schemas;
mod server;
mod state;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::serv().await
}
