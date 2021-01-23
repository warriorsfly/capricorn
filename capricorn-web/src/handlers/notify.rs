use crate::{
    constants,
    datasource::{cache::xread_opts, RedisPool},
    schemas::lab::LabMessage,
    utils::jwt::Claims,
};
use actix::Actor;
use actix::{clock::interval, prelude::*};
use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

use redis::{streams::StreamKey, Commands, Connection};

use std::time::Instant;

pub async fn ws_index(
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
    redis_client: &'static RedisPool,
    hb: Instant,
}

impl LabSocket {
    fn new(pool: &'static RedisPool) -> Self {
        Self {
            redis_client: pool,
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

    fn get_unread_messages(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(constants::HEARTBEAT_STREAM, |act, ctx| {
            let mut con = self.redis_client.get_connection().expect("read");
            let ssr = xread_opts(&con, constants::KEY_COLLAB_MESSAGE, "0", "0").await;
            for StreamKey { key: _, ids } in srr.keys {
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
        self.get_unread_messages(ctx);
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
