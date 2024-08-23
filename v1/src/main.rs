use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::env;
use std::io;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use actix::Addr;

// Import your handlers
mod handlers;
use handlers::rig::{get_rigs_with_gpus, update_rig_farm}; // Import the new handler to get rigs with GPUs and update farm
use handlers::farm::{get_farms, create_farm, update_farm, delete_farm}; // Updated imports
use handlers::websocket_handler::{listen_for_commands, MyWebSocket, ws_index};

async fn index() -> Result<NamedFile, actix_web::Error> {
    let path: std::path::PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
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
            .app_data(web::Data::new(pool.clone())) // Share the database pool
            .app_data(web::Data::new(clients.clone())) // Share WebSocket client state
            .route("/", web::get().to(index)) // Serve the index file
            .route("/rigs", web::get().to(get_rigs_with_gpus)) // Get Rigs with GPUs
            .route("/farms", web::get().to(get_farms)) // Get Farms
            .route("/farms", web::post().to(create_farm)) // Create a Farm
            .route("/farms/{id}", web::put().to(update_farm)) // Update a Farm
            .route("/farms/{id}", web::delete().to(delete_farm)) // Delete a Farm
            .route("/rigs/{id}/move", web::put().to(update_rig_farm)) // Route for moving a rig
            .route("/ws/", web::get().to(ws_index)) // WebSocket route
            .service(Files::new("/js", "./static/js"))  // Serve static JS files
            .service(Files::new("/css", "./static/css"))  // Serve static CSS files
    })
    .bind("0.0.0.0:8080")? // Listen on all network interfaces
    .run()
    .await
}