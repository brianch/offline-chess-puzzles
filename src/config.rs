use crate::{styles, search_tab::TaticsThemes, search_tab::OpeningSide, lang, openings::Openings};
use once_cell::sync::Lazy;
use chess::{Board, ChessMove, Piece, Square};
use std::str::FromStr;
use iced::Font;

use diesel::prelude::*;

pub static SETTINGS: Lazy<OfflinePuzzlesConfig> = Lazy::new(|| {
    load_config()
});

pub const CHESS_ALPHA_BYTES: &[u8] = include_bytes!("../font/Alpha.ttf");
pub const CHESS_ALPHA: Font = iced::Font::with_name("Chess Alpha");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Puzzle,
    Analysis,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OfflinePuzzlesConfig {
    pub engine_path: Option<String>,
    pub engine_limit: String,
    pub window_width: u32,
    pub window_height: u32,
    pub puzzle_db_location: String,
    pub piece_theme: styles::PieceTheme,
    pub search_results_limit: usize,
    pub play_sound: bool,
    pub auto_load_next: bool,
    pub flip_board: bool,
    pub show_coordinates: bool,
    pub board_theme: styles::Theme,
    pub lang: lang::Language,
    pub export_pgs: i32,
    pub last_min_rating: i32,
    pub last_max_rating: i32,
    pub last_theme: TaticsThemes,
    pub last_opening: Openings,
    pub last_opening_side: Option<OpeningSide>,
}

impl ::std::default::Default for OfflinePuzzlesConfig {
    fn default() -> Self {
        Self {
            engine_path: None,
            engine_limit: String::from("depth 40"),
            window_width: 1010,
            window_height: 680,
            puzzle_db_location: String::from("puzzles/lichess_db_puzzle.csv"),
            piece_theme: styles::PieceTheme::Cburnett,
            search_results_limit: 20000,
            play_sound: true,
            auto_load_next: true,
            flip_board: false,
            show_coordinates: false,
            board_theme: styles::Theme::default(),
            lang: lang::Language::English,
            export_pgs: 50,
            last_min_rating: 0,
            last_max_rating: 1000,
            last_theme: TaticsThemes::All,
            last_opening: Openings::Any,
            last_opening_side: Some(OpeningSide::Any),
        }
    }
}

pub fn load_config() -> OfflinePuzzlesConfig {
    let config;
    let file = std::fs::File::open("settings.json");
    match file {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            let config_json = serde_json::from_reader(reader);
            match config_json {
                Ok(cfg) => config = cfg,
                Err(_) => config = OfflinePuzzlesConfig::default()
            }
        } Err(_) => config = OfflinePuzzlesConfig::default()
    }
    config
}

pub fn coord_to_san(board: &Board, coords: String) -> Option<String> {
    let coords = if coords.len() > 4 {
        String::from(&coords[0..4]) + "=" + &coords[4..5].to_uppercase()
    } else {
        coords
    };
    let mut san = None;
    let orig_square = Square::from_str(&coords[0..2]).unwrap();
    let dest_square = Square::from_str(&coords[2..4]).unwrap();
    let piece = board.piece_on(orig_square);
    if let Some(piece) = piece {
        if piece == Piece::King && (coords == "e1g1" || coords == "e8g8") {
            san = Some(String::from("0-0"));
        } else if piece == Piece::King && (coords == "e1c1" || coords == "e8c8") {
            san = Some(String::from("0-0-0"));
        } else {
            let mut san_str = String::new();
            let is_en_passant = piece == Piece::Pawn &&
                board.piece_on(dest_square).is_none() &&
                dest_square.get_file() != orig_square.get_file();
            let is_normal_capture = board.piece_on(dest_square).is_some();
            match piece {
                Piece::Pawn => san_str.push_str(&coords[0..1]),
                Piece::Bishop => san_str.push_str("B"),
                Piece::Knight => san_str.push_str("N"),
                Piece::Rook => san_str.push_str("R"),
                Piece::Queen => san_str.push_str("Q"),
                Piece::King => san_str.push_str("K"),
            }
            if is_en_passant {
                san_str.push_str(&"x");
                san_str.push_str(&coords[2..4]);
                san_str.push_str(" e.p.");
            } else if is_normal_capture {
                let simple_capture = san_str.clone() + &"x" + &coords[2..];
                let try_move = ChessMove::from_san(&board, &simple_capture);
                if let Ok(_) = try_move {
                    san_str.push_str(&"x");
                    san_str.push_str(&coords[2..]);
                } else {
                    //the simple notation can only fail because of ambiguity, so we try to specify
                    //either the file or the rank
                    let capture_with_file = san_str.clone() + &coords[0..1] + &"x" + &coords[2..];
                    let try_move_file = ChessMove::from_san(&board, &capture_with_file);
                    if let Ok(_) = try_move_file {
                        san_str.push_str(&coords[0..1]);
                        san_str.push_str(&"x");
                        san_str.push_str(&coords[2..]);
                    } else {
                        san_str.push_str(&coords[1..2]);
                        san_str.push_str(&"x");
                        san_str.push_str(&coords[2..]);
                    }
                }
            } else {
                if piece==Piece::Pawn {
                    san_str = String::from(&coords[2..]);
                } else {
                    san_str.push_str(&coords[2..]);
                }
            }
            san = Some(san_str);
        }
    }
    san
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Queryable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Puzzle {
    #[serde(rename = "PuzzleId")]
    pub puzzle_id: String,
    #[serde(rename = "FEN")]
    pub fen: String,
    #[serde(rename = "Moves")]
    pub moves: String,
    #[serde(rename = "Rating")]
    pub rating: i32,
    #[serde(rename = "RatingDeviation")]
    pub rating_deviation: i32,
    #[serde(rename = "Popularity")]
    pub popularity: i32,
    #[serde(rename = "NbPlays")]
    pub nb_plays: i32,
    #[serde(rename = "Themes")]
    pub themes: String,
    #[serde(rename = "GameUrl")]
    pub game_url: String,
    #[serde(rename = "OpeningTags")]
    #[serde(default)]
    pub opening: String,
}
