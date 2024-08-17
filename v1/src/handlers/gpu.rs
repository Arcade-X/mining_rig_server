use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

// Data structure for GPU
#[derive(Serialize, Deserialize, Debug)]
pub struct NewGpu {
    pub name: String,
    pub temp: f64,
    pub watt: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gpu {
    pub id: i64,
    pub name: String,
    pub temp: f64,
    pub watt: f64,
}

// Handler to get all GPUs
pub async fn get_gpus(pool: web::Data<SqlitePool>) -> impl Responder {
    let gpus = sqlx::query_as!(Gpu, "SELECT id, name, temp, watt FROM gpu")
        .fetch_all(pool.get_ref())
        .await;

    match gpus {
        Ok(gpu_list) => HttpResponse::Ok().json(gpu_list),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching GPU data"),
    }
}

// Handler to add a new GPU
pub async fn create_gpu(pool: web::Data<SqlitePool>, gpu: web::Json<NewGpu>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO gpu (name, temp, watt) VALUES (?, ?, ?)",
        gpu.name,
        gpu.temp,
        gpu.watt
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            println!("Inserted GPU successfully: {:?}", gpu);
            HttpResponse::Ok().json(gpu.into_inner())
        }
        Err(err) => {
            eprintln!("Failed to insert GPU: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Error adding GPU: {:?}", err))
        }
    }
}

// Handler to delete a GPU by ID
pub async fn delete_gpu(pool: web::Data<SqlitePool>, gpu_id: web::Path<i64>) -> impl Responder {
    let id = gpu_id.into_inner();

    let result = sqlx::query!("DELETE FROM gpu WHERE id = ?", id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("GPU deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting GPU"),
    }
}