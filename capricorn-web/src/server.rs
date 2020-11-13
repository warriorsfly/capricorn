use crate::{
    awc::add_awc, config::CONFIG, database::add_pool, routes::routes, schemas::add_graphql,
};

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};

pub async fn serv() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let serve = HttpServer::new(move || {
        App::new()
            // 连接数据库
            .configure(add_pool)
            // 添加awc
            .configure(add_awc)
            .configure(add_graphql)
            // 添加跨域
            // .wrap(Cors::default().supports_credentials())
            // 添加日志
            .wrap(Logger::default())
            // .wrap(
            //     Cors::default()
            //         .allowed_origin(&CONFIG.server)
            //         .allowed_methods(vec!["POST", "GET"])
            //         .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //         .allowed_header(header::CONTENT_TYPE)
            //         .supports_credentials()
            //         .max_age(3600),
            // )
            // 连接graphql
            .configure(routes)
    });

    serve.bind(&CONFIG.server)?.run().await
}
