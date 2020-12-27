use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

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

#[derive(Debug, Insertable)]
#[table_name = "providers"]
pub struct ProviderInput<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub avatar: &'a str,
}
