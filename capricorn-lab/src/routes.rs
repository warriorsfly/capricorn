use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // .service(
            //     web::scope("/auth")
            //         .route("signup", web::post().to(auth::signup))
            //         .route("login", web::post().to(auth::login)),
            // )
            // .service(
            //     web::scope("/users").route("/{id}", web::get().to(user::get_user)), // .route("", web::get().to(get_users))
            // )
            .service(
                web::scope("/messages"), // .route("", web::post().to(article::create_article))
                                         // .route("", web::get().to(article::search_articles)),
                                         // .route("/{id}", web::delete().to(delete_user))
                                         // .route("", web::get().to(get_users))
            ),
    );
}
