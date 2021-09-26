use iced::Font;
use crate::{styles};

lazy_static!{
    pub static ref SETTINGS: OfflinePuzzlesConfig = load_config();
}

pub const FREE_SERIF: Font = Font::External {
    name: "Free Serif",
    bytes: include_bytes!("../FreeSerif.otf"),
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Puzzle,
    Analysis,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OfflinePuzzlesConfig {
    pub square_size: u16,
    pub puzzle_db_location: String,
    pub piece_theme: styles::PieceTheme,
    pub search_results_limit: usize,
    pub play_sound: bool,
    pub board_theme: styles::BoardStyle,
    pub light_squares_color: [f32; 3],
    pub dark_squares_color: [f32; 3],
}

impl ::std::default::Default for OfflinePuzzlesConfig {
    fn default() -> Self {
        Self {
            square_size: 60,
            puzzle_db_location: String::from("puzzles/lichess_db_puzzle.csv"),
            piece_theme: styles::PieceTheme::Cburnett,
            search_results_limit: 20000,
            play_sound: true,
            board_theme: styles::BoardStyle::Default,
            // Saving the colors as well, so the user may set custom ones by changing
            // the config file directly.
            light_squares_color: styles::BoardStyle::Default.light_sqr(),
            dark_squares_color: styles::BoardStyle::Default.dark_sqr(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub game_url: String
}