use server::serv;

mod handlers;
mod message;
mod route;
mod server;
mod session;
mod socket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
