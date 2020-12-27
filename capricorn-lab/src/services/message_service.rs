use redis::{
    streams::{StreamKey, StreamReadOptions, StreamReadReply},
    Commands, RedisResult,
};

use crate::datasource::RedisPool;

/// application private streams
pub const PRIVATE_APPLICATION_STREAM: &str = "private-message";
/// collab stream for all applications
pub const COLLAB_APPLICATION_STREAM: &str = "collab-stream";

pub const STREAMS: &[&str] = &[PRIVATE_APPLICATION_STREAM, COLLAB_APPLICATION_STREAM];

// pub async fn add_message(
//     redis: &RedisPool,
//     application: i32,
//     uid: i32,
//     message: Message,
// ) -> RedisResult<()> {
//     Ok(())
// }
///Read messages from redis stream
pub async fn subscribe_message(redis: &RedisPool) -> RedisResult<String> {
    let mut redis_connection = redis.get_connection()?;

    let opts = StreamReadOptions::default();
    // Oldest known time index
    let starting_id = "0-0";
    // Same as above
    let another_form = "0";

    let srr: StreamReadReply = redis_connection
        .xread_options(STREAMS, &[starting_id, another_form, starting_id], opts)
        .expect("read");

    for StreamKey { key, ids } in srr.keys {}

    Ok("".into())
}
