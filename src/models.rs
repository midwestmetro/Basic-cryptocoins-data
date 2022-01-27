#[derive(Queryable, Debug)]
pub struct Coin {
  pub id: i32,
  pub coinname: String,
  pub price: String,
  pub bearish: bool,
  pub bullish: bool,
  pub approved: bool
}