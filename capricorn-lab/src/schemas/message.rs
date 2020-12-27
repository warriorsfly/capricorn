use chrono::{DateTime, Utc};
use redis::{
    streams::{StreamId, StreamKey, StreamMaxlen, StreamReadOptions, StreamReadReply},
    Commands,
};
use serde::{Deserialize, Serialize};

/// pushing receive platforms
pub enum Platform {
    All,
    Android,
    Ios,
    Web,
    Windows,
}

pub enum Audience {
    All,
    Tag(Vec<String>),
    TagAnd(Vec<String>),
    TagNot(Vec<String>),
    Alias(Vec<String>),
    RegistrationId(Vec<String>),
    Segment(String),
    Abtest(String),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Nofitication {
    pub app: i32,
    pub title: String,
    pub description: String,
    pub sender: i32,
    pub reciver: i32,
    pub timeout: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// type CollabMessageStream = Pin<Box<dyn futures::Stream<Item = Result<Msg, FieldError>> + Send>>;
// struct Subscription;
// #[graphql_subscription(context = DataSource)]
// impl Subscription {
//     #[graphql(description = "message push service")]
//     async fn subscribe(context: &DataSource) {
//         let mut redis_connection = context.redis.get_connection()?;

//         let opts = StreamReadOptions::default();

//         let srr: StreamReadReply = redis_connection
//             .xread_options(STREAMS, &[starting_id, another_form, starting_id], opts)
//             .expect("read");
//     }
// }
