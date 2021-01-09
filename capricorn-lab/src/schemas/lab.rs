use chrono::{DateTime, Utc};
use redis::{FromRedisValue, ToRedisArgs};
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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
pub struct LabMessageRequest {
    pub platforms: Platform,
    pub audience: Audience,
    pub notification: Option<LabNotification>,
    pub message: Option<LabMessage>,
    pub cid: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LabMessage {
    pub title: String,
    pub typ: String,
    pub content: String,
}

impl Into<String> for LabMessage {
    fn into(self) -> String {
        serde_json::to_string(&self).expect("error deserialize for LabMessage")
    }
}

impl ToRedisArgs for LabMessage {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(b"title");
        out.write_arg_fmt(&self.title);
        out.write_arg(b"typ");
        out.write_arg_fmt(&self.typ);
        out.write_arg(b"content");
        out.write_arg_fmt(&self.content);
    }
}

impl FromRedisValue for LabMessage {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let (title, typ, content) = FromRedisValue::from_redis_value(v)?;

        Ok(LabMessage {
            title,
            typ,
            content,
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LabNotification {
    pub alert: String,
    pub content_type: String,
    pub content: String,
    pub extras: String,
    pub timeout: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
