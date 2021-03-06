use crate::{database::DatabasePool, schema::*};
use diesel::prelude::*;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};

use super::provider::Provider;
pub struct DataSource {
    pub database: DatabasePool,
}

impl juniper::Context for DataSource {}

pub struct Query;

#[graphql_object(Context = DataSource)]
impl Query {
    #[graphql(description = "List of all service provider")]
    fn providers(ctx: &DataSource) -> FieldResult<Vec<Provider>> {
        let conn = &ctx.database.get()?;
        let providers = providers::table.load::<Provider>(conn)?;
        Ok(providers)
    }

    #[graphql(arguments(id(description = "id of the provider")))]
    fn provider(ctx: &DataSource, id: i32) -> FieldResult<Provider> {
        let conn = &ctx.database.get()?;
        let provider = providers::table.find(id).get_result::<Provider>(conn)?;
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
