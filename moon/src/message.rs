use actix::prelude::*;
use bytes::Bytes;

/// text message
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);

/// bytes message
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct BinaryMessage(Bytes);

///Join in room
#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<TextMessage>);
/// Leave out Room
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);

/// `SendMessage.0`: room
/// `SendMessage.1`: sess
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String, pub usize, pub String);
