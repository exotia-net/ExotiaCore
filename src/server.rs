use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::handlers::handle_command;

const HEARBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);

pub struct WebSocket {
    hb: Instant,
}

impl WebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                log::warn!("Websocket Client heartbeat failed, disconnecting!");
                
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}


impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Recieved new connection");
        self.hb(ctx);
    }
}


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                if text.len() == 0 {
                    return;
                }
                let command: Vec<&str> = text.split_whitespace().collect();
                let cmd = command[0];
                let args = &command[1..].to_vec();
                let res = handle_command(cmd, args.clone());
                log::info!("Text message: {:?} resulted in {:?}", text, res);
                ctx.text(res);
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        } 
    }
}

