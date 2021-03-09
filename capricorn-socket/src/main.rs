use actix::Actor;
use std::sync::{atomic::AtomicUsize, Arc};

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use handlers::socket_route;

mod handlers;
mod message;
mod route;
mod session;
mod socket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Arc::new(AtomicUsize::new(0));

    // Start chat server actor
    let server = socket::CapricornServer::new(app_state.clone()).start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .data(server.clone())
            // redirect to websocket.html
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/static/websocket.html")
                    .finish()
            })))
            // .route("/count/", web::get().to(get_count))
            // websocket
            .service(web::resource("/ws/").to(socket_route))
            // static resources
            .service(fs::Files::new("/static/", "static/"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
