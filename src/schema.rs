table! {
    favs (puzzle_id) {
        puzzle_id -> Text,
        fen -> Text,
        moves -> Text,
        rating -> Integer,
        rd -> Integer,
        popularity -> Integer,
        nb_plays -> Integer,
        themes -> Text,
        game_url -> Text,
        opening_tags -> Text,
    }
}
