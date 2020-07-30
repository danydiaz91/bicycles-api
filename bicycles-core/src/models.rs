use chrono::prelude::{DateTime, Utc};
use uuid::Uuid;
pub struct Bicycle {
    id: Uuid,
    owner_id: Uuid,
    color: Option<Color>,
    created_at: DateTime<Utc> 
}

pub enum Color {
    Red,
    Green,
    Yellow
}