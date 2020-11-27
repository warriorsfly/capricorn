use crate::{cache::RedisCache, schema::*};
use diesel::prelude::*;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};

use super::{service_provider::ServiceProvider, DataSource};

pub struct MessageProducer {
    pub cache: RedisCache,
    pub kafka: String,
}
impl juniper::Context for MessageProducer {}

pub struct MessageConsumer {
    pub cache: RedisCache,
    pub kafka: String,
}
impl juniper::Context for MessageConsumer {}

pub struct Query;

#[graphql_object(Context = DataSource)]
impl Query {
    #[graphql(description = "List of all service provider")]
    fn providers(ctx: &DataSource) -> FieldResult<Vec<ServiceProvider>> {
        let conn = &ctx.database.get()?;
        let providers = service_providers::table.load::<ServiceProvider>(conn)?;
        Ok(providers)
    }

    #[graphql(arguments(id(description = "id of the provider")))]
    fn provider(ctx: &DataSource, id: i32) -> FieldResult<ServiceProvider> {
        let conn = &ctx.database.get()?;
        let provider = service_providers::table
            .find(id)
            .get_result::<ServiceProvider>(conn)?;
        Ok(provider)
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<DataSource>, EmptySubscription<DataSource>>;

pub(crate) fn init_schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<DataSource>::new(),
        EmptySubscription::<DataSource>::new(),
    )
}
