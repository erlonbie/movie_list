pub mod models;
pub mod schema;


// #[macro_use]
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::database;


#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub age: u8,
}

// #[macro_use]
// extern crate rocket;

#[macro_use]
extern crate diesel;

#[database("db1")]
pub struct Db1(diesel::PgConnection);

#[database("db2")]
pub struct Db2(diesel::PgConnection);

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}
