mod api;
mod apnservice;

use std::{env, sync::Arc};
use apnservice::setup_client;
use dotenv::dotenv;
use actix_web::{ App, HttpServer, web::Data };
use api::trigger;
use log::error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    let client = match setup_client() {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to setup client on error: {}", err);
            std::process::exit(1)
        },
    };

    let client = Arc::new(client);

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(client.clone()))
        .service(trigger)
    })
    .bind(address)?
    .run()
    .await
}