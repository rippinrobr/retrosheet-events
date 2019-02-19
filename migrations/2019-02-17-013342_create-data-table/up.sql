-- Your SQL goes here
CREATE TABLE data (
	    game_id character(12) NOT NULL references games(game_id),
	    player_id character(8) NOT NULL,
	    er integer NOT NULL,
	    PRIMARY KEY (game_id, player_id)
);
