extern crate coins;
extern crate diesel;

use self::models::*;
use diesel::prelude::*;
use coins::*;

fn main(){
  use self::schema::coins::dsl::*;

  let connection = establish_connection();
  let results = coins.filter(approved.eq(true)).limit(5).load::<Coin>(&connection).expect("Error showing All the crypto currency coins");

  println!("The total coins added are {}", results.len());
  for coin in results {
    println!("{}", coin.coinname);
    println!("---------------\n");
    println!("{}", coin.price);
    println!("{}", coin.bearish);
    println!("{}", coin.bullish);
    println!("{}", coin.approved);
  }
}