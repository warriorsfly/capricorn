use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::database::Pool;

#[derive(sqlx::FromRow, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    id: i32,
    empi: String,
    name: String,
}

#[Object]
impl User {
    async fn id(&self) -> &i32 {
        &self.id
    }

    async fn empi(&self) -> &str {
        &self.empi
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data_unchecked::<Pool>();
        sqlx::query_as::<_, User>("select * from jbxx_index")
            .fetch_all(pool)
            .await
            .map_err(|error| Error::from(error))
    }
}
