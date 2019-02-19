CREATE TABLE coms (
	    game_id character(12) NOT NULL references games(game_id),
	    idx integer NOT NULL,
	    description character varying(128) NOT NULL,
	    PRIMARY KEY (game_id, idx)
);
