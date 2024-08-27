use actix::{Actor, StreamHandler, Addr, AsyncContext};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};

// WebSocket actor to manage rig connections
pub struct RigWebSocket {
    pub clients: Arc<Mutex<Vec<Addr<RigWebSocket>>>>,
}

impl Actor for RigWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.clients.lock().unwrap().push(addr);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.clients.lock().unwrap().retain(|client| client != &addr);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RigWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            // Log the received command
            println!("Received command: {}", text);

            // Here you would send the command to the rigs
            // Simulating sending command to rigs by just printing
            println!("Sending command '{}' to rigs", text);

            // Respond back to the frontend (optional)
            ctx.text(format!("Command '{}' sent to rigs", text));
        }
    }
}

// WebSocket connection handler for the rigs
pub async fn ws_rig(
    req: HttpRequest,
    stream: web::Payload,
    clients: web::Data<Arc<Mutex<Vec<Addr<RigWebSocket>>>>>,
) -> Result<HttpResponse, Error> {
    ws::start(RigWebSocket { clients: clients.get_ref().clone() }, &req, stream)
}