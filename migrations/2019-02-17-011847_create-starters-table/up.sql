-- Your SQL goes here
CREATE TABLE starters (
	game_id character(12) NOT NULL references games(game_id),
	player_id character(8) NOT NULL,
	name character varying(64) NOT NULL,
	team integer NOT NULL,
	batting_order integer NOT NULL,
	position integer NOT NULL,
	PRIMARY KEY (game_id, player_id)
);
