use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::NewFavorite;
use crate::schema::favs;
use crate::schema::favs::dsl::*;
use crate::config::Puzzle;

use crate::search_tab::{TacticalThemes, OpeningSide};
use crate::openings::{Openings, Variation};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_favorites(min_rating: i32, max_rating: i32, theme: TacticalThemes, opening: Openings, variation: Variation, op_side: Option<OpeningSide>, result_limit: usize) -> Option<Vec<Puzzle>> {
    let mut conn = establish_connection();
    let results;
    let theme_filter = String::from("%") + theme.get_tag_name() + "%";
    let limit = result_limit as i64;
    if opening == Openings::Any {
        results = favs
            .filter(rating.between(min_rating, max_rating))
            .filter(themes.like(theme_filter))
            .limit(limit)
            .load::<Puzzle>(&mut conn);
    } else {
        let opening_tag: &str = if variation.name != Variation::ANY_STR {
            &variation.name
        } else {
            opening.get_field_name()
        };
        let opening_filter = opening_tags.like(String::from("%") + &opening_tag + "%");
        let side = match op_side {
            None => OpeningSide::Any,
            Some(x) => x
        };
        if side == OpeningSide::White {
            results = favs
                .filter(rating.between(min_rating, max_rating))
                .filter(themes.like(theme_filter))
                .filter(opening_filter)
                .filter(game_url.like("%black%"))
                .limit(limit)
                .load::<Puzzle>(&mut conn);
        } else if side == OpeningSide::Black {
            results = favs
                .filter(rating.between(min_rating, max_rating))
                .filter(themes.like(theme_filter))
                .filter(opening_filter)
                .filter(game_url.not_like("%black%"))
                .limit(limit)
                .load::<Puzzle>(&mut conn);
        } else {
            results = favs
                .filter(rating.between(min_rating, max_rating))
                .filter(themes.like(theme_filter))
                .filter(opening_filter)
                .limit(limit)
                .load::<Puzzle>(&mut conn);
        }
    }
    results.ok()
}

pub fn is_favorite(id: &str) -> bool {
    let mut conn = establish_connection();
    let results = favs
        .filter(puzzle_id.eq(id))
        .first::<Puzzle>(&mut conn);
    if results.is_ok() {
        return true;
    }
    return false;
}

pub fn toggle_favorite(puzzle: Puzzle) {
    let mut conn = establish_connection();
    let is_fav = favs
        .filter(puzzle_id.eq(&puzzle.puzzle_id))
        .first::<Puzzle>(&mut conn).is_ok();

    if is_fav {
        diesel::delete(favs::table)
            .filter(puzzle_id.eq(&puzzle.puzzle_id))
            .execute(&mut conn)
            .expect("Error removing favorite");
    } else {
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
}
