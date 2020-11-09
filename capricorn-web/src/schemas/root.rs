use crate::{database::DatabasePool, schema::*};
use diesel::prelude::*;
use juniper::{object, FieldResult, RootNode};

use super::provider::Provider;
pub struct Context {
    pub database_pool: DatabasePool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[object(Context=Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn providers(ctx: &Context) -> FieldResult<Vec<Provider>> {
        let conn = &ctx.database_pool.get()?;
        let providers = providers::table.load::<Provider>(conn)?;
        Ok(providers)
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
