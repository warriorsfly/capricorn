// use std::{collections::BTreeMap, error::Error};

// use redis::{
//     streams::{StreamKey, StreamMaxlen, StreamReadOptions, StreamReadReply},
//     Commands, Connection, ErrorKind, RedisError, RedisResult,
// };

// use crate::schemas::lab::LabMessage;

// pub async fn xadd<'a>(
//     connection: &'a mut Connection,
//     uid: i32,
//     message: LabMessage,
// ) -> RedisResult<String> {
//     let key = format!("lab_message:{}", uid);
//     let maxlen = StreamMaxlen::Approx(10000);
//     let rr: RedisResult<String> = connection.xadd_maxlen_map(key, maxlen, "*", message);
//     rr
// }
// ///Read messages from redis stream
// pub async fn xread_opts<'a>(
//     connection: &'a mut Connection,
//     key: &str,
//     start: &str,
//     end: &str,
// ) -> StreamReadReply {
//     // let key = format!("lab_message:{}", uid);
//     let keys = vec![key];
//     let opts = StreamReadOptions::default();

//     let srr: StreamReadReply = connection
//         .xread_options(&keys, &[start, end], opts)
//         .expect("read");

//     srr
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[actix_rt::test]
//     // async fn it_creates_new_application_cache_and_sets_and_reads_it() {
//     //     if !&CONFIG.redis_url.is_empty() {
//     //         let cache = get_cache();
//     //         set(cache.clone(), "testing", "123").await.unwrap();
//     //         let value = get(cache, "testing").await.unwrap();
//     //         assert_eq!(value, "123");
//     //     } else {
//     //         assert!(true);
//     //     }
//     // }

//     // #[actix_rt::test]
//     // async fn it_removes_an_entry_in_application_cache() {
//     //     if !&CONFIG.redis_url.is_empty() {
//     //         let cache = get_cache();
//     //         set(cache.clone(), "testing", "123").await.unwrap();
//     //         let value = get(cache.clone(), "testing").await.unwrap();
//     //         assert_eq!(value, "123");
//     //         delete(cache.clone(), "testing").await.unwrap();
//     //         let value = get(cache, "testing").await.unwrap();
//     //         assert_eq!(value, "");
//     //     } else {
//     //         assert!(true);
//     //     }
//     // }
// }
