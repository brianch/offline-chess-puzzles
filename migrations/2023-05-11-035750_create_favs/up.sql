CREATE TABLE favs (
    puzzle_id TEXT  NOT NULL PRIMARY KEY,
    fen TEXT NOT NULL,
    moves TEXT NOT NULL,
    rating INTEGER NOT NULL,
    rd INTEGER NOT NULL,
    popularity INTEGER NOT NULL,
    nb_plays INTEGER NOT NULL,
    themes TEXT NOT NULL,
    game_url TEXT NOT NULL NOT NULL,
    opening_tags TEXT NOT NULL
)
