// main.rs

mod database;
mod model;
mod schema;
mod handlers;

use actix_web::{web, App, HttpServer};
use crate::handlers::{create_human, delete_human, get_humans, update_human};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("This server is up");
    HttpServer::new(|| {
        App::new()
            .route("/human", web::post().to(create_human))
            .route("/humans", web::get().to(get_humans)) // changed this route path
            .route("/human/{id}", web::put().to(update_human))
            .route("/human/{id}", web::delete().to(delete_human))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
