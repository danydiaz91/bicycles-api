use std::convert::From;
use std::str::FromStr;

use chrono::prelude::NaiveDateTime;
use uuid::Uuid;

use bicycles_core::models::*;

use crate::schema::bicycles;

#[derive(Queryable, Insertable, AsChangeset)]
#[table_name = "bicycles"]
pub struct BicycleDB {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub color: Option<String>,
    pub created_at: NaiveDateTime
}

impl From<Bicycle> for BicycleDB {
    fn from(bicycle: Bicycle) -> Self {
        BicycleDB {
            id: bicycle.id,
            owner_id: bicycle.owner_id,
            color: bicycle.color.map(|color| color.to_string()),
            created_at: bicycle.created_at
        }
    }
}

impl From<BicycleDB> for Bicycle {
    fn from(bicycle_sql: BicycleDB) -> Self {
        Bicycle {
            id: bicycle_sql.id,
            owner_id: bicycle_sql.owner_id,
            color: bicycle_sql.color.map(|color| Color::from_str(&color).unwrap()), 
            created_at: bicycle_sql.created_at
        }
    }
}