use crate::{
    awc::add_awc, config::CONFIG, database::add_pool, handlers::add_graphql, routes::routes,
};

use actix_web::{middleware::Logger, web, App, HttpServer};

pub async fn serv() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let serve = HttpServer::new(move || {
        App::new()
            // 连接数据库
            .configure(add_pool)
            // 添加awc
            .configure(add_awc)
            // 添加跨域
            // .wrap(Cors::default().supports_credentials())
            // 添加日志
            .wrap(Logger::default())
            // 连接graphql
            .configure(add_graphql)
            .default_service(web::to(|| async { "404" }))
    });

    serve.bind(&CONFIG.server)?.run().await
}
