//! `ChatCloud` is an actor. It maintains list of connection client sessions.
//! And manages available rooms. Peers send messages to other peers in same
//! room througn `ChatServer`.
use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

use std::{
    borrow::Borrow,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use std::collections::{HashMap, HashSet};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Notification {
    pub id: usize,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CMessage {
    /// id of client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

impl Handler<CMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: CMessage, _: &mut Self::Context) -> Self::Result {
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
    }
}

pub struct Server {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl Server {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> Self {
        let mut rooms = HashMap::new();
        rooms.insert("Clients".to_owned(), HashSet::new());
        Self {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
            visitor_count,
        }
    }
}

impl Server {
    /// send message to room
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

    /// send notification to user
    fn send_notification(&self, id: usize, message: &str) {
        if let Some(addr) = self.sessions.get(&id) {
            let _ = addr.do_send(Message(message.to_owned()));
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);
        self.rooms
            .entry("Clients".to_owned())
            .or_insert_with(HashSet::new)
            .insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message("Clients", &format!("Total visitors {}", count), 0);
        id
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        let mut rooms: Vec<String> = Vec::new();

        for (name, sessions) in &mut self.rooms {
            if sessions.remove(&msg.id) {
                rooms.push(name.to_owned());
            }
        }

        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}
