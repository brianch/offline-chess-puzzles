#![windows_subsystem = "windows"]

use iced::{button, container, slider, pick_list, Container, Align, Length, HorizontalAlignment, VerticalAlignment, Background, Button, Slider, PickList, Row, Column, Element, Sandbox, Settings, Text, Image};

use chess::{Board, BoardStatus, ChessMove, Color, Piece, Rank, Square, File};
use std::str::FromStr;

use rand::thread_rng;
use rand::seq::SliceRandom;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

lazy_static!{
    static ref SETTINGS: OfflinePuzzlesConfig = load_config();
}

#[derive(Serialize, Deserialize, Clone)]
struct OfflinePuzzlesConfig {
    square_size: u16,
    puzzle_db_location: String,
    piece_theme: String,
    search_results_limit: usize,
}

impl ::std::default::Default for OfflinePuzzlesConfig {
    fn default() -> Self {
        Self {
            square_size: 60,
            puzzle_db_location: String::from("puzzles/lichess_db_puzzle.csv"),
            piece_theme: String::from("merida"),
            search_results_limit: 20000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositionGUI {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    #[serde(rename = "PuzzleId")]
    puzzle_id: String,
    #[serde(rename = "FEN")]
    fen: String,
    #[serde(rename = "Moves")]
    moves: String,
    #[serde(rename = "Rating")]
    rating: i32,
    #[serde(rename = "RatingDeviation")]
    rating_deviation: i32,
    #[serde(rename = "Popularity")]
    popularity: i32,
    #[serde(rename = "NbPlays")]
    nb_plays: i32,
    #[serde(rename = "Themes")]
    themes: String,
    #[serde(rename = "GameUrl")]
    game_url: String
}

impl PositionGUI {
    
    #[inline]
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    /// Get the row number of the position.
    /// This can be any of 0, 1, 2, 3, 4, 5, 6, or 7.
    #[inline]
    pub fn get_row(&self) -> i32 {
        self.row
    }

    #[inline]
    pub fn get_col(&self) -> i32 {
        self.col
    }

    pub fn posgui_to_notation(&self) -> String {
        let file = match self.col {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            _ => "h",
        };
        let rank = match self.row {
            0 => "1",
            1 => "2",
            2 => "3",
            3 => "4",
            4 => "5",
            5 => "6",
            6 => "7",
            _ => "8",
        };
        file.to_owned() + rank
    }

    pub fn posgui_to_square(&self) -> Square {
        let file = match self.col {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            _ => File::H,            
        };
        let rank = match self.row {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            _ => Rank::Eighth,
        };
        Square::make_square(rank, file)
    }

    pub fn chesssquare_to_posgui(square: Square) -> PositionGUI {
        let col = match square.get_file() {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        };
        let row = match square.get_rank() {
            Rank::First => 0,
            Rank::Second => 1,
            Rank::Third => 2,
            Rank::Fourth => 3,
            Rank::Fifth => 4,
            Rank::Sixth => 5,
            Rank::Seventh => 6,
            Rank::Eighth => 7,
        };
        PositionGUI::new(row,col)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectSquare(PositionGUI),
    SliderMinRatingChanged(i32),
    SliderMaxRatingChanged(i32),
    SelectTheme(TaticsThemes),
    ClickSearch,
    SelectPiecePromotion(Piece)
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        iced::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    }
}

const SELECTED_DARK_SQUARE: iced::Color = rgb!(170,162,58);
const SELECTED_LIGHT_SQUARE: iced::Color = rgb!(205,210,106);

const LIGHT_SQUARE: iced::Color = rgb!(240,217,181);
const DARK_SQUARE: iced::Color = rgb!(181,136,99);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaticsThemes {
    All,
    Opening, Middlegame, Endgame, RookEndgame, BishopEndgame, PawnEndgame, KnightEndgame, QueenEndgame, QueenRookEndgame,
    AdvancedPawn, AtackingF2F7, CapturingDefender, DiscoveredAttack, DoubleCheck, ExposedKing, Fork, HangingPiece, KingsideAttack, Pin, QueensideAttack, Sacrifice, Skewer, TrappedPiece,
    Attraction, Clearance, DefensiveMove, Deflection, Interference, Intermezzo, QuietMove, XRayAttack, Zugzwang,
    Mate, MateIn1, MateIn2, MateIn3, MateIn4, MateIn5, AnastasiaMate, ArabianMate, BackRankMate, BodenMate, DoubleBishopMate, DovetailMate, HookMate, SmotheredMate,
    Castling, EnPassant, Promotion, UnderPromotion, Equality, Advantage, Crushing,
    OneMove, Short, Long, VeryLong,
    Master, MasterVsMaster, SuperGM
}

impl TaticsThemes {
    const ALL: [TaticsThemes; 61] = [
        TaticsThemes::All,
        TaticsThemes::Opening, TaticsThemes::Middlegame, TaticsThemes::Endgame, TaticsThemes::RookEndgame,
        TaticsThemes::BishopEndgame, TaticsThemes::PawnEndgame, TaticsThemes::KnightEndgame,
        TaticsThemes::QueenEndgame, TaticsThemes::QueenRookEndgame,

        TaticsThemes::AdvancedPawn, TaticsThemes::AtackingF2F7, TaticsThemes::CapturingDefender,
        TaticsThemes::DiscoveredAttack, TaticsThemes::DoubleCheck, TaticsThemes::ExposedKing,
        TaticsThemes::Fork, TaticsThemes::HangingPiece, TaticsThemes::KingsideAttack, TaticsThemes::Pin,
        TaticsThemes::QueensideAttack, TaticsThemes::Sacrifice, TaticsThemes::Skewer,
        TaticsThemes::TrappedPiece,

        TaticsThemes::Attraction, TaticsThemes::Clearance, TaticsThemes::DefensiveMove,
        TaticsThemes::Deflection, TaticsThemes::Interference, TaticsThemes::Intermezzo,
        TaticsThemes::QuietMove, TaticsThemes::XRayAttack, TaticsThemes::Zugzwang,

        TaticsThemes::Mate, TaticsThemes::MateIn1, TaticsThemes::MateIn2, TaticsThemes::MateIn3,
        TaticsThemes::MateIn4, TaticsThemes::MateIn5, TaticsThemes::AnastasiaMate, TaticsThemes::ArabianMate,
        TaticsThemes::BackRankMate, TaticsThemes::BodenMate, TaticsThemes::DoubleBishopMate,
        TaticsThemes::DovetailMate, TaticsThemes::HookMate, TaticsThemes::SmotheredMate,

        TaticsThemes::Castling, TaticsThemes::EnPassant, TaticsThemes::Promotion,
        TaticsThemes::UnderPromotion, TaticsThemes::Equality, TaticsThemes::Advantage,
        TaticsThemes::Crushing,

        TaticsThemes::OneMove, TaticsThemes::Short, TaticsThemes::Long, TaticsThemes::VeryLong,

        TaticsThemes::Master, TaticsThemes::MasterVsMaster, TaticsThemes::SuperGM
    ];
}

impl Default for TaticsThemes {
    fn default() -> TaticsThemes {
        TaticsThemes::Mate
    }
}

impl std::fmt::Display for TaticsThemes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TaticsThemes::All => "all",
                TaticsThemes::Opening => "opening",
                TaticsThemes::Middlegame=> "middlegame",
                TaticsThemes::Endgame => "endgame",
                TaticsThemes::RookEndgame => "rookEndgame",
                TaticsThemes::BishopEndgame => "bishopEndgame",
                TaticsThemes::PawnEndgame => "pawnEndgame",
                TaticsThemes::KnightEndgame => "knightEndgame",
                TaticsThemes::QueenEndgame => "queenEndgame",
                TaticsThemes::QueenRookEndgame => "queenRookEndgame",
        
                TaticsThemes::AdvancedPawn => "advancedPawn",
                TaticsThemes::AtackingF2F7 => "attackingF2F7",
                TaticsThemes::CapturingDefender => "capturingDefender",
                TaticsThemes::DiscoveredAttack => "discoveredAttack",
                TaticsThemes::DoubleCheck => "doubleCheck",
                TaticsThemes::ExposedKing => "exposedKing",
                TaticsThemes::Fork => "fork",
                TaticsThemes::HangingPiece => "hangingPiece",
                TaticsThemes::KingsideAttack => "kingsideAttack",
                TaticsThemes::Pin => "pin",
                TaticsThemes::QueensideAttack => "queensideAttack",
                TaticsThemes::Sacrifice => "sacrifice",
                TaticsThemes::Skewer => "skewer",
                TaticsThemes::TrappedPiece => "trappedPiece",

                TaticsThemes::Attraction => "attraction",
                TaticsThemes::Clearance => "clearance",
                TaticsThemes::DefensiveMove => "defensiveMove",
                TaticsThemes::Deflection => "deflection",
                TaticsThemes::Interference => "interference",
                TaticsThemes::Intermezzo => "intermezzo",
                TaticsThemes::QuietMove => "quietMove",
                TaticsThemes::XRayAttack => "xRayAttack",
                TaticsThemes::Zugzwang => "zugzwang",
        
                TaticsThemes::Mate => "mate",
                TaticsThemes::MateIn1 => "mateIn1",
                TaticsThemes::MateIn2 => "mateIn2",
                TaticsThemes::MateIn3 => "mateIn2",
                TaticsThemes::MateIn4 => "mateIn4",
                TaticsThemes::MateIn5 => "mateIn5",
                TaticsThemes::AnastasiaMate => "anastasiaMate",
                TaticsThemes::ArabianMate => "arabianMate",
                TaticsThemes::BackRankMate => "backRankMate",
                TaticsThemes::BodenMate => "bodenMate",
                TaticsThemes::DoubleBishopMate => "doubleBishopMate",
                TaticsThemes::DovetailMate => "dovetailMate",
                TaticsThemes::HookMate => "hookMate",
                TaticsThemes::SmotheredMate => "smotheredMate",

                TaticsThemes::Castling => "castling",
                TaticsThemes::EnPassant => "enPassant",
                TaticsThemes::Promotion => "promotion",
                TaticsThemes::UnderPromotion => "underPromotion",
                TaticsThemes::Equality => "equality",
                TaticsThemes::Advantage => "advantage",
                TaticsThemes::Crushing => "crushing",

                TaticsThemes::OneMove => "oneMove",
                TaticsThemes::Short => "short",
                TaticsThemes::Long => "long",
                TaticsThemes::VeryLong => "veryLong",

                TaticsThemes::Master => "master",
                TaticsThemes::MasterVsMaster => "masterVsMaster",
                TaticsThemes::SuperGM => "superGM",

            }
        )
    }
}

struct ChessSquare { row: i32, col: i32, is_selected: bool }

impl From<(PositionGUI, bool)> for ChessSquare {
    fn from(pos_color: (PositionGUI, bool)) -> Self {
        let (pos, is_selected) = pos_color;
        Self::new(pos.get_row(), pos.get_col(), is_selected)
    }
}

impl ChessSquare {
    fn new(row: i32, col: i32, is_selected: bool) -> Self {
        Self { row, col, is_selected }
    }

    fn get_bg_color(&self, is_selected: bool) -> iced::Color {
        if (self.row * 9 + self.col) % 2 == 1 {
            if is_selected {
                SELECTED_LIGHT_SQUARE
            } else {
                LIGHT_SQUARE
            }
        } else {
            if is_selected {
                SELECTED_DARK_SQUARE
            } else {
                DARK_SQUARE
            }
        }
    }
}

impl button::StyleSheet for ChessSquare {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.get_bg_color(self.is_selected))),
            border_color: self.get_bg_color(self.is_selected),
            border_radius: 0.0,
            border_width: 0.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.get_bg_color(true))),
            border_color: self.get_bg_color(true),
            border_radius: 0.0,
            border_width: 0.0,
            ..button::Style::default()
        }
    }
}

struct ChessBoardStyle;

impl container::StyleSheet for ChessBoardStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_color: iced::Color::BLACK,
            //border_width: 10.0,
            //border_radius: 0.0,
            ..container::Style::default()
        }
    }
}

struct PromotionStyle;

impl button::StyleSheet for PromotionStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(LIGHT_SQUARE)),
            border_radius: 0.1,
            border_width: 0.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(SELECTED_LIGHT_SQUARE)),
            border_radius: 1.0,
            border_width: 0.0,
            ..button::Style::default()
        }
    }
}


struct SearchBoxStyle;

impl container::StyleSheet for SearchBoxStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_color: iced::Color::BLACK,
            border_width: 2.0,
            border_radius: 0.0,
            ..container::Style::default()
        }
    }
}

#[derive(Clone)]
pub struct OfflinePuzzles {
    from_square: Option<PositionGUI>,
    board: Board,
    squares: [button::State; 64],
    last_move_from: Option<PositionGUI>,
    last_move_to: Option<PositionGUI>,
    is_playing: bool,

    theme_list: pick_list::State<TaticsThemes>,
    theme: TaticsThemes,

    slider_min_rating_state: slider::State,
    slider_min_rating_value: i32,

    slider_max_rating_state: slider::State,
    slider_max_rating_value: i32,    

    btn_search_state: button::State,

    btns_promotion: [button::State; 4],
    piece_to_promote_to: Piece,

    puzzles: Vec<Puzzle>,

    current_puzzle: usize,
    current_puzzle_move: usize,
    puzzle_status: String
}

impl Default for OfflinePuzzles {
    fn default() -> Self {
        Self {
            from_square: None,
            board: Board::default(),
            squares: [button::State::default(); 64],
            last_move_from: None,
            last_move_to: None,
            is_playing: false,

            theme_list: pick_list::State::default(),
            theme : TaticsThemes::default(),

            slider_min_rating_state: slider::State::new(),
            slider_min_rating_value: 0,

            slider_max_rating_state: slider::State::new(),
            slider_max_rating_value: 1000,

            btn_search_state: button::State::new(),

            btns_promotion: [button::State::default(); 4],
            piece_to_promote_to: Piece::Queen,

            puzzles: Vec::new(),
            current_puzzle: 0,
            current_puzzle_move: 1,
            puzzle_status: String::from("Use the search below.")
        }
    }
}

// Checks if the notation indicates a promotion and return the piece
// if that's the case.
fn check_promotion(notation: &str) -> Option<Piece> {
    let mut promotion = None;
    if notation.len() > 4 {
        promotion = match &notation[4..5] {
            "r" => Some(Piece::Rook),
            "n" => Some(Piece::Knight),
            "b" => Some(Piece::Bishop),
            _ => Some(Piece::Queen),
        }
    }
    promotion
}

fn load_config() -> OfflinePuzzlesConfig {
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

impl Sandbox for OfflinePuzzles {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Offline Chess Puzzles")
    }

    fn update(&mut self, message: Message) {
        match (self.from_square, message) {
            (None, Message::SelectSquare(pos)) => {
                if self.is_playing && self.board.color_on(pos.posgui_to_square()) == Some(self.board.side_to_move()) {
                    self.from_square = Some(pos);
                }
            } (Some(from), Message::SelectSquare(to)) if from != to => {
                self.from_square = None;
                
                if self.puzzles.len() > 0 {
                    let movement;
                    let mut move_made_notation: String = from.posgui_to_notation() + &to.posgui_to_notation();

                    // Check for promotion and adjust the notation accordingly
                    let piece = self.board.piece_on(from.posgui_to_square());
                    let color = self.board.color_on(from.posgui_to_square());
                    if piece.is_some() && color.is_some() && piece.unwrap() == Piece::Pawn {
                        if (color.unwrap() == Color::White && to.get_row() == 7) ||
                                (color.unwrap() == Color::Black && to.get_row() == 0) {
                            match self.piece_to_promote_to {
                                Piece::Rook => move_made_notation += "r",
                                Piece::Knight => move_made_notation += "n",
                                Piece::Bishop => move_made_notation += "b",
                                _ => move_made_notation += "q"
                            }
                        }
                    } 
                    let move_made = ChessMove::new(
                        Square::from_str(&String::from(&move_made_notation[..2])).unwrap(),
                        Square::from_str(&String::from(&move_made_notation[2..4])).unwrap(), check_promotion(&move_made_notation));

                    let is_mate = self.board.legal(move_made) && self.board.make_move_new(move_made).status() == BoardStatus::Checkmate;

                    let correct_moves : Vec<&str> = self.puzzles[self.current_puzzle].moves.split_whitespace().collect::<Vec<&str>>();
                    let correct_move = ChessMove::new(
                        Square::from_str(&String::from(&correct_moves[self.current_puzzle_move][..2])).unwrap(),
                        Square::from_str(&String::from(&correct_moves[self.current_puzzle_move][2..4])).unwrap(), check_promotion(&correct_moves[self.current_puzzle_move]));

                    // If the move is correct we can apply it to the board
                    if is_mate || (move_made == correct_move) {
                       
                        self.board = self.board.make_move_new(move_made);
                        self.current_puzzle_move += 1;

                        if self.current_puzzle_move == correct_moves.len() {
                            if self.current_puzzle < self.puzzles.len() - 1 {
                                // The previous puzzle ended, and we still have puzzles available,
                                // so we prepare the next one.
                                self.current_puzzle += 1;
                                self.current_puzzle_move = 1;

                                let puzzle_moves: Vec<&str> = self.puzzles[self.current_puzzle].moves.split_whitespace().collect();

                                // The opponent's last move (before the puzzle starts)
                                // is in the "moves" field of the cvs, so we need to apply it.
                                self.board = Board::from_str(&self.puzzles[self.current_puzzle].fen).unwrap();

                                movement = ChessMove::new(
                                    Square::from_str(&String::from(&puzzle_moves[0][..2])).unwrap(),
                                    Square::from_str(&String::from(&puzzle_moves[0][2..4])).unwrap(), check_promotion(&puzzle_moves[0]));
            
                                self.last_move_from = Some(PositionGUI::chesssquare_to_posgui(movement.get_source()));
                                self.last_move_to = Some(PositionGUI::chesssquare_to_posgui(movement.get_dest()));
        
                                self.board = self.board.make_move_new(movement);

                                if self.board.side_to_move() == Color::White {
                                    self.puzzle_status = String::from("White to move!");
                                } else {
                                    self.puzzle_status = String::from("Black to move!");
                                }
                            } else {
                                self.board = Board::default();
                                self.last_move_from = None;
                                self.last_move_to = None;
                                self.is_playing = false;
                                self.puzzle_status = String::from("All puzzles done for this search!");
                            }
                        } else {
                            movement = ChessMove::new(
                                Square::from_str(&String::from(&correct_moves[self.current_puzzle_move][..2])).unwrap(),
                                Square::from_str(&String::from(&correct_moves[self.current_puzzle_move][2..4])).unwrap(), check_promotion(&correct_moves[self.current_puzzle_move]));

                            self.last_move_from = Some(PositionGUI::chesssquare_to_posgui(movement.get_source()));
                            self.last_move_to = Some(PositionGUI::chesssquare_to_posgui(movement.get_dest()));
    
                            self.board = self.board.make_move_new(movement);
                            self.current_puzzle_move += 1;
                            self.puzzle_status = String::from("Correct! What now?");
                        }
                    } else {
                        if self.board.side_to_move() == Color::White {
                            self.puzzle_status = String::from("Ops! Wrong move... White to play.");
                        } else {
                            self.puzzle_status = String::from("Ops! Wrong move... Black to play.");
                        }
                    }
                }
            } (Some(_), Message::SelectSquare(to)) => {
                self.from_square = Some(to);
            } (_, Message::SliderMinRatingChanged(new_value)) => {
                self.slider_min_rating_value = new_value;
            } (_, Message::SliderMaxRatingChanged(new_value)) => {
                self.slider_max_rating_value = new_value;
            } (_, Message::SelectTheme(new_theme)) => {
                self.theme = new_theme;
            } (_, Message::ClickSearch) => {
                let reader = csv::ReaderBuilder::new()
                        .has_headers(false)
                        .from_path(&SETTINGS.puzzle_db_location);

                match reader {
                    Ok(mut reader) => {
                        self.puzzles.clear();
                        self.current_puzzle_move = 1;
                        self.current_puzzle = 0;

                        for result in reader.deserialize::<Puzzle>() {
                            if let Ok(record) = result {                                
                                if record.rating >= self.slider_min_rating_value && record.rating <= self.slider_max_rating_value &&
                                        (self.theme == TaticsThemes::All ||
                                        record.themes.to_lowercase().contains(&self.theme.to_string().to_lowercase())) {
                                    self.puzzles.push(record);
                                }
                            }
                            if self.puzzles.len() == SETTINGS.search_results_limit {
                                break;
                            }
                        }
                        if !self.puzzles.is_empty() {
                            self.puzzles.shuffle(&mut thread_rng());
                            self.from_square = None;

                            self.board = Board::from_str(&self.puzzles[0].fen).unwrap();
                            let puzzle_moves: Vec<&str> = self.puzzles[0].moves.split_whitespace().collect();

                            // The last opponent's move is in the "moves" field of the cvs,
                            // so we need to apply it.
                            let movement = ChessMove::new(
                                    Square::from_str(&puzzle_moves[0][..2]).unwrap(),
                                    Square::from_str(&puzzle_moves[0][2..4]).unwrap(), check_promotion(&puzzle_moves[0]));

                            self.last_move_from = Some(PositionGUI::chesssquare_to_posgui(movement.get_source()));
                            self.last_move_to = Some(PositionGUI::chesssquare_to_posgui(movement.get_dest()));

                            self.board = self.board.make_move_new(movement);

                            if self.board.side_to_move() == Color::White {
                                self.puzzle_status = String::from("white to move!");
                            } else {
                                self.puzzle_status = String::from("Black to move!");
                            }
                            self.is_playing = true;
                        } else {
                            // Just putting the default position to make it obvious the search ended.
                            self.board = Board::default();
                            self.last_move_from = None;
                            self.last_move_to = None;
                            self.is_playing = false;
                            self.puzzle_status = String::from("Sorry, no puzzle found");
                        }
                    } Err(_) => {
                        self.puzzle_status = String::from("Problem reading the puzzle DB");
                    }
                }
            } (_, Message::SelectPiecePromotion(piece)) => {
                self.piece_to_promote_to = piece;
            }
            //_ => ()
        }
    }
    
    fn view(&mut self) -> Element<Message> {
        let mut result = Column::new().spacing(0).align_items(Align::Center);
        let mut row = Row::new().spacing(0).align_items(Align::Center);
        let mut i = 0;

        let is_white = self.board.side_to_move() == Color::White;
        for button in &mut self.squares {

            let rol: i32 = if is_white { 7 - i / 8 } else { i / 8 };
            let col: i32 = if is_white { i % 8 } else { 7 - (i % 8) };

            let pos = PositionGUI::new(rol, col);

            let piece = self.board.piece_on(pos.posgui_to_square());
            let color = self.board.color_on(pos.posgui_to_square());
            let mut text = "";

            if let Some(piece) = piece {
                if color.unwrap() == Color::White {
                    text = match piece {
                        Piece::Pawn => "/wP.png",
                        Piece::Rook => "/wR.png",
                        Piece::Knight => "/wN.png",
                        Piece::Bishop => "/wB.png",
                        Piece::Queen => "/wQ.png",
                        Piece::King => "/wK.png"
                    };
                } else {
                    text = match piece {
                        Piece::Pawn => "/bP.png",
                        Piece::Rook => "/bR.png",
                        Piece::Knight => "/bN.png",
                        Piece::Bishop => "/bB.png",
                        Piece::Queen => "/bQ.png",
                        Piece::King => "/bK.png"
                    };
                }
            }

            let selected = self.from_square == Some(pos) || self.last_move_from == Some(pos) || self.last_move_to == Some(pos);

            row = row.push(Button::new(button,
                Image::new(String::from(&SETTINGS.piece_theme) + text)
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .width(Length::Units(SETTINGS.square_size))
                .height(Length::Units(SETTINGS.square_size))
                .on_press(Message::SelectSquare(pos))
                .style(ChessSquare::from((pos, selected)))
            );

            i += 1;
            if i % 8 == 0 {
                result = result.push(row);
                row = Row::new().spacing(0).align_items(Align::Center);
            }            
        }

        let mut results_col = Column::new().spacing(0).align_items(Align::Center);

        let mut row_result = Row::new().spacing(0).align_items(Align::Center);
        row_result = row_result.push(Text::new(&self.puzzle_status)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center));

        results_col = results_col.push(row_result);
        
        //Search box
        let mut search_col = Column::new().spacing(0).align_items(Align::Center);

        let mut row_theme = Row::new().spacing(0).align_items(Align::Center);
        let theme_list = PickList::new(
                & mut self.theme_list,
                &TaticsThemes::ALL[..],
                Some(self.theme),
                Message::SelectTheme
        );

        let mut row_min_rating = Row::new().spacing(0).align_items(Align::Center);
        let slider_rating_min = Slider::new(
            &mut self.slider_min_rating_state,
            0..=3000,
            self.slider_min_rating_value,
            Message::SliderMinRatingChanged,
        );

        let mut row_max_rating = Row::new().spacing(0).align_items(Align::Center);
        let slider_rating_max = Slider::new(
            &mut self.slider_max_rating_state,
            0..=3000,
            self.slider_max_rating_value,
            Message::SliderMaxRatingChanged,
        );

        let mut row_search = Row::new().spacing(0).align_items(Align::Center);
        let btn_search = Button::new(&mut self.btn_search_state,
            Text::new("Search")).on_press(Message::ClickSearch);

        row_min_rating = row_min_rating.push(Text::new("Min. Rating: ")).push(slider_rating_min).push(
            Text::new(self.slider_min_rating_value.to_string())
                .width(Length::Shrink)
                .horizontal_alignment(HorizontalAlignment::Center),
        ).width(Length::Fill);

        row_max_rating = row_max_rating.push(Text::new("Max. Rating: ")).push(slider_rating_max).push(
            Text::new(self.slider_max_rating_value.to_string())
                .width(Length::Shrink)
                .horizontal_alignment(HorizontalAlignment::Center),
        ).width(Length::Fill);

        row_theme = row_theme.push(theme_list);
        row_search = row_search.push(btn_search);

        search_col = search_col.push(row_min_rating).push(row_max_rating).push(row_theme).push(row_search);

        // Promotion piece selector
        let mut promotion_col = Column::new().spacing(0).align_items(Align::Center);
        let mut row_promotion = Row::new().spacing(0).align_items(Align::Center);
        i = 0;
        for button in &mut self.btns_promotion {
            let piece;
            let mut image = String::from(&SETTINGS.piece_theme);
            match i {
                0 => {
                    piece = Piece::Rook;
                    image.push_str("/wR.png");
                }
                1 => {
                    piece = Piece::Knight;
                    image.push_str("/wN.png");
                }
                2 => {
                    piece = Piece::Bishop;
                    image.push_str("/wB.png");
                }
                _ => {
                    piece = Piece::Queen;
                    image.push_str("/wQ.png");
                }
            };
            row_promotion = row_promotion.push(Row::new().spacing(0).align_items(Align::Center).push(Button::new(button,
                Image::new(String::from(image))
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .width(Length::Units(SETTINGS.square_size/2))
                .height(Length::Units(SETTINGS.square_size/2))
                .on_press(Message::SelectPiecePromotion(piece))
                .style(PromotionStyle)
            ));
            i += 1;
        }
        promotion_col = promotion_col.push(
                Row::new().spacing(0).align_items(Align::Center).push(Text::new("Promotion piece:")
                .width(Length::Shrink)
                .horizontal_alignment(HorizontalAlignment::Center))
                .spacing(5)
        ).width(Length::Fill);
        promotion_col = promotion_col.push(row_promotion);

        result = result.push(results_col).push(search_col).push(promotion_col);
        Container::new(result)
            .style(ChessBoardStyle)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }
}

fn main() -> iced::Result {
    OfflinePuzzles::run(Settings {
        window: iced::window::Settings {
            size: (
                (SETTINGS.square_size * 8) as u32,
                (SETTINGS.square_size * 8) as u32 + 180
            ),
            resizable: true,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}