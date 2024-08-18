use actix_files::{Files, NamedFile}; // For serving static files
use actix_web::{web, App, HttpServer, Error, HttpRequest, HttpResponse}; // Main web-related imports
use actix_web_actors::ws::{self, WebsocketContext}; // WebSocket support
use actix::{Actor, StreamHandler, Addr, AsyncContext, Handler}; // Actix actor framework and Handler trait
use dotenv::dotenv; // For loading environment variables
use sqlx::SqlitePool; // For database connection pooling
use std::env; // For environment variable access
use std::io; // For standard I/O operations
use std::collections::HashSet; // For storing WebSocket clients
use std::sync::{Arc, Mutex}; // For thread-safe shared state management
use serde::{Serialize, Deserialize}; // Serialization and deserialization

// Import your handlers
mod handlers;
use handlers::gpu::{get_gpus, create_gpu, delete_gpu};  // Import GPU handlers
use handlers::websocket_handler::{listen_for_commands}; // Import WebSocket handler

// -----------------------------------
// GPU Data Models
// -----------------------------------

#[derive(Serialize, Deserialize, Debug)]
struct Gpu {
    id: i64,
    name: String,
    temp: f64,
    watt: f64,
}

// -----------------------------------
// WebSocket Struct and Implementations
// -----------------------------------

struct MyWebSocket {
    clients: Arc<Mutex<HashSet<Addr<MyWebSocket>>>>, // Thread-safe client list
}

impl MyWebSocket {
    // Initialize a new WebSocket instance with a shared client list
    fn new(clients: Arc<Mutex<HashSet<Addr<MyWebSocket>>>>) -> Self {
        MyWebSocket { clients }
    }
}

// Implementing the Actor trait for MyWebSocket to make it work as an actor
impl Actor for MyWebSocket {
    type Context = WebsocketContext<Self>;

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
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            _ => (),
        }
    }
}

// -----------------------------------
// WebSocket Route Handler
// -----------------------------------

async fn ws_index(
    r: HttpRequest, 
    stream: web::Payload, 
    clients: web::Data<Arc<Mutex<HashSet<Addr<MyWebSocket>>>>>
) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(clients.get_ref().clone()), &r, stream)
}

// -----------------------------------
// Custom Message Struct for WebSocket Communication
// -----------------------------------

struct MyWebSocketMessage(String);

impl actix::Message for MyWebSocketMessage {
    type Result = ();
}

impl Handler<MyWebSocketMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: MyWebSocketMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

// -----------------------------------
// Serve the index.html file
// -----------------------------------

async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("./static/index.html")
}

// -----------------------------------
// Main Function and Server Setup
// -----------------------------------

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap_or_else(|e| {
        eprintln!("Failed to connect to database: {}", e);
        std::process::exit(1);
    });

    let clients = Arc::new(Mutex::new(HashSet::<Addr<MyWebSocket>>::new()));

    // Start the WebSocket listener
    tokio::spawn(listen_for_commands());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the database pool
            .app_data(web::Data::new(clients.clone())) // Share WebSocket client state
            .route("/", web::get().to(index)) // Serve the index file
            .route("/gpus", web::get().to(get_gpus)) // Get GPUs
            .route("/gpus", web::post().to(create_gpu)) // Create a GPU
            .route("/gpus/{id}", web::delete().to(delete_gpu)) // Delete a GPU
            .route("/ws/", web::get().to(ws_index)) // WebSocket route
            .service(Files::new("/js", "./static/js"))  // Serve static JS files
            .service(Files::new("/css", "./static/css"))  // Serve static CSS files
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}