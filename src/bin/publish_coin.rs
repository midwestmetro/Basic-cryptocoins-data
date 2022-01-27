extern crate diesel;
extern crate coins;

use self::diesel::prelude::*;
use self::coins::*;
use std::env::args;

fn main(){

  use self::schema::coins::dsl::{coins, approved};

  let id = args().nth(1).expect("approval needs an id").parse::<i32>().expect("invalid coin id provided");

  let connection = establish_connection();

  let _ = diesel::update(coins.find(id)).set(approved.eq(true)).execute(&connection).expect(&format!("unable to find {}", id));

  let _coin: models::Coin = coins
  .find(id)
  .first(&connection)
  .unwrap_or_else(|_| panic!("Unable to find coin {}", id));

  println!("... {} coin data updated with rust !", _coin.coinname);
}