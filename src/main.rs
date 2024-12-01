use actix_web::{ web, App, HttpServer };
use db::get_mongo_client;

mod db;
mod models;
mod handlers;
mod routes;

use routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = get_mongo_client().await;

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone())) // Ensure shared state is added
            .configure(configure_routes)
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
