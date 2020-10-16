use actix::prelude::*;

/// text message
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);
///Join in room
#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<TextMessage>);
/// Leave out Room
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);
