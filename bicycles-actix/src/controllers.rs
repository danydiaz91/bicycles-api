use actix_web::{HttpResponse, web};
use uuid::Uuid;

use bicycles_core::models::Bicycle;
use bicycles_core::repositories::Repository;
use bicycles_diesel::BicycleRepository;

pub struct AppState {
    pub bicycle_repository: BicycleRepository 
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bicycles")
            .route("", web::post().to(create_bicycle))
            .route("", web::get().to(get_all_bicycles))
            .route("/{uuid}", web::get().to(get_bicycle_by_id))
            .route("/{uuid}", web::put().to(update_bicycle))
            .route("/{uuid}", web::delete().to(delete_bicycle))
    );
}

pub fn create_bicycle(data: web::Data<AppState>, bicycle_json: web::Json<Bicycle>) -> HttpResponse {
    let result = data.bicycle_repository.create(bicycle_json.into_inner());

    match result {
        Ok(bicycle) => HttpResponse::Ok().json(bicycle),
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}

pub fn update_bicycle(data: web::Data<AppState>, info: web::Path<Uuid>, bicycle_json: web::Json<Bicycle>) -> HttpResponse {
    let mut bicycle = bicycle_json.into_inner();
    bicycle.id = Some(info.into_inner());

    let result = data.bicycle_repository.update(bicycle);

    match result {
        Ok(bicycle) => HttpResponse::Ok().json(bicycle),
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}

pub fn delete_bicycle(data: web::Data<AppState>, info: web::Path<Uuid>) -> HttpResponse {
    data.bicycle_repository.delete(info.into_inner());
    HttpResponse::NoContent().body("")
}

pub fn get_all_bicycles(data: web::Data<AppState>) -> HttpResponse {
    let bicycles = data.bicycle_repository.get_all();
    HttpResponse::Ok().json(bicycles)
}

pub fn get_bicycle_by_id(data: web::Data<AppState>, info: web::Path<Uuid>) -> HttpResponse {
    let result = data.bicycle_repository.get_by_id(info.into_inner());

    match result {
        Ok(bicycle) => HttpResponse::Ok().json(bicycle),
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}