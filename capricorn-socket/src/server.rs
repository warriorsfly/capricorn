//! `ChatCloud` is an actor. It maintains list of connection client sessions.
//! And manages available rooms. Peers send messages to other peers in same
//! room througn `ChatServer`.
use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use std::collections::{HashMap, HashSet};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DisConnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Notification {
    pub id: usize,
    pub message: String,
}

pub struct ChatCloud {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatCloud {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> Self {
        Self {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rng: rand::thread_rng(),
            visitor_count,
        }
    }
}

impl ChatCloud {
    /// send message to team
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

impl Actor for ChatCloud {
    type Context = Context<Self>;
}
