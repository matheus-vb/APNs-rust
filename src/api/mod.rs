use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

pub async fn handle() -> impl Responder {
    let hello_message = Message {
        message: "Hello, world!".to_string(),
    };
    HttpResponse::Ok().json(hello_message)
}