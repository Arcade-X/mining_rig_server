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

// Data structure for creating a new Rig entry
#[derive(Serialize, Deserialize, Debug)]
pub struct NewRig {
    pub name: String,
    pub mac_address: String,
    pub location: Option<String>,
    pub farm_id: Option<i64>,
}

// Data structure for an existing Rig entry
#[derive(Serialize, Deserialize, Debug)]
pub struct Rig {
    pub id: i64,
    pub name: String,
    pub mac_address: String,
    pub location: Option<String>,
    pub farm_id: Option<i64>,
    pub gpus: Vec<Gpu>,  // Rigs now have a list of GPUs
}

// Make the MoveRigRequest struct public
#[derive(Deserialize)]
pub struct MoveRigRequest {
    pub farm_id: i64,
}

// Handler to get all Rigs and their GPUs from the database
pub async fn get_rigs_with_gpus(pool: web::Data<SqlitePool>) -> impl Responder {
    let rigs = sqlx::query!(
        r#"
            SELECT id, name, mac_address, location, farm_id
            FROM rig
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match rigs {
        Ok(rig_records) => {
            let mut rig_list = Vec::new();

            for rig_record in rig_records {
                let gpus = sqlx::query_as!(
                    Gpu,
                    "SELECT id, name, temp, watt, rig_id FROM gpu WHERE rig_id = ?",
                    rig_record.id
                )
                .fetch_all(pool.get_ref())
                .await
                .unwrap_or_else(|_| vec![]);

                rig_list.push(Rig {
                    id: rig_record.id,
                    name: rig_record.name,
                    mac_address: rig_record.mac_address,
                    location: rig_record.location,
                    farm_id: rig_record.farm_id,
                    gpus,  // Attach the GPUs to the rig
                });
            }

            HttpResponse::Ok().json(rig_list)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error fetching Rig data"),
    }
}

// Handler to update the farm associated with a rig by farm ID
pub async fn update_rig_farm(
    pool: web::Data<SqlitePool>,
    rig_id: web::Path<i64>,
    move_rig_request: web::Json<MoveRigRequest>,
) -> impl Responder {
    let rig_id_value = rig_id.into_inner();
    let new_farm_id_value = move_rig_request.farm_id;

    // Update the rig with the new farm ID
    let result = sqlx::query!(
        "UPDATE rig SET farm_id = ? WHERE id = ?",
        new_farm_id_value,
        rig_id_value,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Rig moved to new farm successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to move rig: {}", err)),
    }
}

// Handler to get all Rigs from the database (if needed separately)
pub async fn get_rigs(pool: web::Data<SqlitePool>) -> impl Responder {
    let rigs = sqlx::query!(
        r#"
            SELECT id, name, mac_address, location, farm_id
            FROM rig
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match rigs {
        Ok(rig_records) => {
            let mut rig_list = Vec::new();

            for rig_record in rig_records {
                rig_list.push(Rig {
                    id: rig_record.id,
                    name: rig_record.name,
                    mac_address: rig_record.mac_address,
                    location: rig_record.location,
                    farm_id: rig_record.farm_id,
                    gpus: Vec::new(),  // Initialize the gpus field as an empty vector
                });
            }

            HttpResponse::Ok().json(rig_list)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error fetching rig data"),
    }
}