use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler};
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use dyno_core::crossbeam_channel::Sender;

use crate::ServerState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Msg(Vec<u8>);

impl actix::Message for Msg {
    type Result = ();
}

#[derive(Debug)]
pub enum WsMessage {
    Conn(Addr<WsConn>),
    Disconn(Addr<WsConn>),
    Msg(Msg),
}

pub struct WsConn {
    tx: Sender<WsMessage>,
}

impl WsConn {
    pub fn new(tx: Sender<WsMessage>) -> Self {
        Self { tx }
    }
}
impl Handler<Msg> for WsConn {
    type Result = ();
    fn handle(&mut self, msg: Msg, ctx: &mut Self::Context) -> Self::Result {
        ctx.binary(msg.0)
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        if let Err(err) = self.tx.send(WsMessage::Conn(addr)) {
            dyno_core::log::error!("MPSC SEND ERROR: {err}")
        }
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        if let Err(err) = self.tx.send(WsMessage::Disconn(addr)) {
            dyno_core::log::error!("MPSC SEND ERROR: {err}")
        }
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                if let Err(err) = self.tx.send(WsMessage::Msg(Msg(msg.to_vec()))) {
                    dyno_core::log::error!("MPSC SEND ERROR: {err}")
                }
            }
            Ok(ws::Message::Text(msg)) => {
                if let Err(err) = self.tx.send(WsMessage::Msg(Msg(msg.into_bytes().to_vec()))) {
                    dyno_core::log::error!("MPSC SEND ERROR: {err}")
                }
            }
            Ok(ws::Message::Binary(msg)) => {
                if let Err(err) = self.tx.send(WsMessage::Msg(Msg(msg.to_vec()))) {
                    dyno_core::log::error!("MPSC SEND ERROR: {err}")
                }
            }
            Ok(_) => {}
            Err(err) => dyno_core::log::error!("Websocket Error: {err}"),
        }
    }
}

#[get("/ws")]
pub async fn websocket_endpoint(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<ServerState>,
) -> HttpResponse {
    match ws::WsResponseBuilder::new(WsConn::new(data.ws_sender.clone()), &req, stream).start() {
        Ok(response) => response,
        Err(err) => err.error_response(),
    }
}
