use uuid::Uuid;
use std::result::Result;

pub trait Repository<T> {
    fn get_by_id(id: Uuid) -> Result<T, String>;
    fn get_all() -> Vec<T>;
    fn create(t: T) -> Result<T, String>;
    fn update(t: T) -> Result<T, String>;
    fn delete(id: Uuid) -> bool;
}