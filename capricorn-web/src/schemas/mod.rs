use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::{config::CONFIG, database::init_pool};

use self::user::QueryRoot;

pub mod user;

pub type IndexSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn init_schema() -> IndexSchema {
    let pool = init_pool(CONFIG.clone())
        .await
        .expect("Failed to create database connection pool");
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}
