-- Your SQL goes here
ALTER TABLE games
  ALTER COLUMN edittime SET DATA TYPE varchar(16);

ALTER TABLE games
  ALTER COLUMN inputtime SET DATA TYPE varchar(16);
