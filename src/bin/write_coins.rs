extern crate diesel;
extern crate coins;

use self::coins::*;
use std::io::{stdin};

fn main(){
  
  let connection = establish_connection();

  println!("Please type in a coinname");

  let mut coinname = String::new();

  stdin().read_line(&mut coinname).unwrap();

  let coinname = &coinname[..(coinname.len() - 1)];

  println!("Type in the price for the coin");

  let mut price = String::new();

  stdin().read_line(&mut price).unwrap();

  let price = &price[..(price.len() -1)];

  let _ = create_coin(&connection, coinname, price);

  println!("\n ... Coin {} generated in 5ms with price of {} with id {}", coinname, price, coin.id);
}
