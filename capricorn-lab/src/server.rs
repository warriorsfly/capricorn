use crate::{
    awc::add_awc,
    config::CONFIG,
    datasource::{add_pool, add_redis},
    routes::routes,
};

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Compress, Logger},
    App, HttpServer,
};

pub async fn serv() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let serve = HttpServer::new(move || {
        App::new()
            // .wrap(middlewares::JwtAuthorization)
            // 连接数据库
            .configure(add_pool)
            // 添加redis client
            .configure(add_redis)
            // 添加awc
            .configure(add_awc)
            // 添加压缩
            .wrap(Compress::default())
            // 添加日志
            .wrap(Logger::default())
            // 添加跨域支持
            .wrap(
                Cors::default()
                    // .allowed_origin(&CONFIG.server)
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            // 连接graphql
            .configure(routes)
    });

    serve.bind(&CONFIG.server)?.run().await
}
