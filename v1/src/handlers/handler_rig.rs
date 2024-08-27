use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Farm {
    pub id: i64,
    pub name: String,
    pub location: String,
}

// This function can be used to handle the creation of a farm
pub async fn create_farm(data: web::Json<Farm>) -> HttpResponse {
    // Perform your database insert operation here
    println!("Creating farm: {:?}", data);
    HttpResponse::Ok().json("Farm created successfully")
}

// Similarly, add functions for `edit_farm`, `delete_farm`, `show_rigs`, etc.
// These functions can be called from ws_frontend.rs based on the incoming WebSocket messages.