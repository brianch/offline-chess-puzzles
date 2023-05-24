use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::NewFavorite;
use crate::schema::favs;
use crate::schema::favs::dsl::*;
use crate::config::Puzzle;

use crate::search_tab::{TaticsThemes, Openings, OpeningSide};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_favorites(min_rating: i32, max_rating: i32, theme: TaticsThemes, opening: Openings, op_side: Option<OpeningSide>, result_limit: usize) -> Option<Vec<Puzzle>> {
    let mut conn = establish_connection();
    let results;
    let theme_filter = String::from("%") + theme.get_tag_name() + "%";
    let limit = result_limit as i64;
    if let Some(side) = op_side {
        if side == OpeningSide::White {
            results = favs
                .filter(rating.between(min_rating, max_rating))
                .filter(themes.like(theme_filter))
                .filter(opening_tags.like(opening.get_field_name()))
                .filter(game_url.like("%black%"))
                .limit(limit)
                .load::<Puzzle>(&mut conn);
        } else if side == OpeningSide::Black {
            results = favs
                .filter(rating.between(min_rating, max_rating))
                .filter(themes.like(theme_filter))
                .filter(opening_tags.like(opening.get_field_name()))
                .filter(game_url.not_like("%black%"))
                .limit(limit)
                .load::<Puzzle>(&mut conn);
        } else {
            results = favs
                .filter(rating.between(min_rating, max_rating))
                .filter(themes.like(theme_filter))
                .filter(opening_tags.like(String::from("%") + opening.get_field_name() + "%"))
                .limit(limit)
                .load::<Puzzle>(&mut conn);
        }
    } else {
        results = favs
            .filter(rating.between(min_rating, max_rating))
            .filter(themes.like(theme.get_tag_name()))
            .filter(opening_tags.like(opening.get_field_name()))
            .limit(limit)
            .load::<Puzzle>(&mut conn);
    }
    results.ok()
}

pub fn is_favorite(id: String) -> bool {
    let mut conn = establish_connection();
    let results = favs
        .filter(puzzle_id.eq(id))
        .limit(1)
        .execute(&mut conn);
    if let Ok(result) = results {
        if result == 1 {
            return true;
        }
    }
    return false;
}

pub fn add_favorite(puzzle: Puzzle) {
    let mut conn = establish_connection();
    let new_fav = NewFavorite {
        puzzle_id: &puzzle.puzzle_id,
        fen: &puzzle.fen,
        moves: &puzzle.moves,
        rating: puzzle.rating,
        rd: puzzle.rating_deviation,
        popularity: puzzle.popularity,
        nb_plays: puzzle.nb_plays,
        themes: &puzzle.themes,
        game_url: &puzzle.game_url,
        opening_tags: &puzzle.opening,
    };

    diesel::insert_into(favs::table)
        .values(&new_fav)
        .execute(&mut conn)
        .expect("Error saving new favorite");
}
