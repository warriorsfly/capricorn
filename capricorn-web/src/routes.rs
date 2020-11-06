use crate::handlers;
use actix_web::{guard, web};
use actix_web_actors::ws;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(handlers::index))
        .route("/", web::get().to(handlers::index_playground))
        .route(
            "/",
            web::get()
                .to(handlers::index_ws)
                .guard(guard::Header("upgrade", "websocket")),
        );
}
