use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Debug, Deserialize, Queryable, Identifiable, PartialEq, Serialize)]
pub struct Service {
    pub id: uuid::Uuid,
    pub provider: i32,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub secret: String,
    pub key: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "services"]
pub struct ServiceInput<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub avatar: &'a str,
}
