#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::NewCoin;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must set");

    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}",database_url))
}

pub fn create_coin(conn: &SqliteConnection, coinname: &str, price: &str) -> usize {
    use schema::coins; 

    let new_coin = NewCoin{coinname, price};

    diesel::insert_into(coins::table).values(&new_coin).execute(conn).expect("Error saving new coin data")
}