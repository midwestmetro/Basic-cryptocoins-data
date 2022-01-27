extern crate diesel;
extern crate coins;

use self::models::post;
use self::coins::*;
use diesel::prelude::*;
use std::env::args

fn main(){

  use diesel::schema::coins::dsl::{coins, approved};

  let id = args().nth(1).expect("approval needs an id").parse::<i32>.expect("invalid coin id provided");

  let connection = establish_connection();

  let coin = diesel.update(coins.find(id)).set(approved.eq(true)).execute(conn).expect(&format!("unable to find {}", id));

  println!("Coin data updated .... name: {}", coin.coinname);
}