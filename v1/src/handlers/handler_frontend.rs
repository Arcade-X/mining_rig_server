use serde::{Deserialize, Serialize};
use actix_web::web;
use serde_json::json;
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Farm {
    pub id: i64,
    pub name: String,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rig {
    pub id: i64,
    pub farm_id: i64,
    pub name: String,
}

// Create a new farm
pub async fn create_farm(pool: web::Data<SqlitePool>, data: web::Json<Farm>) -> String {
    let result = sqlx::query!(
        "INSERT INTO farm (name, location) VALUES (?, ?)",
        data.name,
        data.location
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => json!({ "status": "success", "message": "Farm created" }).to_string(),
        Err(e) => json!({ "status": "error", "message": e.to_string() }).to_string(),
    }
}

// Edit an existing farm
pub async fn edit_farm(pool: web::Data<SqlitePool>, id: web::Path<i64>, data: web::Json<Farm>) -> String {
    let farm_id = id.into_inner();
    let result = sqlx::query!(
        "UPDATE farm SET name = ?, location = ? WHERE id = ?",
        data.name,
        data.location,
        farm_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => json!({ "status": "success", "message": "Farm updated" }).to_string(),
        Err(e) => json!({ "status": "error", "message": e.to_string() }).to_string(),
    }
}

// Delete a farm
pub async fn delete_farm(pool: web::Data<SqlitePool>, id: web::Path<i64>) -> String {
    let farm_id = id.into_inner();
    let result = sqlx::query!("DELETE FROM farm WHERE id = ?", farm_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => json!({ "status": "success", "message": "Farm deleted" }).to_string(),
        Err(e) => json!({ "status": "error", "message": e.to_string() }).to_string(),
    }
}

// Show rigs for a specific farm
pub async fn show_rigs(pool: web::Data<SqlitePool>, farm_id: web::Path<Option<i64>>) -> String {
    if let Some(farm_id) = farm_id.into_inner() {
        println!("Showing rigs for farm with id {}", farm_id);

        let rigs: Vec<Rig> = sqlx::query_as!(
            Rig,
            "SELECT id, farm_id, name FROM rig WHERE farm_id = ?",
            farm_id
        )
        .fetch_all(pool.get_ref())
        .await
        .expect("Failed to fetch rigs");

        let response = serde_json::json!({
            "message": format!("Rigs for farm with id {}", farm_id),
            "rigs": rigs
        });

        response.to_string()
    } else {
        json!({ "status": "error", "message": "Invalid farm_id" }).to_string()
    }
}