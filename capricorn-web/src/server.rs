use crate::{awc::add_awc, config::CONFIG, database::init_pool, schemas::IndexSchema};
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

pub async fn serv() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = init_pool(CONFIG.clone())
        .await
        .expect("Failed to create database connection pool");

    let schema = IndexSchema::new();
    let serve = HttpServer::new(move || {
        App::new()
            // 添加缓存
            // .configure(add_cache)
            // 添加awc
            .configure(add_awc)
            // 添加跨域
            .wrap(Cors::default().supports_credentials())
            // 添加日志
            .wrap(Logger::default())
            // 连接数据库
            .data(pool.clone())
        // 添加状态
        // .app_data(data.clone())
        // 注册Graphql
        // .configure(add_graphql)
    });

    serve.bind(&CONFIG.server)?.run().await
}
