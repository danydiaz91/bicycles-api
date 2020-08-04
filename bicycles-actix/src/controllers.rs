use actix_web::{HttpResponse, web};
use uuid::Uuid;

use bicycles_core::repositories::Repository;
use bicycles_diesel::BicycleRepository;

pub struct AppState {
    pub bicycle_repository: BicycleRepository 
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bicycles")
            .route("/{uuid}", web::get().to(get_bicycle_by_id))
    );
}

pub fn get_bicycle_by_id(data: web::Data<AppState>, info: web::Path<Uuid>) -> HttpResponse {
    let result = data.bicycle_repository.get_by_id(info.into_inner());

    match result {
        Ok(bicycle) => HttpResponse::Ok().json(bicycle),
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}