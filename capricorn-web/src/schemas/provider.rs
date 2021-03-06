use super::{function::Function, root::DataSource};
use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Queryable, Identifiable, PartialEq, Serialize)]
pub struct Provider {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub avatar: String,
    #[serde(skip_serializing)]
    pub salt: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[graphql_object(Context = DataSource)]
impl Provider {
    fn id(&self) -> &i32 {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn avatar(&self) -> &str {
        &self.avatar
    }

    fn functions(&self, ctx: &DataSource) -> FieldResult<Vec<Function>> {
        use crate::schema::functions::dsl::*;
        let conn = ctx.database.get()?;

        let apps = functions
            .filter(provider.eq(self.id))
            .load::<Function>(&conn)?;
        Ok(apps)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "providers"]
pub struct ProviderInput<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub avatar: &'a str,
}
