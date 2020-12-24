use super::DataSource;
use chrono::{DateTime, Utc};
use juniper::{graphql_object, graphql_subscription};
use redis::{
    streams::{StreamId, StreamKey, StreamMaxlen, StreamReadOptions, StreamReadReply},
    Commands,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CollabMessage {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub sender: String,
    pub reciver: String,
    pub timeout: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[graphql_object(Context = DataSource)]
impl CollabMessage {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn sender(&self) -> &str {
        &self.sender
    }

    pub fn reciver(&self) -> &str {
        &self.reciver
    }

    pub fn timeout(&self) -> &i32 {
        &self.timeout
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

// type CollabMessageStream = Pin<Box<dyn futures::Stream<Item = Result<Msg, FieldError>> + Send>>;
// struct Subscription;
// #[graphql_subscription(context = DataSource)]
// impl Subscription {
//     #[graphql(description = "message push service")]
//     async fn push_message(context: &DataSource) -> CollabMessageStream {
//         let mut connection = context.cache.get_connection()?;

//         let opts = StreamReadOptions::default();

//         let srr: StreamReadReply = connection
//             .xread_options(STREAMS, &[starting_id, another_form, starting_id], opts)
//             .expect("read messages error");
//     }
// }
