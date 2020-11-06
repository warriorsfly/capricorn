use async_graphql::{Context, Object};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    id: String,
    empi: String,
    #[serde(rename = "XM")]
    name: String,
}

#[Object]
impl User {
    async fn id(&self) -> &str {
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
    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        sqlx::query!(r#"select * from JBXX_INDEX"#)
            .fetch_all(ctx.data())
            .await
    }
}
