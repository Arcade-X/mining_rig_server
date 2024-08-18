use actix::{Actor, StreamHandler, Addr, AsyncContext, Handler, Message};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use actix_web::{web, HttpRequest, HttpResponse, Error};
use tokio::task;

// WebSocket message structure
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
struct ServerMessage(String);

// WebSocket actor to manage connections and messaging
pub struct MyWebSocket {
    pub clients: Arc<Mutex<HashSet<Addr<MyWebSocket>>>>, // Shared list of WebSocket clients
}

impl MyWebSocket {
    // Initialize a new WebSocket instance with a shared client list
    pub fn new(clients: Arc<Mutex<HashSet<Addr<MyWebSocket>>>>) -> Self {
        Self { clients }
    }
}

// Implementing the Actor trait for MyWebSocket to make it work as an actor
impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    // Called when WebSocket starts
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.clients.lock().unwrap().insert(addr);
    }

    // Called when WebSocket stops
    fn stopped(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.clients.lock().unwrap().remove(&addr);
    }
}

// Handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received command: {}", text); // Debug log for received command
                ctx.text(format!("Server received: {}", text));
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            _ => (),
        }
    }
}

// Custom handler for incoming messages
impl Handler<ServerMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

// WebSocket connection handler for incoming connections
pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    clients: web::Data<Arc<Mutex<HashSet<Addr<MyWebSocket>>>>>,
) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(clients.get_ref().clone()), &req, stream)
}

// Function to listen for commands (this was missing)
pub async fn listen_for_commands() {
    println!("Listening for WebSocket commands...");
    // Here you can add the logic to process commands received over WebSocket
    // For example, periodically checking for messages or interacting with connected clients
    loop {
        // Example debug message
       
        task::yield_now().await;
    }
}