use iced::pure::widget::{button, Container, Button, Column, Text, Row, Svg, PickList, Slider};
use iced::pure::{Element};
use iced::{alignment, container, Command, Alignment, Length, Background};

use iced_aw::pure::TabLabel;
use chess::{Piece};
use crate::{Tab, Message, config, styles};

#[derive(Debug, Clone)]
pub enum SearchMesssage {
    SliderMinRatingChanged(i32),
    SliderMaxRatingChanged(i32),
    SelectTheme(TaticsThemes),
    SelectPiecePromotion(Piece),
    ClickSearch,
}

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
    
    pub fn get_tag_name(&self) -> &str {
        match self {
            TaticsThemes::All => "all",
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

struct PromotionStyle {bg_color: iced::Color }

impl PromotionStyle {
    fn new(bg_color: iced::Color) -> Self {
        Self { bg_color }
    }
}

impl button::StyleSheet for PromotionStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.bg_color)),
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
            background: Some(Background::Color(styles::SELECTED_DARK_SQUARE)),
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

#[derive(Debug, Clone)]
pub struct SearchTab {
    pub theme: TaticsThemes,

    slider_min_rating_value: i32,
    slider_max_rating_value: i32,    

    pub bg_color_promotion: iced::Color,
    pub piece_theme_promotion: styles::PieceTheme,
    pub piece_to_promote_to: Piece,
}

impl SearchTab {
    pub fn new() -> Self {
        SearchTab {
            theme : TaticsThemes::default(),

            slider_min_rating_value: 0,
            slider_max_rating_value: 1000,

            bg_color_promotion: config::SETTINGS.dark_squares_color.into(),
            piece_theme_promotion: config::SETTINGS.piece_theme,
            piece_to_promote_to: Piece::Queen,
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
            } SearchMesssage::SelectPiecePromotion(piece) => {
                self.piece_to_promote_to = piece;
                Command::none()
            } SearchMesssage::ClickSearch => {
                Command::perform(
                    SearchTab::search(self.slider_min_rating_value,
                           self.slider_max_rating_value,
                           self.theme), Message::LoadPuzzle)
            }
        }
    }
    pub async fn search(min_rating: i32, max_rating: i32, theme: TaticsThemes) -> Option<Vec<config::Puzzle>> {
        let mut puzzles: Vec<config::Puzzle> = Vec::new();
    
        let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(&config::SETTINGS.puzzle_db_location);
    
        match reader {
            Ok(mut reader) => {
                puzzles.clear();
                //self.current_puzzle_move = 1;
                //self.current_puzzle = 0;
    
                for result in reader.deserialize::<config::Puzzle>() {
                    if let Ok(record) = result {                                
                        if record.rating >= min_rating && record.rating <= max_rating &&
                                (theme == TaticsThemes::All ||
                                record.themes.contains(theme.get_tag_name())) {
                            puzzles.push(record);
                        }
                    }
                    if puzzles.len() == config::SETTINGS.search_results_limit {
                        break;
                    }
                }
            } Err(_) => {
                //self.puzzle_status = String::from("Problem reading the puzzle DB");
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
        TabLabel::IconText('\u{F217}'.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        
        let mut search_col = Column::new().spacing(10).align_items(Alignment::Center);

        let mut row_theme = Row::new().spacing(5).align_items(Alignment::Center);
        let theme_list = PickList::new(
            &TaticsThemes::ALL[..],
            Some(self.theme),
            SearchMesssage::SelectTheme
        );

        let mut row_min_rating = Row::new().spacing(5).align_items(Alignment::Center);
        let slider_rating_min = Slider::new(
            0..=3000,
            self.slider_min_rating_value,
            SearchMesssage::SliderMinRatingChanged,
        );

        let mut row_max_rating = Row::new().spacing(5).align_items(Alignment::Center);
        let slider_rating_max = Slider::new(
            0..=3000,
            self.slider_max_rating_value,
            SearchMesssage::SliderMaxRatingChanged,
        );

        let mut row_search = Row::new().spacing(5).align_items(Alignment::Center);
        let btn_search = Button::new(
            Text::new("Search")).on_press(SearchMesssage::ClickSearch);

        row_min_rating = row_min_rating.push(Text::new("Min. Rating: ")).push(slider_rating_min).push(
            Text::new(self.slider_min_rating_value.to_string())
                .width(Length::Shrink)
                .horizontal_alignment(alignment::Horizontal::Center),
        ).width(Length::Fill);

        row_max_rating = row_max_rating.push(Text::new("Max. Rating: ")).push(slider_rating_max).push(
            Text::new(self.slider_max_rating_value.to_string())
                .width(Length::Shrink)
                .horizontal_alignment(alignment::Horizontal::Center),
        ).width(Length::Fill);

        row_theme = row_theme.push(theme_list);
        row_search = row_search.push(btn_search);

        search_col = search_col.push(row_min_rating).push(row_max_rating).push(row_theme).push(row_search);

        // Promotion piece selector
        let mut promotion_col = Column::new().spacing(10).align_items(Alignment::Center);
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
            row_promotion = row_promotion.push(Row::new().spacing(5).align_items(Alignment::Center)
                .push(Button::new(
                    Svg::from_path(
                        String::from("pieces/") + &self.piece_theme_promotion.to_string() + image)
                )
                .width(Length::Units(config::SETTINGS.square_size))
                .height(Length::Units(config::SETTINGS.square_size))
                .on_press(SearchMesssage::SelectPiecePromotion(piece))
                .style(PromotionStyle::new(self.bg_color_promotion))
            ));            
        }
        promotion_col = promotion_col.push(
                Row::new().spacing(5).align_items(Alignment::Center).push(Text::new("Promotion piece:")
                .width(Length::Shrink)
                .horizontal_alignment(alignment::Horizontal::Center))
                .spacing(5)
        ).width(Length::Fill);
        promotion_col = promotion_col.push(row_promotion);
        
        search_col = search_col.push(promotion_col);
        let content: Element<'_, SearchMesssage> = Container::new(search_col)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into();
        
        content.map(Message::Search)
    }
}