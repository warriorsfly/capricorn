use crate::config::Config;

pub type Pool = sqlx::PgPool;
pub async fn init_pool(config: Config) -> sqlx::Result<Pool> {
    Pool::connect(&config.database_url).await
}

// pub async fn add_pool(cfg: &mut web::ServiceConfig) {
//     let pool = init_pool(CONFIG.clone())
//         .await
//         .expect("Failed to create connection pool");
//     cfg.data(pool);
// }
