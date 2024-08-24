use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::env;
use std::io;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use actix::Addr;

// Import your handlers
mod handlers;
use handlers::rig::{get_rigs_with_gpus, update_rig_farm}; 
use handlers::farm::{get_farms, get_farm_by_id, create_farm, update_farm, delete_farm}; 
use handlers::server_websocket::{listen_for_commands, MyWebSocket, ws_index, send_command_to_rigs};

async fn index() -> Result<NamedFile, actix_web::Error> {
    let path: std::path::PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

// This is the async handler for sending commands to rigs
async fn send_command(
    clients: web::Data<Arc<Mutex<HashSet<Addr<MyWebSocket>>>>>,
    command: web::Path<String>,
) -> impl Responder {
    send_command_to_rigs(clients, &command);
    HttpResponse::Ok().body("Command sent")
}

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
            .app_data(web::Data::new(pool.clone())) 
            .app_data(web::Data::new(clients.clone())) 
            .route("/", web::get().to(index)) 
            .route("/rigs", web::get().to(get_rigs_with_gpus)) 
            .route("/farms", web::get().to(get_farms)) 
            .route("/farms", web::post().to(create_farm)) 
            .route("/farms/{id}", web::get().to(get_farm_by_id))  // <-- Added route
            .route("/farms/{id}", web::put().to(update_farm)) 
            .route("/farms/{id}", web::delete().to(delete_farm)) 
            .route("/rigs/{id}/move", web::put().to(update_rig_farm)) 
            .route("/ws/", web::get().to(ws_index)) 
            .route("/send-command/{command}", web::post().to(send_command)) // Command sending endpoint
            .service(Files::new("/js", "./static/js")) 
            .service(Files::new("/css", "./static/css")) 
    })
    .bind("0.0.0.0:8080")? 
    .run()
    .await
}