use std::{collections::BTreeMap, error::Error};

use redis::{
    streams::{StreamKey, StreamMaxlen, StreamReadOptions, StreamReadReply},
    Commands, ErrorKind, RedisError, RedisResult,
};

use crate::{constants, datasource::RedisPool, schemas::lab::LabMessage};

pub async fn publish_redis_message(
    redis: &RedisPool,
    uid: i32,
    lab_message: LabMessage,
) -> RedisResult<String> {
    let mut redis_connection = redis.get_connection()?;
    let key = format!("lab_message:{}", uid);
    let maxlen = StreamMaxlen::Approx(constants::STREAM_LEN);
    let rr: RedisResult<String> = redis_connection.xadd_maxlen_map(key, maxlen, "*", lab_message);
    rr
}
///Read messages from redis stream
pub async fn subscribe_redis_message(redis: &RedisPool, key: String) -> RedisResult<String> {
    let mut redis_connection = redis.get_connection()?;
    // let key = format!("lab_message:{}", uid);
    let keys = vec![key];
    let opts = StreamReadOptions::default();
    // Oldest known time index
    let starting_id = "0-0";
    // Same as above
    let another_form = "0";

    let srr: StreamReadReply = redis_connection
        .xread_options(&keys, &[starting_id, another_form, starting_id], opts)
        .expect("read");

    for StreamKey { key, ids } in srr.keys {}

    Ok("".into())
}
