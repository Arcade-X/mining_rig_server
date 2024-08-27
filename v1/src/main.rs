use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use actix::Addr;
use sqlx::SqlitePool; // Import SqlitePool
use dotenv::dotenv;

mod handlers; // Declare the `handlers` module

use handlers::ws_frontend::{ws_frontend, FrontendWebSocket};
use handlers::ws_rig::{ws_rig, RigWebSocket};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables
    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await.expect("Failed to create pool.");
    
    let frontend_clients = Arc::new(Mutex::new(Vec::<Addr<FrontendWebSocket>>::new()));
    let rig_clients = Arc::new(Mutex::new(Vec::<Addr<RigWebSocket>>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(frontend_clients.clone()))
            .app_data(web::Data::new(rig_clients.clone()))
            .app_data(web::Data::new(pool.clone())) // Provide the pool here
            .route("/ws/frontend", web::get().to(ws_frontend))
            .route("/ws/rig", web::get().to(ws_rig))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}