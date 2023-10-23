mod api;

use actix_web::{web, App, HttpServer};
use crate::api::handle;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handle))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
