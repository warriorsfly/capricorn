use crate::{awc::add_awc, config::CONFIG, routes::routes, schemas::init_schema};
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

pub async fn serv() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let schema = init_schema().await;
    let serve = HttpServer::new(move || {
        App::new()
            // 添加缓存
            // .configure(add_cache)
            // 添加awc
            .configure(add_awc)
            // 添加跨域
            // .wrap(Cors::default().supports_credentials())
            // 添加日志
            .wrap(Logger::default())
            // 连接数据库
            .data(schema.clone())
            // 注册路由
            .configure(routes)
    });

    serve.bind(&CONFIG.server)?.run().await
}
