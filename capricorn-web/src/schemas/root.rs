use crate::{database::DatabasePool, schema::*};
use diesel::prelude::*;
use juniper::{object, EmptySubscription, FieldResult, RootNode};

use super::service_provider::ServiceProvider;
pub struct Context {
    pub database_pool: DatabasePool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[object(Context=Context)]
impl QueryRoot {
    #[graphql(description = "List of all service provider")]
    fn providers(ctx: &Context) -> FieldResult<Vec<ServiceProvider>> {
        let conn = &ctx.database_pool.get()?;
        let providers = service_providers::table.load::<ServiceProvider>(conn)?;
        Ok(providers)
    }

    #[graphql(description = "Get service provider by id")]
    fn provider(ctx: &Context, id: i32) -> FieldResult<ServiceProvider> {
        let conn = &ctx.database_pool.get()?;
        let provider = service_providers::table
            .find(id)
            .get_result::<ServiceProvider>(conn)?;
        Ok(provider)
    }
}

pub struct MutationRoot;

#[object(Context=Context)]
impl MutationRoot {}

pub struct Subscription;

#[object(Context=Context)]
impl Subscription {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<()>>;

pub fn init_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
    