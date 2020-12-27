use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid;

use crate::schema::services;

#[derive(Debug, Deserialize, Queryable, PartialEq, Serialize)]
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
    pub provider: &'a i32,
    pub slug: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub icon: &'a str,
    pub secret: &'a str,
    pub key: &'a str,
    pub enabled: &'a bool,
    pub created_at: &'a DateTime<Utc>,
    pub updated_at: &'a DateTime<Utc>,
}
