use diesel::prelude::*;
use crate::schema::favs;

/*
#[derive(Queryable)]
pub struct Favorite {
    pub puzzle_id: String,
    pub fen: String,
    pub moves: String,
    pub rating: i32,
    pub rd: i32,
    pub popularity: i32,
    pub nb_plays: i32,
    pub themes: String,
    pub game_url: String,
    pub opening_tags: String,
}
*/
#[derive(Insertable)]
#[diesel(table_name = favs)]
pub struct NewFavorite<'a> {
    pub puzzle_id: &'a str,
    pub fen: &'a str,
    pub moves: &'a str,
    pub rating: i32,
    pub rd: i32,
    pub popularity: i32,
    pub nb_plays: i32,
    pub themes: &'a str,
    pub game_url: &'a str,
    pub opening_tags: &'a str,
}
