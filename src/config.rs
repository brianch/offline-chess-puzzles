use crate::{styles, search_tab::TaticsThemes, search_tab::Openings, search_tab::OpeningSide};

lazy_static!{
    pub static ref SETTINGS: OfflinePuzzlesConfig = load_config();
}

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
    pub auto_load_next: bool,
    pub board_theme: styles::Theme,
    pub last_min_rating: i32,
    pub last_max_rating: i32,
    pub last_theme: TaticsThemes,
    pub last_opening: Option<Openings>,
    pub last_opening_side: Option<OpeningSide>,
}

impl ::std::default::Default for OfflinePuzzlesConfig {
    fn default() -> Self {
        Self {
            square_size: 60,
            puzzle_db_location: String::from("puzzles/lichess_db_puzzle.csv"),
            piece_theme: styles::PieceTheme::Cburnett,
            search_results_limit: 20000,
            play_sound: true,
            auto_load_next: true,
            board_theme: styles::Theme::default(),
            last_min_rating: 0,
            last_max_rating: 1000,
            last_theme: TaticsThemes::All,
            last_opening: None,
            last_opening_side: None,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    #[serde(rename = "OpeningFamily")]
    #[serde(default)]
    pub opening: String,
    #[serde(rename = "OpeningVariation")]
    #[serde(default)]
    pub variation: String
}
