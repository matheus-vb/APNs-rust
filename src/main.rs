mod api;
mod apnservice;

use std::env;
use dotenv::dotenv;
use actix_web::{ App, HttpServer };
use api::trigger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .service(trigger)
    })
    .bind(address)?
    .run()
    .await
}