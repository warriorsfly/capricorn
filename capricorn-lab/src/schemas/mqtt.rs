use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// pushing receive platforms
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Platform {
    All,
    Android,
    Ios,
    Web,
    Windows,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Audience {
    pub all: Option<String>,
    pub tag: Option<Vec<String>>,
    pub tag_and: Option<Vec<String>>,
    pub tag_not: Option<Vec<String>>,
    pub alais: Option<Vec<String>>,
    pub registration_id: Option<Vec<String>>,
    pub segment: Option<String>,
    pub ab_test: Option<String>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct LabMessage {
    pub platforms: Platform,
    pub audience: Audience,
    pub notification: Option<Notification>,
    pub message: Option<Message>,
    pub cid: Option<String>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Message {
    pub title: String,
    pub content_type: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Notification {
    pub alert: String,
    pub content_type: String,
    pub content: String,
    pub extras: String,
    pub timeout: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
