use iced::widget::{Container, Button, column as col, Text, Radio, row, Row, Svg, PickList, Slider, Scrollable, Space};
use iced::{Element};
use iced::{alignment, Command, Alignment, Length};
use std::io::BufReader;

use iced_aw::TabLabel;
use chess::{Piece};
use crate::config::load_config;
use crate::{Tab, Message, config, styles};

#[derive(Debug, Clone)]
pub enum SearchMesssage {
    SliderMinRatingChanged(i32),
    SliderMaxRatingChanged(i32),
    SelectTheme(TaticsThemes),
    SelectOpening(Openings),
    SelectOpeningSide(OpeningSide),
    SelectPiecePromotion(Piece),
    ClickSearch,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
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
    
    pub fn get_tag_name(&self) -> &str {
        match self {
            TaticsThemes::All => "",
            TaticsThemes::Opening => "opening",
            TaticsThemes::Middlegame => "middlegame",
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
            TaticsThemes::XRayAttack =>"xRayAttack",
            TaticsThemes::Zugzwang => "zugzwang",
    
            TaticsThemes::Mate => "mate",
            TaticsThemes::MateIn1 => "mateIn1",
            TaticsThemes::MateIn2 => "mateIn2",
            TaticsThemes::MateIn3 => "mateIn3",
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
    }
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
                TaticsThemes::Opening => "Opening",
                TaticsThemes::Middlegame=> "Middlegame",
                TaticsThemes::Endgame => "Endgame",
                TaticsThemes::RookEndgame => "Rook endgame",
                TaticsThemes::BishopEndgame => "Bishop endgame",
                TaticsThemes::PawnEndgame => "Pawn endgame",
                TaticsThemes::KnightEndgame => "Knight endgame",
                TaticsThemes::QueenEndgame => "Queen endgame",
                TaticsThemes::QueenRookEndgame => "Queen and rook endgame",
        
                TaticsThemes::AdvancedPawn => "Advanced pawn",
                TaticsThemes::AtackingF2F7 => "Attacking f2/f7",
                TaticsThemes::CapturingDefender => "Capturing defender",
                TaticsThemes::DiscoveredAttack => "Discovered attack",
                TaticsThemes::DoubleCheck => "Double check",
                TaticsThemes::ExposedKing => "Exposed king",
                TaticsThemes::Fork => "Fork",
                TaticsThemes::HangingPiece => "Hanging piece",
                TaticsThemes::KingsideAttack => "Kingside attack",
                TaticsThemes::Pin => "Pin",
                TaticsThemes::QueensideAttack => "Queenside attack",
                TaticsThemes::Sacrifice => "Sacrifice",
                TaticsThemes::Skewer => "Skewer",
                TaticsThemes::TrappedPiece => "Trapped piece",

                TaticsThemes::Attraction => "Attraction",
                TaticsThemes::Clearance => "Clearance",
                TaticsThemes::DefensiveMove => "Defensive move",
                TaticsThemes::Deflection => "Deflection",
                TaticsThemes::Interference => "Interference",
                TaticsThemes::Intermezzo => "Intermezzo",
                TaticsThemes::QuietMove => "Quiet move",
                TaticsThemes::XRayAttack => "X-Ray attack",
                TaticsThemes::Zugzwang => "Zugzwang",
        
                TaticsThemes::Mate => "Mate",
                TaticsThemes::MateIn1 => "Mate in 1",
                TaticsThemes::MateIn2 => "Mate in 2",
                TaticsThemes::MateIn3 => "Mate in 3",
                TaticsThemes::MateIn4 => "Mate in 4",
                TaticsThemes::MateIn5 => "Mate in 5",
                TaticsThemes::AnastasiaMate => "Anastasia mate",
                TaticsThemes::ArabianMate => "Arabian mate",
                TaticsThemes::BackRankMate => "Back-rank mate",
                TaticsThemes::BodenMate => "Boden's mate",
                TaticsThemes::DoubleBishopMate => "Double bishop mate",
                TaticsThemes::DovetailMate => "Dovetail mate",
                TaticsThemes::HookMate => "Hook mate",
                TaticsThemes::SmotheredMate => "Smothered mate",

                TaticsThemes::Castling => "Castling",
                TaticsThemes::EnPassant => "En passant",
                TaticsThemes::Promotion => "Promotion",
                TaticsThemes::UnderPromotion => "Under promotion",
                TaticsThemes::Equality => "Equality",
                TaticsThemes::Advantage => "Advantage",
                TaticsThemes::Crushing => "Crushing",

                TaticsThemes::OneMove => "One move",
                TaticsThemes::Short => "Short",
                TaticsThemes::Long => "Long",
                TaticsThemes::VeryLong => "Very long",

                TaticsThemes::Master => "From games of titled players",
                TaticsThemes::MasterVsMaster => "From games between titled players",
                TaticsThemes::SuperGM => "From games of super GMs",

            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Openings {
    Any, AlekhineDefense, Benoni, Bird, BishopsOpening, BlackmarDiemerGambit, CaroKann, Catalan,
    Dutch, English, FourKnightsGame, French, GiuocoPiano, Grunfeld, HorwitzDefense, IndianDefense,
    ItalianGame, KingsGambit, KingsGambitAccepted, KingsGambitDeclined, KingsIndianAttack,
    KingsIndianDefense, KingsPawnGame, ModernDefense, NimzoIndianDefense, NimzoLarsenAttack,
    NimzowitschDefense, PhilidorDefense, PircDefense, Ponziani, QueensGambitAccepted,
    QueensGambitDeclined, QueensPawnGame, RapportJobavaSystem, Reti, RussianGame, RuyLopez,
    Scandinavian, ScotchGame, SemiSlav,Sicilian, SlavDefense, ThreeKnightsOpening, Trompowsky,
    ViennaGame, ZukertortOpening
}

impl Openings {
    const ALL: [Openings; 46] = [
        Openings::Any, Openings::AlekhineDefense, Openings::Benoni, Openings::Bird,
        Openings::BishopsOpening, Openings::BlackmarDiemerGambit, Openings::CaroKann,
        Openings::Catalan, Openings::Dutch, Openings::English, Openings::FourKnightsGame,
        Openings::French, Openings::GiuocoPiano, Openings::Grunfeld, Openings::HorwitzDefense,
        Openings::IndianDefense, Openings::ItalianGame, Openings::KingsGambit,
        Openings::KingsGambitAccepted, Openings::KingsGambitDeclined, Openings::KingsIndianAttack,
        Openings::KingsIndianDefense, Openings::KingsPawnGame, Openings::ModernDefense,
        Openings::NimzoIndianDefense, Openings::NimzoLarsenAttack, Openings::NimzowitschDefense,
        Openings::PhilidorDefense, Openings::PircDefense, Openings::Ponziani,
        Openings::QueensGambitAccepted, Openings::QueensGambitDeclined, Openings::QueensPawnGame,
        Openings::RapportJobavaSystem, Openings::Reti, Openings::RussianGame, Openings::RuyLopez,
        Openings::Scandinavian, Openings::ScotchGame, Openings::SemiSlav, Openings::Sicilian,
        Openings::SlavDefense, Openings::ThreeKnightsOpening, Openings::Trompowsky,
        Openings::ViennaGame, Openings::ZukertortOpening];

    pub fn get_field_name(&self) -> &str {
        match self {
            Openings::Any => "",
            Openings::Sicilian => "Sicilian_Defense", Openings::French => "French_Defense",
            Openings::QueensPawnGame => "Queens_Pawn_Game", Openings::ItalianGame => "Italian_Game",
            Openings::CaroKann => "Caro-Kann_Defense", Openings::QueensGambitDeclined => "Queens_Gambit_Declined",
            Openings::Scandinavian => "Scandinavian_Defense", Openings::RuyLopez => "Ruy_Lopez",
            Openings::English => "English_Opening", Openings::IndianDefense => "Indian_Defense",
            Openings::ScotchGame => "Scotch_Game", Openings::PhilidorDefense => "Philidor_Defense",
            Openings::RussianGame => "Russian_Game", Openings::ModernDefense => "Modern_Defense",
            Openings::FourKnightsGame => "Four_Knights_Game", Openings::KingsGambitAccepted => "Kings_Gambit_Accepted",
            Openings::SlavDefense => "Slav_Defense", Openings::PircDefense => "Pirc_Defense",
            Openings::ZukertortOpening => "Zukertort_Opening", Openings::BishopsOpening => "Bishops_Opening",
            Openings::KingsPawnGame => "Kings_Pawn_Game", Openings::ViennaGame => "Vienna_Game",
            Openings::KingsIndianDefense => "Kings_Indian_Defense", Openings::QueensGambitAccepted => "Queens_Gambit_Accepted",
            Openings::Benoni => "Benoni_Defense", Openings::AlekhineDefense => "Alekhine_Defense",
            Openings::NimzowitschDefense => "Nimzowitsch_Defense", Openings::HorwitzDefense => "Horwitz_Defense",
            Openings::NimzoLarsenAttack => "Nimzo-Larsen_Attack", Openings::KingsGambitDeclined => "Kings_Gambit_Declined",
            Openings::NimzoIndianDefense => "Nimzo-Indian_Defense", Openings::Bird => "Bird_Opening",
            Openings::Dutch => "Dutch_Defense", Openings::SemiSlav => "Semi-Slav_Defense",
            Openings::GiuocoPiano => "Giuoco_Piano", Openings::Grunfeld => "Grunfeld_Defense",
            Openings::ThreeKnightsOpening => "Three_Knights_Opening", Openings::Ponziani => "Ponziani_Opening",
            Openings::KingsIndianAttack => "Kings_Indian_Attack", Openings::BlackmarDiemerGambit => "Blackmar-Diemer_Gambit",
            Openings::Trompowsky => "Trompowsky_Attack", Openings::KingsGambit => "Kings_Gambit",
            Openings::RapportJobavaSystem => "Rapport-Jobava_System", Openings::Catalan => "Catalan_Opening",
            Openings::Reti => "Reti_Opening"
        }
    }
}

impl std::fmt::Display for Openings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Openings::Any => "Any",
                Openings::Sicilian => "Sicilian Defense", Openings::French => "French Defense",
                Openings::QueensPawnGame => "Queen's Pawn Game", Openings::ItalianGame => "Italian Game",
                Openings::CaroKann => "Caro-Kann Defense", Openings::QueensGambitDeclined => "Queen's Gambit Declined",
                Openings::Scandinavian => "Scandinavian Defense", Openings::RuyLopez => "Ruy Lopez",
                Openings::English => "English Opening", Openings::IndianDefense => "Indian Defense",
                Openings::ScotchGame => "Scotch Game", Openings::PhilidorDefense => "Philidor Defense",
                Openings::RussianGame => "Russian Game", Openings::ModernDefense => "Modern Defense",
                Openings::FourKnightsGame => "Four Knights Game", Openings::KingsGambitAccepted => "King's Gambit Accepted",
                Openings::SlavDefense => "Slav Defense", Openings::PircDefense => "Pirc Defense",
                Openings::ZukertortOpening => "Zukertort Opening", Openings::BishopsOpening => "Bishops Opening",
                Openings::KingsPawnGame => "King's Pawn Game", Openings::ViennaGame => "Vienna Game",
                Openings::KingsIndianDefense => "King's Indian Defense", Openings::QueensGambitAccepted => "Queen's Gambit Accepted",
                Openings::Benoni => "Benoni Defense", Openings::AlekhineDefense => "Alekhine Defense",
                Openings::NimzowitschDefense => "Nimzowitsch Defense", Openings::HorwitzDefense => "Horwitz Defense",
                Openings::NimzoLarsenAttack => "Nimzo-Larsen Attack", Openings::KingsGambitDeclined => "King's Gambit Declined",
                Openings::NimzoIndianDefense => "Nimzo-Indian Defense", Openings::Bird => "Bird Opening",
                Openings::Dutch => "Dutch Defense", Openings::SemiSlav => "Semi-Slav Defense",
                Openings::GiuocoPiano => "Giuoco Piano", Openings::Grunfeld => "Grunfeld Defense",
                Openings::ThreeKnightsOpening => "Three Knights Opening", Openings::Ponziani => "Ponziani Opening",
                Openings::KingsIndianAttack => "King's Indian Attack", Openings::BlackmarDiemerGambit => "Blackmar-Diemer Gambit",
                Openings::Trompowsky => "Trompowsky Attack", Openings::KingsGambit => "King's Gambit",
                Openings::RapportJobavaSystem => "Rapport-Jobava System", Openings::Catalan => "Catalan Opening",
                Openings::Reti => "Réti Opening"
                }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum OpeningSide {
    Any, White, Black
}

#[derive(Debug, Clone)]
pub struct SearchTab {
    pub theme: TaticsThemes,
    pub opening: Option<Openings>,
    pub opening_side: Option<OpeningSide>,
    slider_min_rating_value: i32,
    slider_max_rating_value: i32,    
    pub piece_theme_promotion: styles::PieceTheme,
    pub piece_to_promote_to: Piece,

    pub show_searching_msg: bool,
}

impl SearchTab {
    pub fn new() -> Self {
        SearchTab {
            theme : config::SETTINGS.last_theme,
            opening: config::SETTINGS.last_opening,
            opening_side: config::SETTINGS.last_opening_side,
            slider_min_rating_value: config::SETTINGS.last_min_rating,
            slider_max_rating_value: config::SETTINGS.last_max_rating,
            piece_theme_promotion: config::SETTINGS.piece_theme,
            piece_to_promote_to: Piece::Queen,
            show_searching_msg: false,
        }
    }

    pub fn update(&mut self, message: SearchMesssage) -> Command<Message> {//config::AppEvents {
        match message {
            SearchMesssage::SliderMinRatingChanged(new_value) => {
                self.slider_min_rating_value = new_value;
                Command::none()
            } SearchMesssage::SliderMaxRatingChanged(new_value) => {
                self.slider_max_rating_value = new_value;
                Command::none()
            } SearchMesssage::SelectTheme(new_theme) => {
                self.theme = new_theme;
                Command::none()   
            } SearchMesssage::SelectOpening(new_opening) => {
                self.opening = Some(new_opening);
                Command::none()
            } SearchMesssage::SelectOpeningSide(new_opening_side) => {
                self.opening_side = Some(new_opening_side);
                Command::none()
            } SearchMesssage::SelectPiecePromotion(piece) => {
                self.piece_to_promote_to = piece;
                Command::none()
            } SearchMesssage::ClickSearch => {
                self.show_searching_msg = true;
                SearchTab::save_search_settings(self.slider_min_rating_value,
                    self.slider_max_rating_value,
                    self.theme, self.opening, self.opening_side);

                let config = load_config();                    
                Command::perform(
                    SearchTab::search(self.slider_min_rating_value,
                           self.slider_max_rating_value,
                           self.theme, self.opening, self.opening_side, config.search_results_limit), Message::LoadPuzzle)
            }
        }
    }

    pub fn save_search_settings(min_rating: i32, max_rating: i32, theme: TaticsThemes, opening: Option<Openings>, op_side: Option<OpeningSide>) {
        let file = std::fs::File::open("settings.json");        
        if let Ok(file) = file {
            let buf_reader = BufReader::new(file);
            if let Ok(mut config) = serde_json::from_reader::<std::io::BufReader<std::fs::File>, config::OfflinePuzzlesConfig>(buf_reader) {
                config.last_min_rating = min_rating;
                config.last_max_rating = max_rating;
                config.last_theme = theme;
                config.last_opening = opening;
                config.last_opening_side = op_side;
                        
                let file = std::fs::File::create("settings.json");
                if let Ok(file) = file {
                    if serde_json::to_writer_pretty(file, &config).is_err() {
                        println!("Error saving search options.");
                    }
                }
            }
        }
    }
    
    pub async fn search(min_rating: i32, max_rating: i32, theme: TaticsThemes, opening: Option<Openings>, op_side: Option<OpeningSide>, result_limit: usize) -> Option<Vec<config::Puzzle>> {
        let mut puzzles: Vec<config::Puzzle> = Vec::new();
    
        let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(&config::SETTINGS.puzzle_db_location);
    
        if let Ok(mut reader) = reader {
            puzzles.clear();
            //self.current_puzzle_move = 1;
            //self.current_puzzle = 0;
            let op = match opening {
                None => Openings::Any,
                Some(x) => x
            };

            if op != Openings::Any {
                let side = match op_side {
                    None => OpeningSide::Any,
                    Some(x) => x
                };
                match side {
                    OpeningSide::Any => {                        
                        for result in reader.deserialize::<config::Puzzle>() {
                            if let Ok(record) = result {                                
                                if record.opening == op.get_field_name() &&
                                        record.rating >= min_rating && record.rating <= max_rating &&
                                        record.themes.contains(theme.get_tag_name()) {
                                    puzzles.push(record);
                                }
                            }
                            if puzzles.len() == result_limit {
                                break;
                            }
                        }
                    } OpeningSide::Black => {
                        for result in reader.deserialize::<config::Puzzle>() {
                            if let Ok(record) = result {                                
                                if record.opening == op.get_field_name() &&
                                        !record.game_url.contains("black") &&
                                        record.rating >= min_rating && record.rating <= max_rating &&
                                        record.themes.contains(theme.get_tag_name()) {
                                    puzzles.push(record);
                                }
                            }
                            if puzzles.len() == result_limit {
                                break;
                            }
                        }
                    } OpeningSide::White => {
                        for result in reader.deserialize::<config::Puzzle>() {
                            if let Ok(record) = result {                                
                                if record.opening == op.get_field_name() &&
                                        record.game_url.contains("black") &&
                                        record.rating >= min_rating && record.rating <= max_rating &&
                                        record.themes.contains(theme.get_tag_name()) {
                                    puzzles.push(record);
                                }
                            }
                            if puzzles.len() == result_limit {
                                break;
                            }
                        }
                    }
                }
            } else {
                for result in reader.deserialize::<config::Puzzle>() {
                    if let Ok(record) = result {                                
                        if record.rating >= min_rating && record.rating <= max_rating &&
                                record.themes.contains(theme.get_tag_name()) {
                            puzzles.push(record);
                        }
                    }
                    if puzzles.len() == result_limit {
                        break;
                    }
                }
            }
        }
        
        Some(puzzles)
    }

}


impl Tab for SearchTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Search")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText('\u{F217}', self.title())
    }

    fn content(&self) -> Element<Message, iced::Renderer<styles::Theme>> {

        let mut search_col = col![
            row![
                Text::new("Min. Rating: "),
                Slider::new(
                    0..=3000,
                    self.slider_min_rating_value,
                    SearchMesssage::SliderMinRatingChanged,
                ),
                Text::new(self.slider_min_rating_value.to_string())
            ].width(Length::Fill),
            row![
                Text::new("Max. Rating: "),
                Slider::new(
                    0..=3000,
                    self.slider_max_rating_value,
                    SearchMesssage::SliderMaxRatingChanged,
                ),
                Text::new(self.slider_max_rating_value.to_string())
                ].width(Length::Fill),
            Text::new("Tactics theme:"),
            PickList::new(
                &TaticsThemes::ALL[..],
                Some(self.theme),
                SearchMesssage::SelectTheme
            ),
            Text::new("In the opening:"),
            PickList::new(
                &Openings::ALL[..],
                self.opening,
                SearchMesssage::SelectOpening
            )
        ].spacing(10).align_items(Alignment::Center);

        if let Some(op) = self.opening {
            if op != Openings::Any {
                let row_color = row![
                    Radio::new(OpeningSide::Any, "Any", self.opening_side, SearchMesssage::SelectOpeningSide),
                    Radio::new(OpeningSide::White, "White", self.opening_side, SearchMesssage::SelectOpeningSide),
                    Radio::new(OpeningSide::Black, "Black", self.opening_side, SearchMesssage::SelectOpeningSide)
                ].spacing(5).align_items(Alignment::Center);
                search_col = search_col.push(Text::new("Side: ")).push(row_color);
            }
        }

        // Promotion piece selector
        let mut row_promotion = Row::new().spacing(5).align_items(Alignment::Center);
        for i in 0..4 {
            let piece;
            let image;
            match i {
                0 => {
                    piece = Piece::Rook;
                    image = "/wR.svg";
                }
                1 => {
                    piece = Piece::Knight;
                    image = "/wN.svg";
                }
                2 => {
                    piece = Piece::Bishop;
                    image = "/wB.svg";
                }
                _ => {
                    piece = Piece::Queen;
                    image = "/wQ.svg";
                }
            };
            let square_style =
                if self.piece_to_promote_to == piece {
                    styles::ButtonStyle::DarkSquare
                } else {   
                    styles::ButtonStyle::LightSquare
                };
            row_promotion = row_promotion.push(Row::new().spacing(5).align_items(Alignment::Center)
                .push(Button::new(
                    Svg::from_path(String::from("pieces/") + &self.piece_theme_promotion.to_string() + image)
                )
                .width(60)
                .height(60)
                .on_press(SearchMesssage::SelectPiecePromotion(piece))
                .style(square_style)
            ));            
        }

        search_col = search_col.push(Space::new(Length::Fill, 10));
        if self.show_searching_msg {
            search_col = search_col.push(Text::new("Searching, please wait..."));
        }
        search_col = search_col
            .push(Button::new(Text::new("Search")).padding(5).on_press(SearchMesssage::ClickSearch))
            .push(Text::new("Promotion piece:"))
            .push(row_promotion);

        let scroll = Scrollable::new(search_col);
        let content: Element<SearchMesssage, iced::Renderer<styles::Theme>> = Container::new(scroll)
            .align_x(alignment::Horizontal::Center).height(Length::Fill)
            .into();
        
        content.map(Message::Search)
    }
}