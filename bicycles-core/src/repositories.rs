use uuid::Uuid;
use std::result::Result;

pub trait Repository<T> {
    fn get_by_id(&self, id: Uuid) -> Result<T, String>;
    fn get_all(&self) -> Vec<T>;
    fn create(&self, t: T) -> Result<T, String>;
    fn update(&self, t: T) -> Result<T, String>;
    fn delete(&self, id: Uuid) -> bool;
}