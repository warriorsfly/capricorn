use crate::{constants, datasource::RedisPool, schemas::lab::LabMessage, utils::jwt::Claims};
use actix::prelude::*;
use actix::Actor;
use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use redis::{
    streams::{StreamId, StreamKey, StreamRangeReply, StreamReadOptions, StreamReadReply},
    Commands, Value,
};

use std::{sync::Arc, time::Instant};

pub async fn ws_lab_index(
    req: HttpRequest,
    pool: Data<RedisPool>,
    // claims: Data<Claims>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // pub async fn ws_lab_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // println!("{:?}", r);
    let res = ws::start(LabSocket::new(&pool), &req, stream);
    println!("{:?}", res);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct LabSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    srr: StreamReadReply,
    hb: Instant,
}

impl LabSocket {
    fn new(pool: &RedisPool) -> Self {
        let mut connection = pool.get_connection().expect("redis connection error");
        // let key = format!("lab_message:{}", uid);
        let keys = vec!["lab-message"];
        let opts = StreamReadOptions::default();
        // Oldest known time index
        let start = "$";
        let srr: StreamReadReply = connection
            .xread_options(&keys, &[start], opts)
            // .xread_options(&keys, &[starting_id], opts)
            .expect("read");
        Self {
            srr,
            hb: Instant::now(),
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
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
        ctx.run_interval(constants::HEARTBEAT_STREAM, |act, ctx| {
            for StreamKey { key: _, ids } in &act.srr.keys {
                let messages: Vec<LabMessage> =
                    ids.iter().map(|item| LabMessage::from(item)).collect();

                for message in messages {
                    ctx.text(serde_json::to_string(&message).unwrap());
                }
            }
        });
    }
}

impl Actor for LabSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        //cancel heart beat for now
        // self.hb(ctx);
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

#[cfg(test)]
mod tests {

    use crate::tests::helpers::tests::assert_get;

    #[actix_rt::test]
    async fn test_socket_connect() {
        assert_get("ws://localhost:3000/ws/lab").await;
    }
}
