use crate::database::Pool;

use super::user::User;
use juniper::{FieldResult, RootNode};

pub(crate) struct Context {
    pub pool: Pool,
    // pub kv:
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    // #[graphql(description = "List of all users")]
    // pub async fn users(ctx: &Context) -> FieldResult<Vec<User>> {
    //     let users = sqlx::query_as!(User, r#"select * from users"#)
    //         .fetch_all(ctx.pool)
    //         .await?;

    //     Ok(users)
    // }
}

pub async fn users(ctx: &Context) -> FieldResult<Vec<User>> {
    let users = sqlx::query_as!(User, r#"select * from users"#)
        .fetch_all(ctx.pool)
        .await?;

    Ok(users)
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
