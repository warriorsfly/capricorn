use super::{root::DataSource, service_application::ServiceApplication};
use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    pub id: i32,
    pub title: String,
    pub discription: String,
    pub sender: String,
    pub reciver: String,
    pub timeout: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
