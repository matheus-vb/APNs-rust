use std::sync::Arc;

use a2::Client;
use actix_web::{web::{self, Data}, HttpResponse, Responder, get};
use serde::Serialize;
use crate::apnservice::send_notification;

#[derive(Serialize)]
struct ApiResponse {
    data: String,
}

#[get("/trigger/{device_token}")]
pub async fn trigger(
    client: Data<Arc<Client>>,
    device_token: web::Path<String>
) -> impl Responder {
    match send_notification(client.get_ref(), device_token.as_str()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse { data: "Notification sent.".to_string() }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}