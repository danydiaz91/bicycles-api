use chrono::prelude::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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