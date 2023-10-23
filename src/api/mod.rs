use actix_web::{web, HttpResponse, Responder, get};
use serde::Serialize;
use crate::apnservice::{send_notification, setup_client};

#[derive(Serialize)]
struct ApiResponse {
    data: String,
}

#[get("/trigger/{device_token}")]
pub async fn trigger(device_token: web::Path<String>) -> impl Responder {
    let client = match setup_client() {
        Ok(client) => client,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    };

    match send_notification(&client, device_token.as_str()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse { data: "Notification sent.".to_string() }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}