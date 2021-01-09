use crate::{constants, datasource::RedisPool, schemas::lab::LabMessage, utils::jwt::Claims};
use actix::prelude::*;
use actix::Actor;
use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use redis::{
    streams::{StreamId, StreamKey, StreamReadOptions, StreamReadReply},
    Commands, Connection,
};
use web::Json;

use std::{sync::Arc, time::Instant};

pub async fn ws_lab_index(
    req: HttpRequest,
    pool: Data<RedisPool>,
    // claims: Data<Claims>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // pub async fn ws_lab_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // println!("{:?}", r);

    let key = format!("lab_message:{}", 1);
    let res = ws::start(LabSocket::new(pool.into_inner(), key), &req, stream);
    println!("{:?}", res);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct LabSocket {
    /// Redis Connection for load data
    pool: Arc<RedisPool>,
    key: String,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl LabSocket {
    fn new(pool: Arc<RedisPool>, key: String) -> Self {
        Self {
            pool,
            key,
            hb: Instant::now(),
        }
    }

    fn heart_beat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(constants::HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) >= constants::CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn subscribe_lab(&self, ctx: &mut <Self as Actor>::Context) {
        let mut connection = Arc::clone(&self.pool)
            .get_connection()
            .expect("redis connection error");
        // let key = format!("lab_message:{}", uid);
        let keys = vec![self.key.clone()];
        let opts = StreamReadOptions::default();
        // Oldest known time index
        let starting_id = "0-0";
        // Same as above
        let end_id = "0";

        let srr: StreamReadReply = connection
            .xread_options(&keys, &[starting_id, end_id, starting_id], opts)
            .expect("read");

        for StreamKey { key, ids } in srr.keys {
            let messages: Vec<LabMessage> = ids
                .iter()
                .map(|item| item.get::<LabMessage>(&key).unwrap())
                .collect();

            for message in messages {
                ctx.text(message);
            }
        }
    }
}

impl Actor for LabSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heart_beat(ctx);
        self.subscribe_lab(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LabSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("WS:{:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bytes)) => ctx.binary(bytes),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
