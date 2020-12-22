#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod cache;
mod config;
mod constants;
mod database;
mod jwt;
mod service;
mod token_granter;
mod token_store;
fn main() {
    println!("Hello, world!");
}
