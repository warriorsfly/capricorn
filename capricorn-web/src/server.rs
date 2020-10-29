use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

use crate::{awc::add_awc, cache::add_cache, config::CONFIG};

pub async fn serv() -> std::io::Result<()> {
    dotenv::dotenv().ok();

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
        // .configure(add_pool)
        // 添加状态
        // .app_data(data.clone())
        // 注册路由
        // .configure(routes)
    });

    serve.bind(&CONFIG.server)?.run().await
}
