use actix::{Actor, StreamHandler, Addr, AsyncContext, Handler, Message};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use actix_web::{web, HttpRequest, HttpResponse, Error};

// Define the GpuData struct to match the structure sent by the client
#[derive(Serialize, Deserialize, Debug)]
pub struct GpuData {
    pub name: String,
    pub temp: f64,
    pub watt: f64,
    pub fan_speed: f64,
}

// WebSocket message structure
#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct ServerMessage(String);

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
                // Attempt to deserialize the message as GpuData
                if let Ok(gpu_data) = serde_json::from_str::<GpuData>(&text) {
                    println!("Received GPU data: {:?}", gpu_data);
                } else {
                    println!("Received command: {}", text); // Handle as a command if not GpuData
                    ctx.text(format!("Server received: {}", text));
                }
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

// Function to send commands to all connected clients
pub fn send_command_to_rigs(clients: web::Data<Arc<Mutex<HashSet<Addr<MyWebSocket>>>>>, command: &str) {
    let message = ServerMessage(command.to_string());
    let clients = clients.lock().unwrap();

    for client in clients.iter() {
        client.do_send(message.clone());
    }
    
    println!("Command '{}' sent to all connected rigs", command);
}

// Function to listen for commands (optional, for future use)
pub async fn listen_for_commands() {
    println!("Listening for WebSocket commands...");
    loop {
        tokio::task::yield_now().await;
    }
}