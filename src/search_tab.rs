use iced::widget::{Container, Button, column as col, Text, Radio, row, Row, Svg, PickList, Slider, Scrollable, Space};
use iced::widget::text::LineHeight;
use iced::{alignment, Command, Element, Alignment, Length};
use std::io::BufReader;

use iced_aw::TabLabel;
use chess::Piece;
use crate::config::load_config;
use crate::styles::PieceTheme;
use crate::{Tab, Message, config, styles, lang, db, openings};

use lang::{DisplayTranslated,PickListWrapper};
use openings::{Openings, Variation};

#[derive(Debug, Clone)]
pub enum SearchMesssage {
    SliderMinRatingChanged(i32),
    SliderMaxRatingChanged(i32),
    SelectTheme(PickListWrapper<TaticsThemes>),
    SelectOpening(PickListWrapper<Openings>),
    SelectVariation(PickListWrapper<Variation>),
    SelectOpeningSide(OpeningSide),
    SelectPiecePromotion(Piece),
    ClickSearch,
    SelectBase(SearchBase),
}

impl PickListWrapper<TaticsThemes> {
    pub fn get_themes(lang: lang::Language) -> Vec<PickListWrapper<TaticsThemes>> {
        let mut themes_wrapper = Vec::new();
        for theme in TaticsThemes::ALL {
            themes_wrapper.push(
                PickListWrapper::<TaticsThemes> {
                    lang: lang,
                    item: theme,
                }
            );
        }
        themes_wrapper
    }

    pub fn new_theme(lang: lang::Language, theme: TaticsThemes) -> Self {
        Self { lang, item: theme}
    }
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

    pub fn get_tr_key(&self) -> &str {
        match self {
            TaticsThemes::All => "themes_all",
            _ => self.get_tag_name(),
        }
    }

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

impl DisplayTranslated for TaticsThemes {
    fn to_str_tr(&self) -> &str {
        self.get_tr_key()
    }
}

impl Default for TaticsThemes {
    fn default() -> TaticsThemes {
        TaticsThemes::Mate
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

#[derive(Debug)]
pub struct SearchTab {
    pub theme: PickListWrapper<TaticsThemes>,
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

    pub fn save_search_settings(min_rating: i32, max_rating: i32, theme: TaticsThemes, opening: Openings, variation: Variation, op_side: Option<OpeningSide>) {
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

    pub async fn search_favs(min_rating: i32, max_rating: i32, theme: TaticsThemes, opening: Openings, variation:Variation, op_side: Option<OpeningSide>, result_limit: usize) -> Option<Vec<config::Puzzle>> {
        db::get_favorites(min_rating, max_rating, theme, opening, variation, op_side, result_limit)
    }

    pub async fn search(min_rating: i32, max_rating: i32, theme: TaticsThemes, opening: Openings, variation: Variation, op_side: Option<OpeningSide>, result_limit: usize) -> Option<Vec<config::Puzzle>> {
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

    fn content(&self) -> Element<Message, iced::Renderer<styles::Theme>> {
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
                PickListWrapper::get_themes(self.lang.clone()),
                Some(self.theme.clone()),
                SearchMesssage::SelectTheme
            ),
            Text::new(lang::tr(&self.lang, "in_opening")),
            PickList::new(
                PickListWrapper::get_openings(self.lang.clone()),
                Some(self.opening.clone()),
                SearchMesssage::SelectOpening
            ),
            Text::new(lang::tr(&self.lang, "in_the_variation")),
            PickList::new(
                PickListWrapper::get_variations(self.lang.clone(), Some(&self.opening.item)),
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
        let content: Element<SearchMesssage, iced::Renderer<styles::Theme>> = Container::new(scroll)
            .align_x(alignment::Horizontal::Center).height(Length::Fill)
            .into();

        content.map(Message::Search)
    }
}
