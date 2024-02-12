use iced::widget::svg::Handle;
use iced::widget::{Container, Button, column as col, Text, Radio, row, Row, Svg, PickList, Slider, Scrollable, Space};
use iced::widget::text::LineHeight;
use iced::{alignment, Command, Element, Alignment, Length};
use std::io::BufReader;

use iced_aw::TabLabel;
use chess::{Piece, PROMOTION_PIECES};
use crate::config::load_config;
use crate::styles::{PieceTheme, Theme};
use crate::{Tab, Message, config, styles, lang, db, openings};

use lang::{DisplayTranslated,PickListWrapper};
use openings::{Openings, Variation};

#[derive(Debug, Clone)]
pub enum SearchMesssage {
    SliderMinRatingChanged(i32),
    SliderMaxRatingChanged(i32),
    SelectTheme(PickListWrapper<TacticalThemes>),
    SelectOpening(PickListWrapper<Openings>),
    SelectVariation(PickListWrapper<Variation>),
    SelectOpeningSide(OpeningSide),
    SelectPiecePromotion(Piece),
    ClickSearch,
    SelectBase(SearchBase),
}

impl PickListWrapper<TacticalThemes> {
    pub fn get_themes(lang: lang::Language) -> Vec<PickListWrapper<TacticalThemes>> {
        let mut themes_wrapper = Vec::new();
        for theme in TacticalThemes::ALL {
            themes_wrapper.push(
                PickListWrapper::<TacticalThemes> {
                    lang,
                    item: theme,
                }
            );
        }
        themes_wrapper
    }

    pub fn new_theme(lang: lang::Language, theme: TacticalThemes) -> Self {
        Self { lang, item: theme}
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TacticalThemes {
    #[default]
    All,
    Opening, Middlegame, Endgame, RookEndgame, BishopEndgame, PawnEndgame, KnightEndgame, QueenEndgame, QueenRookEndgame,
    AdvancedPawn, AtackingF2F7, CapturingDefender, DiscoveredAttack, DoubleCheck, ExposedKing, Fork, HangingPiece, KingsideAttack, Pin, QueensideAttack, Sacrifice, Skewer, TrappedPiece,
    Attraction, Clearance, DefensiveMove, Deflection, Interference, Intermezzo, QuietMove, XRayAttack, Zugzwang,
    Mate, MateIn1, MateIn2, MateIn3, MateIn4, MateIn5, AnastasiaMate, ArabianMate, BackRankMate, BodenMate, DoubleBishopMate, DovetailMate, HookMate, SmotheredMate,
    Castling, EnPassant, Promotion, UnderPromotion, Equality, Advantage, Crushing,
    OneMove, Short, Long, VeryLong,
    Master, MasterVsMaster, SuperGM
}

impl TacticalThemes {

    const ALL: [TacticalThemes; 61] = [
        TacticalThemes::All,
        TacticalThemes::Opening, TacticalThemes::Middlegame, TacticalThemes::Endgame, TacticalThemes::RookEndgame,
        TacticalThemes::BishopEndgame, TacticalThemes::PawnEndgame, TacticalThemes::KnightEndgame,
        TacticalThemes::QueenEndgame, TacticalThemes::QueenRookEndgame,

        TacticalThemes::AdvancedPawn, TacticalThemes::AtackingF2F7, TacticalThemes::CapturingDefender,
        TacticalThemes::DiscoveredAttack, TacticalThemes::DoubleCheck, TacticalThemes::ExposedKing,
        TacticalThemes::Fork, TacticalThemes::HangingPiece, TacticalThemes::KingsideAttack, TacticalThemes::Pin,
        TacticalThemes::QueensideAttack, TacticalThemes::Sacrifice, TacticalThemes::Skewer,
        TacticalThemes::TrappedPiece,

        TacticalThemes::Attraction, TacticalThemes::Clearance, TacticalThemes::DefensiveMove,
        TacticalThemes::Deflection, TacticalThemes::Interference, TacticalThemes::Intermezzo,
        TacticalThemes::QuietMove, TacticalThemes::XRayAttack, TacticalThemes::Zugzwang,

        TacticalThemes::Mate, TacticalThemes::MateIn1, TacticalThemes::MateIn2, TacticalThemes::MateIn3,
        TacticalThemes::MateIn4, TacticalThemes::MateIn5, TacticalThemes::AnastasiaMate, TacticalThemes::ArabianMate,
        TacticalThemes::BackRankMate, TacticalThemes::BodenMate, TacticalThemes::DoubleBishopMate,
        TacticalThemes::DovetailMate, TacticalThemes::HookMate, TacticalThemes::SmotheredMate,

        TacticalThemes::Castling, TacticalThemes::EnPassant, TacticalThemes::Promotion,
        TacticalThemes::UnderPromotion, TacticalThemes::Equality, TacticalThemes::Advantage,
        TacticalThemes::Crushing,

        TacticalThemes::OneMove, TacticalThemes::Short, TacticalThemes::Long, TacticalThemes::VeryLong,

        TacticalThemes::Master, TacticalThemes::MasterVsMaster, TacticalThemes::SuperGM
    ];

    pub fn get_tr_key(&self) -> &str {
        match self {
            TacticalThemes::All => "themes_all",
            _ => self.get_tag_name(),
        }
    }

    pub fn get_tag_name(&self) -> &str {
        match self {
            TacticalThemes::All => "",
            TacticalThemes::Opening => "opening",
            TacticalThemes::Middlegame => "middlegame",
            TacticalThemes::Endgame => "endgame",
            TacticalThemes::RookEndgame => "rookEndgame",
            TacticalThemes::BishopEndgame => "bishopEndgame",
            TacticalThemes::PawnEndgame => "pawnEndgame",
            TacticalThemes::KnightEndgame => "knightEndgame",
            TacticalThemes::QueenEndgame => "queenEndgame",
            TacticalThemes::QueenRookEndgame => "queenRookEndgame",

            TacticalThemes::AdvancedPawn => "advancedPawn",
            TacticalThemes::AtackingF2F7 => "attackingF2F7",
            TacticalThemes::CapturingDefender => "capturingDefender",
            TacticalThemes::DiscoveredAttack => "discoveredAttack",
            TacticalThemes::DoubleCheck => "doubleCheck",
            TacticalThemes::ExposedKing => "exposedKing",
            TacticalThemes::Fork => "fork",
            TacticalThemes::HangingPiece => "hangingPiece",
            TacticalThemes::KingsideAttack => "kingsideAttack",
            TacticalThemes::Pin => "pin",
            TacticalThemes::QueensideAttack => "queensideAttack",
            TacticalThemes::Sacrifice => "sacrifice",
            TacticalThemes::Skewer => "skewer",
            TacticalThemes::TrappedPiece => "trappedPiece",

            TacticalThemes::Attraction => "attraction",
            TacticalThemes::Clearance => "clearance",
            TacticalThemes::DefensiveMove => "defensiveMove",
            TacticalThemes::Deflection => "deflection",
            TacticalThemes::Interference => "interference",
            TacticalThemes::Intermezzo => "intermezzo",
            TacticalThemes::QuietMove => "quietMove",
            TacticalThemes::XRayAttack =>"xRayAttack",
            TacticalThemes::Zugzwang => "zugzwang",

            TacticalThemes::Mate => "mate",
            TacticalThemes::MateIn1 => "mateIn1",
            TacticalThemes::MateIn2 => "mateIn2",
            TacticalThemes::MateIn3 => "mateIn3",
            TacticalThemes::MateIn4 => "mateIn4",
            TacticalThemes::MateIn5 => "mateIn5",
            TacticalThemes::AnastasiaMate => "anastasiaMate",
            TacticalThemes::ArabianMate => "arabianMate",
            TacticalThemes::BackRankMate => "backRankMate",
            TacticalThemes::BodenMate => "bodenMate",
            TacticalThemes::DoubleBishopMate => "doubleBishopMate",
            TacticalThemes::DovetailMate => "dovetailMate",
            TacticalThemes::HookMate => "hookMate",
            TacticalThemes::SmotheredMate => "smotheredMate",

            TacticalThemes::Castling => "castling",
            TacticalThemes::EnPassant => "enPassant",
            TacticalThemes::Promotion => "promotion",
            TacticalThemes::UnderPromotion => "underPromotion",
            TacticalThemes::Equality => "equality",
            TacticalThemes::Advantage => "advantage",
            TacticalThemes::Crushing => "crushing",

            TacticalThemes::OneMove => "oneMove",
            TacticalThemes::Short => "short",
            TacticalThemes::Long => "long",
            TacticalThemes::VeryLong => "veryLong",

            TacticalThemes::Master => "master",
            TacticalThemes::MasterVsMaster => "masterVsMaster",
            TacticalThemes::SuperGM => "superGM",
        }
    }

}

impl DisplayTranslated for TacticalThemes {
    fn to_str_tr(&self) -> &str {
        self.get_tr_key()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum OpeningSide {
    Any, White, Black
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum SearchBase {
    Lichess, Favorites
}

pub fn gen_piece_vec(theme: &PieceTheme) -> Vec<Handle> {
    let mut handles = Vec::<Handle>::with_capacity(5);
    let theme_str = &theme.to_string();
    // this first entry won't be used, it's there just to fill the vec, so we can index by the Piece
    handles.insert(0, Handle::from_path("pieces/cburnett/wP.svg"));
    handles.insert(Piece::Knight.to_index(), Handle::from_path(String::from("pieces/") + &theme_str + "/wN.svg"));
    handles.insert(Piece::Bishop.to_index(), Handle::from_path(String::from("pieces/") + &theme_str + "/wB.svg"));
    handles.insert(Piece::Rook.to_index(), Handle::from_path(String::from("pieces/") + &theme_str + "/wR.svg"));
    handles.insert(Piece::Queen.to_index(), Handle::from_path(String::from("pieces/") + &theme_str + "/wQ.svg"));
    handles
}

#[derive(Debug)]
pub struct SearchTab {
    pub theme: PickListWrapper<TacticalThemes>,
    pub opening: PickListWrapper<Openings>,
    pub variation: PickListWrapper<Variation>,
    pub opening_side: Option<OpeningSide>,
    slider_min_rating_value: i32,
    slider_max_rating_value: i32,
    pub piece_theme_promotion: styles::PieceTheme,
    pub piece_to_promote_to: Piece,

    pub show_searching_msg: bool,
    pub lang: lang::Language,
    base: Option<SearchBase>,
    pub promotion_piece_img: Vec<Handle>,
}

impl SearchTab {
    pub fn new() -> Self {
        SearchTab {
            theme : PickListWrapper::new_theme(config::SETTINGS.lang, config::SETTINGS.last_theme),
            opening: PickListWrapper::new_opening(config::SETTINGS.lang, config::SETTINGS.last_opening),
            variation: PickListWrapper::new_variation(config::SETTINGS.lang, config::SETTINGS.last_variation.clone()),
            opening_side: config::SETTINGS.last_opening_side,
            slider_min_rating_value: config::SETTINGS.last_min_rating,
            slider_max_rating_value: config::SETTINGS.last_max_rating,
            piece_theme_promotion: config::SETTINGS.piece_theme,
            piece_to_promote_to: Piece::Queen,
            show_searching_msg: false,
            lang: config::SETTINGS.lang,
            base: Some(SearchBase::Lichess),
            promotion_piece_img: gen_piece_vec(&config::SETTINGS.piece_theme),
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
                self.opening = new_opening;
                self.variation.item = Variation::ANY;
                Command::none()
            } SearchMesssage::SelectVariation(new_variation) => {
                self.variation = new_variation;
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
                    self.slider_max_rating_value, self.theme.item,
                    self.opening.item, self.variation.item.clone(), self.opening_side);

                let config = load_config();
                if self.base == Some(SearchBase::Favorites) {
                    Command::perform(
                        SearchTab::search_favs(self.slider_min_rating_value,
                            self.slider_max_rating_value,
                            self.theme.item, self.opening.item, self.variation.item.clone(),
                            self.opening_side, config.search_results_limit), Message::LoadPuzzle)
                } else {
                    Command::perform(
                        SearchTab::search(self.slider_min_rating_value,
                            self.slider_max_rating_value,
                            self.theme.item, self.opening.item, self.variation.item.clone(),
                            self.opening_side, config.search_results_limit), Message::LoadPuzzle)
                }
            } SearchMesssage::SelectBase(base) => {
                self.base = Some(base);
                Command::none()
            }
        }
    }

    pub fn save_search_settings(min_rating: i32, max_rating: i32, theme: TacticalThemes, opening: Openings, variation: Variation, op_side: Option<OpeningSide>) {
        let file = std::fs::File::open("settings.json");
        if let Ok(file) = file {
            let buf_reader = BufReader::new(file);
            if let Ok(mut config) = serde_json::from_reader::<std::io::BufReader<std::fs::File>, config::OfflinePuzzlesConfig>(buf_reader) {
                config.last_min_rating = min_rating;
                config.last_max_rating = max_rating;
                config.last_theme = theme;
                config.last_opening = opening;
                config.last_variation = variation;
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

    pub async fn search_favs(min_rating: i32, max_rating: i32, theme: TacticalThemes, opening: Openings, variation:Variation, op_side: Option<OpeningSide>, result_limit: usize) -> Option<Vec<config::Puzzle>> {
        db::get_favorites(min_rating, max_rating, theme, opening, variation, op_side, result_limit)
    }

    pub async fn search(min_rating: i32, max_rating: i32, theme: TacticalThemes, opening: Openings, variation: Variation, op_side: Option<OpeningSide>, result_limit: usize) -> Option<Vec<config::Puzzle>> {
        let mut puzzles: Vec<config::Puzzle> = Vec::new();

        let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(&config::SETTINGS.puzzle_db_location);

        if let Ok(mut reader) = reader {
            puzzles.clear();
            //self.current_puzzle_move = 1;
            //self.current_puzzle = 0;
            if opening != Openings::Any {
                let opening_tag: &str = if variation.name != Variation::ANY_STR {
                    &variation.name
                } else {
                    opening.get_field_name()
                };
                let side = match op_side {
                    None => OpeningSide::Any,
                    Some(x) => x
                };
                match side {
                    OpeningSide::Any => {
                        for result in reader.deserialize::<config::Puzzle>() {
                            if let Ok(record) = result {
                                if record.opening.contains(opening_tag) &&
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
                                if record.opening.contains(opening_tag) &&
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
                                if record.opening.contains(opening_tag) &&
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
        lang::tr(&self.lang, "search")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<Message, Theme, iced::Renderer> {
        let mut search_col = col![
            Container::new(
                row![
                    Radio::new(lang::tr(&self.lang, "lichess_db"), SearchBase::Lichess, self.base, SearchMesssage::SelectBase),
                    Radio::new(lang::tr(&self.lang, "my_favories"), SearchBase::Favorites, self.base, SearchMesssage::SelectBase),
                ].spacing(10)
            ).align_x(alignment::Horizontal::Center).width(Length::Fill),
            row![
                Text::new(lang::tr(&self.lang, "min_rating")),
                Slider::new(
                    0..=3000,
                    self.slider_min_rating_value,
                    SearchMesssage::SliderMinRatingChanged,
                ),
                Text::new(self.slider_min_rating_value.to_string())
            ].width(Length::Fill),
            row![
                Text::new(lang::tr(&self.lang, "max_rating")),
                Slider::new(
                    0..=3000,
                    self.slider_max_rating_value,
                    SearchMesssage::SliderMaxRatingChanged,
                ),
                Text::new(self.slider_max_rating_value.to_string())
                ].width(Length::Fill),
            Text::new(lang::tr(&self.lang, "theme_label")),
            PickList::new(
                PickListWrapper::get_themes(self.lang),
                Some(self.theme.clone()),
                SearchMesssage::SelectTheme
            ),
            Text::new(lang::tr(&self.lang, "in_opening")),
            PickList::new(
                PickListWrapper::get_openings(self.lang),
                Some(self.opening.clone()),
                SearchMesssage::SelectOpening
            ),
            Text::new(lang::tr(&self.lang, "in_the_variation")),
            PickList::new(
                PickListWrapper::get_variations(self.lang, Some(&self.opening.item)),
                Some(self.variation.clone()),
                SearchMesssage::SelectVariation
            )
        ].spacing(10).align_items(Alignment::Center);

        if self.opening.item != Openings::Any {
            let row_color = row![
                Radio::new(lang::tr(&self.lang, "any"), OpeningSide::Any, self.opening_side, SearchMesssage::SelectOpeningSide),
                Radio::new(lang::tr(&self.lang, "white"), OpeningSide::White, self.opening_side, SearchMesssage::SelectOpeningSide),
                Radio::new(lang::tr(&self.lang, "black"), OpeningSide::Black, self.opening_side, SearchMesssage::SelectOpeningSide)
            ].spacing(5).align_items(Alignment::Center);
            search_col = search_col.push(Text::new(lang::tr(&self.lang, "side"))).push(row_color);
        }

        let mut row_promotion = Row::new().spacing(5).align_items(Alignment::Center);
        if self.piece_theme_promotion == PieceTheme::FontAlpha {
            // Promotion piece selector
            for i in 0..4 {
                let piece;
                let mut text;
                match i {
                    0 => {
                        piece = Piece::Rook;
                        text = String::from("r");
                    }
                    1 => {
                        piece = Piece::Knight;
                        text = String::from("h");
                    }
                    2 => {
                        piece = Piece::Bishop;
                        text = String::from("b");
                    }
                    _ => {
                        piece = Piece::Queen;
                        text = String::from("q");
                    }
                };
                if self.piece_to_promote_to == piece {
                    text = text.to_uppercase();
                };
                row_promotion = row_promotion.push(Row::new().spacing(0).align_items(Alignment::Center)
                    .push(Button::new(
                        Text::new(text).font(config::CHESS_ALPHA).size(60).line_height(LineHeight::Absolute(60.into()))
                    )
                    .padding(0)
                    .width(60)
                    .height(60)
                    .style(styles::ButtonStyle::Paper)
                    .on_press(SearchMesssage::SelectPiecePromotion(piece))
                ));
            }
        } else {
            for piece in PROMOTION_PIECES {
                let square_style =
                    if self.piece_to_promote_to == piece {
                        styles::ButtonStyle::DarkSquare
                    } else {
                        styles::ButtonStyle::LightSquare
                    };
                row_promotion = row_promotion.push(Row::new().spacing(5).align_items(Alignment::Center)
                    .push(Button::new(
                        Svg::new(self.promotion_piece_img[piece.to_index()].clone())
                    )
                    .width(60)
                    .height(60)
                    .on_press(SearchMesssage::SelectPiecePromotion(piece))
                    .style(square_style)
                ));
            }
        }

        search_col = search_col.push(Space::new(Length::Fill, 10));
        if self.show_searching_msg {
            search_col = search_col.push(Text::new(lang::tr(&self.lang, "searching")));
        }
        search_col = search_col
            .push(Button::new(Text::new(lang::tr(&self.lang, "btn_search"))).padding(5).on_press(SearchMesssage::ClickSearch))
            .push(Text::new(lang::tr(&self.lang, "promotion_piece")))
            .push(row_promotion);

        let scroll = Scrollable::new(search_col);
        let content: Element<SearchMesssage, Theme, iced::Renderer> = Container::new(scroll)
            .align_x(alignment::Horizontal::Center).height(Length::Fill)
            .into();

        content.map(Message::Search)
    }
}
