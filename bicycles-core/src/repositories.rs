use uuid::Uuid;
use std::result::Result;
use crate::models::Bicycle;

pub trait BicycleRepository {
    fn get_by_id(id: Uuid) -> Result<Bicycle, String>;
    fn get_all() -> Vec<Bicycle>;
    fn create(bicycle: Bicycle) -> Result<Bicycle, String>;
    fn update(bicycle: Bicycle) -> Result<Bicycle, String>;
    fn delete(id: Uuid) -> bool;
}