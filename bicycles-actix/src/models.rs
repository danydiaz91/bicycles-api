use std::str::FromStr;

use uuid::Uuid;

use bicycles_core::models::*;

pub struct BicycleHttp {
    owner_id: Uuid,
    color: String
}

impl From<BicycleHttp> for Bicycle {
    fn from(bicycle_http: BicycleHttp) -> Self {
        Bicycle {
            id: None,
            owner_id: bicycle_http.owner_id,
            color: Color::from_str(&bicycle_http.color).ok(),
            created_at: None
        }
    }
}