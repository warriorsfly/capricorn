use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")).service(
        web::scope("/ws"), // .route("re", route)
    );
}
