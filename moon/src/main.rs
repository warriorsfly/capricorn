use server::serv;

mod handlers;
mod message;
// mod route;
mod moon;
mod server;
mod session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
