#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv
use std::env;

pub mod schema;
pub mod models;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must set");

    SqliteConnection.establish(&connection).expect(&format!("Error connecting to {}",database_url ));
}