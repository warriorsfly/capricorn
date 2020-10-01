use actix::prelude::*;

/// 文字消息
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);
///加入群组
#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<TextMessage>);
/// 退出群
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);
