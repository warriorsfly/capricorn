use super::{service_application::ServiceApplication, DataSource};
use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Queryable, Identifiable, PartialEq, Serialize)]
pub struct ServiceProvider {
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
impl ServiceProvider {
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

    fn applications(&self, ctx: &DataSource) -> FieldResult<Vec<ServiceApplication>> {
        use crate::schema::service_applications::dsl::*;
        let conn = ctx.database.get()?;

        let apps = service_applications
            .filter(provider.eq(self.id))
            .load::<ServiceApplication>(&conn)?;
        Ok(apps)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "service_providers"]
pub struct ProviderInput<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub avatar: &'a str,
}
