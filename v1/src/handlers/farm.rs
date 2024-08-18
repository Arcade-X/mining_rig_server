use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use crate::handlers::rig::Rig;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewFarm {
    pub name: String,
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Farm {
    pub id: i64,
    pub name: String,
    pub location: Option<String>,
    pub rigs: Vec<Rig>,
}

pub async fn get_farms(pool: web::Data<SqlitePool>) -> impl Responder {
    let farms_with_rigs = sqlx::query!(
        r#"
        SELECT farm.id AS farm_id, farm.name AS farm_name, farm.location AS farm_location,
               rig.id AS rig_id, rig.name AS rig_name, rig.location AS rig_location,
               gpu.id AS gpu_id, gpu.name AS gpu_name, gpu.temp AS gpu_temp, gpu.watt AS gpu_watt
        FROM farm
        LEFT JOIN rig ON farm.id = rig.farm_id
        LEFT JOIN gpu ON rig.id = gpu.rig_id
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match farms_with_rigs {
        Ok(records) => {
            let mut farms: Vec<Farm> = Vec::new();

            for record in records {
                let farm_id = record.farm_id; // farm_id is i64, not Option<i64>
                let farm = farms.iter_mut().find(|f| f.id == farm_id);

                if let Some(farm) = farm {
                    if let Some(rig_id) = record.rig_id {
                        let rig = farm.rigs.iter_mut().find(|r| r.id == rig_id);
                        if let Some(rig) = rig {
                            if let Some(gpu_id) = record.gpu_id {
                                rig.gpus.push(crate::handlers::rig::Gpu {
                                    id: gpu_id,
                                    name: record.gpu_name.clone().unwrap_or_else(|| "".to_string()),
                                    temp: record.gpu_temp.unwrap_or_default(),
                                    watt: record.gpu_watt.unwrap_or_default(),
                                    rig_id: Some(rig_id),
                                });
                            }
                        } else {
                            farm.rigs.push(Rig {
                                id: rig_id,
                                name: record.rig_name.clone().unwrap_or_else(|| "".to_string()),
                                location: record.rig_location.clone(),
                                farm_id: Some(farm_id),
                                mac_address: String::new(),
                                gpus: if let Some(gpu_id) = record.gpu_id {
                                    vec![crate::handlers::rig::Gpu {
                                        id: gpu_id,
                                        name: record.gpu_name.clone().unwrap_or_else(|| "".to_string()),
                                        temp: record.gpu_temp.unwrap_or_default(),
                                        watt: record.gpu_watt.unwrap_or_default(),
                                        rig_id: Some(rig_id),
                                    }]
                                } else {
                                    Vec::new()
                                },
                            });
                        }
                    }
                } else {
                    let mut new_farm = Farm {
                        id: farm_id,
                        name: record.farm_name.clone(),
                        location: record.farm_location.clone(),
                        rigs: Vec::new(),
                    };

                    if let Some(rig_id) = record.rig_id {
                        new_farm.rigs.push(Rig {
                            id: rig_id,
                            name: record.rig_name.clone().unwrap_or_else(|| "".to_string()),
                            location: record.rig_location.clone(),
                            farm_id: Some(farm_id),
                            mac_address: String::new(),
                            gpus: if let Some(gpu_id) = record.gpu_id {
                                vec![crate::handlers::rig::Gpu {
                                    id: gpu_id,
                                    name: record.gpu_name.clone().unwrap_or_else(|| "".to_string()),
                                    temp: record.gpu_temp.unwrap_or_default(),
                                    watt: record.gpu_watt.unwrap_or_default(),
                                    rig_id: Some(rig_id),
                                }]
                            } else {
                                Vec::new()
                            },
                        });
                    }

                    farms.push(new_farm);
                }
            }

            HttpResponse::Ok().json(farms)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error fetching farms with rigs and GPUs"),
    }
}

pub async fn create_farm(pool: web::Data<SqlitePool>, new_farm: web::Json<NewFarm>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO farm (name, location) VALUES (?, ?)",
        new_farm.name,
        new_farm.location,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Farm created successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to create farm: {}", err)),
    }
}

pub async fn update_farm(
    pool: web::Data<SqlitePool>,
    farm_id: web::Path<i64>,
    updated_farm: web::Json<NewFarm>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE farm SET name = ?, location = ? WHERE id = ?",
        updated_farm.name,
        updated_farm.location,
        *farm_id,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Farm updated successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to update farm: {}", err)),
    }
}

pub async fn delete_farm(
    pool: web::Data<SqlitePool>,
    farm_id: web::Path<i64>,
) -> impl Responder {
    let result = sqlx::query!("DELETE FROM farm WHERE id = ?", *farm_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Farm deleted successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to delete farm: {}", err)),
    }
}