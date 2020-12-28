use redis::{
    streams::{StreamKey, StreamReadOptions, StreamReadReply},
    Commands, RedisResult,
};

use crate::{datasource::RedisPool, schemas::mqtt::LabMessage};

/// service private streams
pub const SERVICE_STREAM: &str = "service-stream";
/// collab stream for all applications
pub const LAB_STREAM: &str = "lab-stream";

pub const STREAMS: &[&str] = &[SERVICE_STREAM, LAB_STREAM];

pub async fn collab(
    redis: &RedisPool,
    service_id: i32,
    uid: i32,
    message: LabMessage,
) -> RedisResult<()> {
    Ok(())
}
///Read messages from redis stream
pub async fn subscribe(redis: &RedisPool) -> RedisResult<String> {
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
