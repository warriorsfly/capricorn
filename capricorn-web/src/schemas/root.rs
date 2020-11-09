use crate::{database::DatabasePool, schema::*};
use diesel::prelude::*;
use juniper::{object, FieldResult, RootNode};

use super::user::User;
pub struct Context {
    pub database_pool: DatabasePool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[object(Context=Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn users(ctx: &Context) -> FieldResult<Vec<User>> {
        let conn = &ctx.database_pool.get()?;
        let users = users::table.load::<User>(conn)?;
        Ok(users)
    }
}

pub struct MutationRoot;

#[object(Context=Context)]
impl MutationRoot {}

pub struct Subscription;

#[object(Context=Context)]
impl Subscription {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn init_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
