-- Your SQL goes here
CREATE TABLE coins(
  id INTEGER PRIMARY KEY NOT NULL,
  coinname VARCHAR NOT NULL,
  price VARCHAR NOT NULL,
  bearish BOOLEAN NOT NULL DEFAULT 'f',
  bullish BOOLEAN NOT NULL DEFAULT 'f'
)