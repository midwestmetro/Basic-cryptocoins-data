#[derive(Queryable)]
struct Coins {
  id i32,
  coinname String,
  price String,
  bearish bool,
  bullish bool
}