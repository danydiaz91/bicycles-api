#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use std::result::Result;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use uuid::Uuid;

use bicycles_core::models::Bicycle;
use bicycles_core::repositories::Repository;
use models::BicycleDB;
use schema::bicycles::{self, dsl::*};

mod schema;
mod models;

fn establish_connection() -> Result<PgConnection, String> {
    dotenv().ok();
    let connect = |database_url: String| PgConnection::establish(&database_url).map_err(|error| error.to_string());

    env::var("DATABASE_URL")
        .map_err(|error| error.to_string())
        .and_then(connect)
}
pub struct BicycleRepository;

impl BicycleRepository {
    pub fn new() -> Self {
        BicycleRepository{}
    }
}

impl Repository<Bicycle> for BicycleRepository {
    fn get_by_id(&self, bicycle_id: Uuid) -> Result<Bicycle, String> {
        let connection = establish_connection()?;
        bicycles.find(bicycle_id)
            .first::<BicycleDB>(&connection)
            .map(|bicycle_sql| Bicycle::from(bicycle_sql))
            .map_err(|error| error.to_string())
    }

    fn get_all(&self) -> Vec<Bicycle> {
        let bicycles_op = |list: Vec<BicycleDB>| -> Vec<Bicycle> {
                list.into_iter()
                .map(|bicycle_db| Bicycle::from(bicycle_db))
                .collect()
            }; 

        let command = |connection| bicycles.load::<BicycleDB>(&connection)
                .map(bicycles_op)
                .or_else(|_| Ok(Vec::new()));

        let result = establish_connection()
                .and_then(command);

        if let Ok(list_bicycles) = result {
            list_bicycles
        } else {
            Vec::new()
        }
    }

    fn create(&self, bicycle: Bicycle) -> Result<Bicycle, String> {
        let bicycle_db: BicycleDB = bicycle.into();
        let insert_op = |connection| diesel::insert_into(bicycles::table)
                .values(&bicycle_db)
                .get_result::<BicycleDB>(&connection)
                .map(|bicycle_db| Bicycle::from(bicycle_db))
                .map_err(|error| error.to_string());

        establish_connection()
            .map(insert_op)?
    }

    fn update(&self, bicycle: Bicycle) -> Result<Bicycle, String> {
        let bicycle_db: BicycleDB = bicycle.into();
        let update_op = |connection| diesel::update(bicycles.find(bicycle_db.id))
            .set(&bicycle_db)
            .get_result::<BicycleDB>(&connection)
            .map(|bicycle_db| Bicycle::from(bicycle_db))
            .map_err(|error| error.to_string());

        establish_connection()
            .map(update_op)?
    }

    fn delete(&self, bicycle_id: Uuid) -> bool {
        let delete_op = |connection| diesel::delete(bicycles.find(bicycle_id))
                .execute(&connection)
                .map(|rows| rows > 0);

        let result = establish_connection()
                .map(delete_op);

        if let Ok(Ok(deleted)) = result {
            deleted
        } else {
            false
        }
    }
}