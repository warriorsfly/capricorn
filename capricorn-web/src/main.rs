#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

mod awc;
mod config;
mod constants;
mod datasource;
mod handlers;
mod lab_socket;
mod middlewares;
mod routes;
mod schema;
mod schemas;
mod server;
mod state;
mod tests;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::serv().await
}
