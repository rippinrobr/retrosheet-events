CREATE TABLE plays (
        game_id character(12) NOT NULL references games(game_id),
        idx integer NOT NULL,
        inning integer NOT NULL,
        team integer NOT NULL,
        player_id character(8) NOT NULL,
        count character varying(16) NOT NULL,
        pitches character varying(32) NOT NULL,
        event character varying(32) NOT NULL,
        PRIMARY KEY (game_id, idx)
      );
