use super::root::Context;
use crate::schema::*;
use chrono::{DateTime, Utc};
use juniper::object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Queryable, Identifiable, PartialEq, Serialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[object(Context=Context)]
impl User {
    fn id(&self) -> &i32 {
        &self.id
    }

    fn user_name(&self) -> &str {
        &self.user_name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn avatar(&self) -> &str {
        &self.avatar
    }
}
