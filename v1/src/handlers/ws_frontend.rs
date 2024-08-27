use actix::{Actor, StreamHandler, Addr, AsyncContext};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};
use serde_json::{json, Value};
use sqlx::SqlitePool;  // Import SqlitePool
use crate::handlers::handler_frontend::{create_farm, edit_farm, delete_farm, show_rigs, Farm};

// Define the Frontend WebSocket
pub struct FrontendWebSocket {
    pub clients: Arc<Mutex<Vec<Addr<FrontendWebSocket>>>>,
    pub pool: web::Data<SqlitePool>, // Add the pool as a member
}

impl Actor for FrontendWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.clients.lock().unwrap().push(addr);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.clients.lock().unwrap().retain(|client| client != &addr);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for FrontendWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            println!("Received from frontend: {}", text);
            let message: Value = serde_json::from_str(&text).unwrap();
            let pool_clone = self.pool.clone(); // Clone the pool for async use

            // Spawn an async task to handle the request
            let ctx_addr = ctx.address();
            actix_rt::spawn(async move {
                let response = match message["type"].as_str() {
                    Some("CREATE_FARM") => {
                        if let Ok(farm) = serde_json::from_value::<Farm>(message.clone()) {
                            let farm_data = create_farm(pool_clone.clone(), web::Json(farm)).await;
                            json!({ "type": "CREATE_FARM_RESPONSE", "data": farm_data })
                        } else {
                            json!({ "type": "ERROR", "message": "Invalid data for CREATE_FARM" })
                        }
                    },
                    Some("EDIT_FARM") => {
                        let id = message["id"].as_i64().unwrap_or_default(); // Get id as i64
                        if let Ok(farm) = serde_json::from_value::<Farm>(message.clone()) {
                            let farm_data = edit_farm(pool_clone.clone(), web::Path::from(id), web::Json(farm)).await;
                            json!({ "type": "EDIT_FARM_RESPONSE", "data": farm_data })
                        } else {
                            json!({ "type": "ERROR", "message": "Invalid data for EDIT_FARM" })
                        }
                    },
                    Some("DELETE_FARM") => {
                        let id = message["id"].as_i64().unwrap_or_default(); // Get id as i64
                        let delete_message = delete_farm(pool_clone.clone(), web::Path::from(id)).await;
                        json!({ "type": "DELETE_FARM_RESPONSE", "message": delete_message })
                    },
                    Some("SHOW_RIGS") => {
                        if let Some(id) = message["id"].as_i64() {
                            let rigs_data = show_rigs(pool_clone.clone(), web::Path::from(Some(id))).await;
                            json!({ "type": "SHOW_RIGS_RESPONSE", "data": rigs_data })
                        } else {
                            json!({ "type": "ERROR", "message": "Invalid farm ID" })
                        }
                    },,
                    _ => json!({ "type": "ERROR", "message": "Unknown command" }),
                };
            
                ctx_addr.do_send(SendMessage(response.to_string()));
            });
        }
    }
}

// Define a message type for sending responses back to the WebSocket client
struct SendMessage(String);

impl actix::Message for SendMessage {
    type Result = ();
}

impl actix::Handler<SendMessage> for FrontendWebSocket {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0); // Send the message back to the WebSocket client
    }
}

// WebSocket connection handler for the frontend
pub async fn ws_frontend(
    req: HttpRequest,
    stream: web::Payload,
    clients: web::Data<Arc<Mutex<Vec<Addr<FrontendWebSocket>>>>>,
    pool: web::Data<SqlitePool>, // Receive the pool here
) -> Result<HttpResponse, Error> {
    ws::start(
        FrontendWebSocket {
            clients: clients.get_ref().clone(),
            pool: pool.clone(), // Pass the pool into the WebSocket struct
        },
        &req,
        stream,
    )
}