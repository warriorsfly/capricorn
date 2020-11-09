use super::{application::Application, root::Context};
use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::{object, FieldResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Queryable, Identifiable, PartialEq, Serialize)]
pub struct Provider {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[object(Context=Context)]
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

    fn applications(&self, ctx: &Context) -> FieldResult<Vec<Application>> {
        use crate::schema::applications::dsl::*;
        let conn = ctx.database_pool.get()?;

        let apps = applications
            .filter(provider.eq(self.id))
            .load::<Application>(&conn)?;
        Ok(apps)
    }
}
