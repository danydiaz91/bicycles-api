use chrono::prelude::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use strum_macros::{Display, EnumString};
#[derive(Serialize, Deserialize)]
pub struct Bicycle {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub color: Option<Color>,
    pub created_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, Display, EnumString)]
pub enum Color {
    Red,
    Green,
    Yellow
}