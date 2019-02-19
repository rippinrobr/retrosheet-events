-- Your SQL goes here
ALTER TABLE games
  ALTER COLUMN edittime SET DATA TYPE timestamp;

ALTER TABLE games
  ALTER COLUMN inputtime SET DATA TYPE timestamp;
