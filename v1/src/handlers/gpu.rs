use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

// Data structure for an existing GPU entry
#[derive(Serialize, Deserialize, Debug)]
pub struct Gpu {
    pub id: i64,
    pub name: String,
    pub temp: f64,
    pub watt: f64,
    pub rig_id: Option<i64>,
}

// Handler to add a new GPU to the database
pub async fn add_gpu(pool: web::Data<SqlitePool>, gpu: web::Json<Gpu>) -> impl Responder {
    let rig_id = gpu.rig_id;  // Assuming the rig_id is provided in the request

    let rig_id = match rig_id {
        Some(id) => id,
        None => {
            // Insert a new rig with a default name and location if rig_id is not provided
            sqlx::query!(
                "INSERT INTO rig (name, mac_address, location) VALUES (?, ?, ?)",
                "New Rig",
                "Unknown MAC",  // You might want to pass this information differently
                "Unknown Location"
            )
            .execute(pool.get_ref())
            .await
            .expect("Failed to insert new rig");

            sqlx::query_scalar!(
                "SELECT last_insert_rowid() as id"
            )
            .fetch_one(pool.get_ref())
            .await
            .expect("Failed to retrieve rig ID") as i64
        }
    };

    // Insert the GPU associated with this rig
    let result = sqlx::query!(
        "INSERT INTO gpu (name, temp, watt, rig_id) VALUES (?, ?, ?, ?)",
        gpu.name,
        gpu.temp,
        gpu.watt,
        rig_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("GPU added"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error adding GPU: {:?}", err)),
    }
}