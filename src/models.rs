use super::schema::coins;

#[derive(Insertable)]
#[table_name="coins"]
pub struct NewCoin <'a>{
  pub coinname: &'a str,
  pub price: &'a str,
}

#[derive(Queryable, Debug)]
pub struct Coin {
  pub id: i32,
  pub coinname: String,
  pub price: String,
  pub bearish: bool,
  pub bullish: bool,
  pub approved: bool
}