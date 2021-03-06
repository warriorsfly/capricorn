use super::root::DataSource;
use crate::config::DATE_FORMAT;
use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::graphql_object;
use serde::{Deserialize, Serialize};
use uuid;
#[derive(Debug, Deserialize, Queryable, Identifiable, PartialEq, Serialize)]
pub struct Function {
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

#[graphql_object(Context = DataSource)]
impl Function {
    fn id(&self) -> String {
        (&self.id.to_simple().to_string()).to_owned()
    }

    fn provider(&self) -> &i32 {
        &self.provider
    }

    fn slug(&self) -> &str {
        &self.slug
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn icon(&self) -> &str {
        &self.icon
    }

    fn secret(&self) -> &str {
        &self.secret
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn enabled(&self) -> &bool {
        &self.enabled
    }

    fn created_at(&self) -> String {
        self.created_at.format(DATE_FORMAT).to_string()
    }

    fn updated_at(&self) -> String {
        self.updated_at.format(DATE_FORMAT).to_string()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "functions"]
pub struct FunctionInput<'a> {
    pub provider: &'a i32,
    pub slug: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub icon: &'a str,
    pub secret: &'a str,
    pub key: &'a str,
    pub enabled: &'a bool,
}
