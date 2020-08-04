use actix_rt;
use actix_web::{App, HttpServer};

use bicycles_diesel::BicycleRepository;
use controllers::AppState;

mod controllers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                bicycle_repository: BicycleRepository::new()
            })
            .configure(controllers::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}