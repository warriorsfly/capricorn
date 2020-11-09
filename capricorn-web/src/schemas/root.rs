use crate::{database::DatabasePool, schema::*};
use diesel::prelude::*;
use juniper::{object, FieldResult};

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
        let conn = ctx.database_pool.get()?;
        let users = users::table::get_results::<User>(conn)?;
        Ok(users)
    }
}
