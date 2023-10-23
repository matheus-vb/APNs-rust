use actix_web::{web, HttpResponse, Responder, get};
use serde::Serialize;
use crate::apnservice::send_notification;

#[derive(Serialize)]
struct ApiResponse {
    data: String,
}

#[get("/trigger/{device_token}")]
pub async fn trigger(device_token: web::Path<String>) -> impl Responder {
    match send_notification(device_token.as_str()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse { data: "Notification sent.".to_string() }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}