use std::{time::{Duration, Instant}, sync::{Arc, Mutex}};

use actix::prelude::*;
use actix_web::{HttpRequest, http::StatusCode};
use actix_web_actors::ws;
use serde::Serialize;

use crate::handlers::handle_command;

const HEARBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);

pub struct WebSocket {
    hb: Instant,
    req: HttpRequest
}

#[derive(Serialize)]
struct ResponseBody {
    code: u16,
    message: String,
    data: Option<String>,
}

impl ResponseBody {
    fn new() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            message: String::new(),
            data: None,
        }
    }
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap_or_default()
    }
}

impl WebSocket {
    #[must_use]
    pub fn new(req: HttpRequest) -> Self {
        Self {
            hb: Instant::now(),
            req
        }
    }

    #[allow(clippy::unused_self)]
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
                let text = text.replace("|", " ");
                let command: Vec<String> = text.split_whitespace().map(|v| v.to_owned()).collect();
                let cmd = command.get(0).unwrap().clone();
                let args = command[1..].to_vec();
                let res: Arc<Mutex<ResponseBody>> = Arc::new(Mutex::new(ResponseBody::new()));
                let request = self.req.clone();
                async move {
                    let res_command = handle_command(cmd, args.clone(), request).await;

                    let res_command = match res_command {
                        Ok(v) => {
                            let val: Option<String>;
                            if v.is_empty() { val = None; } 
                            else { val = Some(v); }
                            
                            ResponseBody {
                                code: StatusCode::OK.as_u16(),
                                message: String::from("Ok"),
                                data: val,
                            }
                        },
                        Err(e) => {
                            ResponseBody {
                                code: e.code(),
                                message: e.to_string(),
                                data: None,
                            }
                        }
                    };
                    let mut res_guard = res.lock().unwrap();
                    *res_guard = res_command;
                    drop(res_guard);
                    res
                }.into_actor(self).map(move |res, _, ctx| {
                    let res_ref = res.lock().unwrap();
                    log::info!("Text message: {:?} resulted in {:?}", text, res_ref.data);
                    ctx.text(&*res_ref.to_string());
                }).wait(ctx);
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

