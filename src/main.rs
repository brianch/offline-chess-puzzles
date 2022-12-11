#![windows_subsystem = "windows"]

use iced::pure::widget::{button, Svg, Container, Button, Row, Column, Text, Radio};
use iced::pure::{Application, Element};
use iced::{executor, alignment, Command, Alignment, Length, Background, Settings };
use iced_aw::pure::{TabLabel, Tabs};
use chess::{Board, BoardStatus, ChessMove, Color, Piece, Rank, Square, File, Game};
use std::str::FromStr;
use soloud::{Soloud, Wav, audio, AudioExt, LoadExt};
use rand::thread_rng;
use rand::seq::SliceRandom;

mod config;
mod styles;
mod search_tab;
use search_tab::{SearchMesssage, SearchTab};

mod settings;
use settings::{SettingsMessage, SettingsTab};

mod puzzles;
use puzzles::{PuzzleMessage, PuzzleTab};

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositionGUI {
    row: i32,
    col: i32,
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
    Search(SearchMesssage),
    Settings(SettingsMessage),
    PuzzleInfo(PuzzleMessage),
    SelectMode(config::GameMode),
    TabSelected(usize),
    ShowHint(Option<Square>),
    ShowNextPuzzle,
    GoBackMove,
    RedoPuzzle,
    LoadPuzzle(Option<Vec<config::Puzzle>>),
    ChangeSettings(Option<config::OfflinePuzzlesConfig>)
}

struct ChessSquare { row: i32, col: i32, is_selected: bool, light_sqr: [f32; 3], dark_sqr: [f32; 3]}

impl From<(PositionGUI, bool, [f32; 3], [f32; 3])> for ChessSquare {
    fn from(pos_color: (PositionGUI, bool, [f32; 3], [f32; 3])) -> Self {
        let (pos, is_selected, light_sqr, dark_sqr) = pos_color;
        Self::new(pos.get_row(), pos.get_col(), is_selected, light_sqr, dark_sqr)
    }
}

impl ChessSquare {
    fn new(row: i32, col: i32, is_selected: bool, light_sqr: [f32; 3], dark_sqr: [f32; 3]) -> Self {
        Self { row, col, is_selected, light_sqr, dark_sqr }
    }

    fn get_bg_color(&self, is_selected: bool, light_sqr: [f32; 3], dark_sqr: [f32; 3]) -> iced::Color {
        if (self.row * 9 + self.col) % 2 == 1 {
            if is_selected {
                styles::SELECTED_LIGHT_SQUARE
            } else {
                light_sqr.into()
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if is_selected {
                styles::SELECTED_DARK_SQUARE
            } else {
                dark_sqr.into()
            }
        }
    }
}

impl button::StyleSheet for ChessSquare {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.get_bg_color(self.is_selected, self.light_sqr, self.dark_sqr))),
            border_color: self.get_bg_color(self.is_selected, self.light_sqr, self.dark_sqr),
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
            background: Some(Background::Color(self.get_bg_color(true, self.light_sqr, self.dark_sqr))),
            border_color: self.get_bg_color(true, self.light_sqr, self.dark_sqr),
            border_radius: 0.0,
            border_width: 0.0,
            ..button::Style::default()
        }
    }
}

//#[derive(Clone)]
struct OfflinePuzzles {
    from_square: Option<PositionGUI>,
    board: Board,
    last_move_from: Option<PositionGUI>,
    last_move_to: Option<PositionGUI>,
    hint_square: Option<PositionGUI>,
    puzzle_status: String,

    analysis: Game,
    analysis_history: Vec<Board>,

    active_tab: usize,
    search_tab: SearchTab,
    settings_tab: SettingsTab,
    puzzle_tab: PuzzleTab,
    game_mode: config::GameMode,
    sound_player: Option<Soloud>,
    two_pieces_sound: Option<Wav>,
    one_piece_sound: Option<Wav>
}

impl Default for OfflinePuzzles {
    fn default() -> Self {
        Self {
            from_square: None,
            board: Board::default(),
            last_move_from: None,
            last_move_to: None,
            hint_square: None,

            analysis: Game::new(),
            analysis_history: vec![Board::default()],

            puzzle_status: String::from("Use the search."),
            search_tab: SearchTab::new(),
            settings_tab: SettingsTab::new(),
            puzzle_tab: PuzzleTab::new(),
            active_tab: 0,

            game_mode: config::GameMode::Puzzle,
            sound_player: Soloud::default().ok(),
            two_pieces_sound: load_two_pieces_sound(),
            one_piece_sound: load_one_piece_sound(),
        }
    }
}

fn load_two_pieces_sound() -> Option<Wav> {
    let mut sound = audio::Wav::default();
    match sound.load("2pieces.wav") {
        Ok(_) => Some(sound),
        Err(_) => None,
    }
}

fn load_one_piece_sound() -> Option<Wav> {
    let mut sound = audio::Wav::default();
    match sound.load("1piece.wav") {
        Ok(_) => Some(sound),
        Err(_) => None,
    }
}

fn get_notation_string(board: Board, promo_piece: Piece, from: PositionGUI, to: PositionGUI) -> String {

    let mut move_made_notation = from.posgui_to_notation() + &to.posgui_to_notation();
    let piece = board.piece_on(from.posgui_to_square());
    let color = board.color_on(from.posgui_to_square());

    // Check for promotion and adjust the notation accordingly
    if let (Some(piece), Some(color)) = (piece, color) {
        if piece == Piece::Pawn && ((color == Color::White && to.get_row() == 7) ||
                                   (color == Color::Black && to.get_row() == 0)) {
            match promo_piece {
                Piece::Rook => move_made_notation += "r",
                Piece::Knight => move_made_notation += "n",
                Piece::Bishop => move_made_notation += "b",
                _ => move_made_notation += "q"
            }
        }
    }
    move_made_notation
}

impl Application for OfflinePuzzles {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (OfflinePuzzles, Command<Message>) {
        (
            Self::default(),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Offline Chess Puzzles")
    }

    fn update(&mut self, message: self::Message) -> Command<Message> {
        match (self.from_square, message) {
            (None, Message::SelectSquare(pos)) => {
                let side =
                    match self.game_mode {
                        config::GameMode::Analysis => { self.analysis.side_to_move() }
                        config::GameMode::Puzzle => { self.board.side_to_move() }
                    };
                let color =
                    match self.game_mode {
                        config::GameMode::Analysis => { self.analysis.current_position().color_on(pos.posgui_to_square()) }
                        config::GameMode::Puzzle => { self.board.color_on(pos.posgui_to_square()) }
                    };

                if (self.puzzle_tab.is_playing || self.game_mode == config::GameMode::Analysis) && color == Some(side) {
                    self.hint_square = None;
                    self.from_square = Some(pos);
                }
                Command::none()
            } (Some(from), Message::SelectSquare(to)) if from != to => {
                let side =
                    match self.game_mode {
                        config::GameMode::Analysis => { self.analysis.side_to_move() }
                        config::GameMode::Puzzle => { self.board.side_to_move() }
                    };
                let color =
                    match self.game_mode {
                        config::GameMode::Analysis => { self.analysis.current_position().color_on(to.posgui_to_square()) }
                        config::GameMode::Puzzle => { self.board.color_on(to.posgui_to_square()) }
                    };
                // If the user clicked on another piece of his own side,
                // just replace the previous selection and exit
                if self.puzzle_tab.is_playing && color == Some(side) {
                    self.from_square = Some(to);
                    return Command::none()
                }
                self.from_square = None;

                if self.game_mode == config::GameMode::Analysis {
                     let move_made_notation =
                        get_notation_string(self.analysis.current_position(), self.search_tab.piece_to_promote_to, from, to);

                    let move_made = ChessMove::new(
                        Square::from_str(&String::from(&move_made_notation[..2])).unwrap(),
                        Square::from_str(&String::from(&move_made_notation[2..4])).unwrap(), PuzzleTab::check_promotion(&move_made_notation));

                    if self.analysis.make_move(move_made) {
                        self.analysis_history.push(self.analysis.current_position());
                        if self.settings_tab.saved_configs.play_sound {
                            if let (Some(soloud), Some(wav)) = (&self.sound_player, &self.one_piece_sound) {
                                soloud.play(wav);
                            }
                        }
                    }
                } else if !self.puzzle_tab.puzzles.is_empty() {
                    let movement;
                    let move_made_notation =
                        get_notation_string(self.board, self.search_tab.piece_to_promote_to, from, to);

                    let move_made = ChessMove::new(
                        Square::from_str(&String::from(&move_made_notation[..2])).unwrap(),
                        Square::from_str(&String::from(&move_made_notation[2..4])).unwrap(), PuzzleTab::check_promotion(&move_made_notation));

                    let is_mate = self.board.legal(move_made) && self.board.make_move_new(move_made).status() == BoardStatus::Checkmate;

                    let correct_moves : Vec<&str> = self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].moves.split_whitespace().collect::<Vec<&str>>();
                    let correct_move = ChessMove::new(
                        Square::from_str(&String::from(&correct_moves[self.puzzle_tab.current_puzzle_move][..2])).unwrap(),
                        Square::from_str(&String::from(&correct_moves[self.puzzle_tab.current_puzzle_move][2..4])).unwrap(), PuzzleTab::check_promotion(correct_moves[self.puzzle_tab.current_puzzle_move]));

                    // If the move is correct we can apply it to the board
                    if is_mate || (move_made == correct_move) {

                        self.board = self.board.make_move_new(move_made);
                        self.analysis_history.push(self.board);

                        self.puzzle_tab.current_puzzle_move += 1;

                        if self.puzzle_tab.current_puzzle_move == correct_moves.len() {
                            if self.settings_tab.saved_configs.play_sound {
                                if let (Some(soloud), Some(wav)) = (&self.sound_player, &self.one_piece_sound) {
                                    soloud.play(wav);
                                }
                            }
                            if self.puzzle_tab.current_puzzle < self.puzzle_tab.puzzles.len() - 1 {
                                if self.settings_tab.saved_configs.auto_load_next {
                                    // The previous puzzle ended, and we still have puzzles available,
                                    // so we prepare the next one.
                                    self.puzzle_tab.current_puzzle += 1;
                                    self.puzzle_tab.current_puzzle_move = 1;

                                    let puzzle_moves: Vec<&str> = self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].moves.split_whitespace().collect();

                                    // The opponent's last move (before the puzzle starts)
                                    // is in the "moves" field of the cvs, so we need to apply it.
                                    self.board = Board::from_str(&self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].fen).unwrap();

                                    movement = ChessMove::new(
                                        Square::from_str(&String::from(&puzzle_moves[0][..2])).unwrap(),
                                        Square::from_str(&String::from(&puzzle_moves[0][2..4])).unwrap(), PuzzleTab::check_promotion(puzzle_moves[0]));

                                    self.last_move_from = Some(PositionGUI::chesssquare_to_posgui(movement.get_source()));
                                    self.last_move_to = Some(PositionGUI::chesssquare_to_posgui(movement.get_dest()));

                                    self.board = self.board.make_move_new(movement);
                                    self.analysis_history = vec![self.board];

                                    if self.board.side_to_move() == Color::White {
                                        self.puzzle_status = String::from("White to move!");
                                    } else {
                                        self.puzzle_status = String::from("Black to move!");
                                    }
                                    self.puzzle_tab.current_puzzle_side = self.board.side_to_move();
                                } else {
                                    self.puzzle_status = String::from("Well done!");
                                    self.puzzle_tab.is_playing = false;
                                }
                            } else {
                                self.board = Board::default();
                                // quite meaningless but allows the user to use the takeback button
                                // to analyze a full game in analysis mode after the puzzles ended.
                                self.analysis_history = vec![self.board];
                                self.puzzle_tab.current_puzzle_move = 1;

                                self.last_move_from = None;
                                self.last_move_to = None;
                                self.puzzle_tab.is_playing = false;
                                self.puzzle_status = String::from("All puzzles done for this search!");
                            }
                        } else {
                            if self.settings_tab.saved_configs.play_sound {
                                if let (Some(soloud), Some(wav)) = (&self.sound_player, &self.two_pieces_sound) {
                                    soloud.play(wav);
                                }
                            }
                            movement = ChessMove::new(
                                Square::from_str(&String::from(&correct_moves[self.puzzle_tab.current_puzzle_move][..2])).unwrap(),
                                Square::from_str(&String::from(&correct_moves[self.puzzle_tab.current_puzzle_move][2..4])).unwrap(), PuzzleTab::check_promotion(correct_moves[self.puzzle_tab.current_puzzle_move]));

                            self.last_move_from = Some(PositionGUI::chesssquare_to_posgui(movement.get_source()));
                            self.last_move_to = Some(PositionGUI::chesssquare_to_posgui(movement.get_dest()));

                            self.board = self.board.make_move_new(movement);
                            self.analysis_history.push(self.board);

                            self.puzzle_tab.current_puzzle_move += 1;
                            self.puzzle_status = String::from("Correct! What now?");
                        }
                    } else {
                        #[allow(clippy::collapsible_else_if)]
                        if self.board.side_to_move() == Color::White {
                            self.puzzle_status = String::from("Ops! Wrong move... White to play.");
                        } else {
                            self.puzzle_status = String::from("Ops! Wrong move... Black to play.");
                        }
                    }
                }
                Command::none()
            } (Some(_), Message::SelectSquare(to)) => {
                self.from_square = Some(to);
                Command::none()
            } (_, Message::TabSelected(selected)) => {
                self.active_tab = selected;
                Command::none()
            } (_, Message::Settings(message)) => {
                self.settings_tab.update(message)
            } (_, Message::SelectMode(message)) => {
                self.game_mode = message;
                if message == config::GameMode::Analysis {
                    self.analysis = Game::new_with_board(self.board);
                } else {
                    self.analysis_history.truncate(self.puzzle_tab.current_puzzle_move);
                }
                Command::none()
            } (_, Message::ShowHint(square)) => {
                if self.game_mode == config::GameMode::Puzzle {
                    match square {
                        Some(square) => {
                            self.hint_square = Some(PositionGUI::chesssquare_to_posgui(square));
                        } None => {
                            self.hint_square = None;
                        }
                    }
                }
                Command::none()
            } (_, Message::ShowNextPuzzle) => {
                // The previous puzzle ended, and we still have puzzles available,
                // so we prepare the next one.
                self.puzzle_tab.current_puzzle += 1;
                self.puzzle_tab.current_puzzle_move = 1;

                let puzzle_moves: Vec<&str> = self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].moves.split_whitespace().collect();

                // The opponent's last move (before the puzzle starts)
                // is in the "moves" field of the cvs, so we need to apply it.
                self.board = Board::from_str(&self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].fen).unwrap();

                let movement = ChessMove::new(
                    Square::from_str(&String::from(&puzzle_moves[0][..2])).unwrap(),
                    Square::from_str(&String::from(&puzzle_moves[0][2..4])).unwrap(), PuzzleTab::check_promotion(puzzle_moves[0]));

                self.last_move_from = Some(PositionGUI::chesssquare_to_posgui(movement.get_source()));
                self.last_move_to = Some(PositionGUI::chesssquare_to_posgui(movement.get_dest()));

                self.board = self.board.make_move_new(movement);
                self.analysis_history = vec![self.board];

                if self.board.side_to_move() == Color::White {
                    self.puzzle_status = String::from("White to move!");
                } else {
                    self.puzzle_status = String::from("Black to move!");
                }
                self.puzzle_tab.current_puzzle_side = self.board.side_to_move();
                self.puzzle_tab.is_playing = true;
                self.game_mode = config::GameMode::Puzzle;
                Command::none()
            } (_, Message::GoBackMove) => {
                if self.game_mode == config::GameMode::Analysis && self.analysis_history.len() > self.puzzle_tab.current_puzzle_move {
                    self.analysis_history.pop();
                    self.analysis = Game::new_with_board(*self.analysis_history.last().unwrap());
                }
                Command::none()
            } (_, Message::RedoPuzzle) => {
                self.puzzle_tab.current_puzzle_move = 1;

                let puzzle_moves: Vec<&str> = self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].moves.split_whitespace().collect();

                // The opponent's last move (before the puzzle starts)
                // is in the "moves" field of the cvs, so we need to apply it.
                self.board = Board::from_str(&self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].fen).unwrap();

                let movement = ChessMove::new(
                    Square::from_str(&String::from(&puzzle_moves[0][..2])).unwrap(),
                    Square::from_str(&String::from(&puzzle_moves[0][2..4])).unwrap(), PuzzleTab::check_promotion(puzzle_moves[0]));

                self.last_move_from = Some(PositionGUI::chesssquare_to_posgui(movement.get_source()));
                self.last_move_to = Some(PositionGUI::chesssquare_to_posgui(movement.get_dest()));

                self.board = self.board.make_move_new(movement);
                self.analysis_history = vec![self.board];

                if self.board.side_to_move() == Color::White {
                    self.puzzle_status = String::from("White to move!");
                } else {
                    self.puzzle_status = String::from("Black to move!");
                }
                self.puzzle_tab.current_puzzle_side = self.board.side_to_move();
                self.puzzle_tab.is_playing = true;
                Command::none()
            } (_, Message::LoadPuzzle(puzzles_vec)) => {
                self.from_square = None;
                self.search_tab.show_searching_msg = false;
                if let Some(puzzles_vec) = puzzles_vec {
                    if !puzzles_vec.is_empty() {
                        self.puzzle_tab.puzzles = puzzles_vec;
                        self.puzzle_tab.puzzles.shuffle(&mut thread_rng());
                        self.puzzle_tab.current_puzzle_move = 1;
                        self.puzzle_tab.current_puzzle = 0;

                        self.board = Board::from_str(&self.puzzle_tab.puzzles[0].fen).unwrap();
                        let puzzle_moves: Vec<&str> = self.puzzle_tab.puzzles[0].moves.split_whitespace().collect();

                        // The last opponent's move is in the "moves" field of the cvs,
                        // so we need to apply it.
                        let movement = ChessMove::new(
                                Square::from_str(&puzzle_moves[0][..2]).unwrap(),
                                Square::from_str(&puzzle_moves[0][2..4]).unwrap(), PuzzleTab::check_promotion(puzzle_moves[0]));

                        self.last_move_from = Some(PositionGUI::chesssquare_to_posgui(movement.get_source()));
                        self.last_move_to = Some(PositionGUI::chesssquare_to_posgui(movement.get_dest()));

                        self.board = self.board.make_move_new(movement);
                        self.analysis_history = vec![self.board];

                        if self.board.side_to_move() == Color::White {
                            self.puzzle_status = String::from("White to move!");
                        } else {
                            self.puzzle_status = String::from("Black to move!");
                        }
                        self.puzzle_tab.current_puzzle_side = self.board.side_to_move();
                        self.puzzle_tab.is_playing = true;
                    } else {
                        // Just putting the default position to make it obvious the search ended.
                        self.board = Board::default();
                        self.last_move_from = None;
                        self.last_move_to = None;
                        self.puzzle_tab.is_playing = false;
                        self.puzzle_status = String::from("Sorry, no puzzle found");
                    }
                } else {
                    self.board = Board::default();
                    self.last_move_from = None;
                    self.last_move_to = None;
                    self.puzzle_tab.is_playing = false;
                    self.puzzle_status = String::from("Sorry, no puzzle found");
                }
                Command::none()
            } (_, Message::ChangeSettings(message)) => {
                if let Some(settings) = message {
                    self.settings_tab.saved_configs = settings;
                    self.search_tab.piece_theme_promotion = self.settings_tab.saved_configs.piece_theme;
                    self.search_tab.bg_color_promotion = self.settings_tab.saved_configs.light_squares_color.into();
                    self.search_tab.bg_color_promotion_selected = self.settings_tab.saved_configs.dark_squares_color.into();
                }
                Command::none()
            }
             (_, Message::PuzzleInfo(message)) => {
                self.puzzle_tab.update(message)
            } (_, Message::Search(message)) => {
                self.search_tab.update(message)
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut board_col = Column::new().spacing(0).align_items(Alignment::Center);
        let mut board_row = Row::new().spacing(0).align_items(Alignment::Center);
        let mut i = 0;

        let is_white = self.puzzle_tab.current_puzzle_side == Color::White;

        for _ in 0..64 {

            let rol: i32 = if is_white { 7 - i / 8 } else { i / 8 };
            let col: i32 = if is_white { i % 8 } else { 7 - (i % 8) };

            let pos = PositionGUI::new(rol, col);

            let (piece, color) =
                match self.game_mode {
                    config::GameMode::Analysis => {
                        (self.analysis.current_position().piece_on(pos.posgui_to_square()),
                        self.analysis.current_position().color_on(pos.posgui_to_square()))
                    } config::GameMode::Puzzle => {
                        (self.board.piece_on(pos.posgui_to_square()),
                        self.board.color_on(pos.posgui_to_square()))
                    }
                };

            let mut text = "";
            if let Some(piece) = piece {
                if color.unwrap() == Color::White {
                    text = match piece {
                        Piece::Pawn => "/wP.svg",
                        Piece::Rook => "/wR.svg",
                        Piece::Knight => "/wN.svg",
                        Piece::Bishop => "/wB.svg",
                        Piece::Queen => "/wQ.svg",
                        Piece::King => "/wK.svg"
                    };
                } else {
                    text = match piece {
                        Piece::Pawn => "/bP.svg",
                        Piece::Rook => "/bR.svg",
                        Piece::Knight => "/bN.svg",
                        Piece::Bishop => "/bB.svg",
                        Piece::Queen => "/bQ.svg",
                        Piece::King => "/bK.svg"
                    };
                }
            }

            let selected =
                if self.game_mode == config::GameMode::Puzzle {
                    self.from_square == Some(pos)    ||
                    self.last_move_from == Some(pos) ||
                    self.last_move_to == Some(pos)   ||
                    self.hint_square == Some(pos)
                } else {
                    self.from_square == Some(pos)
                };

            board_row = board_row.push(Button::new(
                    Svg::from_path(
                        String::from("pieces/") + &self.settings_tab.saved_configs.piece_theme.to_string() + text)
                )
                .width(Length::Units(config::SETTINGS.square_size))
                .height(Length::Units(config::SETTINGS.square_size))
                .on_press(Message::SelectSquare(pos))
                .style(ChessSquare::from((pos, selected, self.settings_tab.saved_configs.light_squares_color, self.settings_tab.saved_configs.dark_squares_color)))
            );

            i += 1;
            if i % 8 == 0 {
                board_col = board_col.push(board_row);
                board_row = Row::new().spacing(0).align_items(Alignment::Center);
            }
        }

        let game_mode_row = Row::new().spacing(10).padding(10).align_items(Alignment::Center)
            .push(Text::new("Mode:")
                .width(Length::Shrink)
                .horizontal_alignment(alignment::Horizontal::Center))
            .push(
                Radio::new(config::GameMode::Puzzle, "Puzzle", Some(self.game_mode), Message::SelectMode))
            .push(
                Radio::new(config::GameMode::Analysis, "Analysis", Some(self.game_mode), Message::SelectMode));

        let mut status_col = Column::new().padding(3).align_items(Alignment::Center);

        let mut row_result = Row::new().spacing(0).align_items(Alignment::Center);
        row_result = row_result.push(Text::new(&self.puzzle_status)
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center));

        status_col = status_col.push(row_result);

        let mut navigation_row = Row::new().padding(3).spacing(50);
        let has_puzzles = !self.puzzle_tab.puzzles.is_empty() && self.puzzle_tab.current_puzzle < self.puzzle_tab.puzzles.len() - 1;
        if has_puzzles {
            navigation_row = navigation_row.push(
                    Button::new(Text::new("Next puzzle")).on_press(Message::ShowNextPuzzle));
        } else {
            navigation_row = navigation_row.push(Button::new(Text::new("Next puzzle")));
        }
        if self.game_mode == config::GameMode::Analysis {
            if self.analysis_history.len() > self.puzzle_tab.current_puzzle_move {
                navigation_row = navigation_row.push(
                    Button::new(Text::new("Takeback move")).on_press(Message::GoBackMove));
            } else {
                navigation_row = navigation_row.push(
                    Button::new(Text::new("Takeback move")));
            }
        } else if has_puzzles && !self.puzzle_tab.is_playing {
            navigation_row = navigation_row.push(
                Button::new(Text::new("Redo Puzzle")).on_press(Message::RedoPuzzle));
        }

        board_col = board_col.push(status_col).push(game_mode_row).push(navigation_row);
        let mut layout_row = Row::new().spacing(30).align_items(Alignment::Start);
        layout_row = layout_row.push(board_col);

        let tab_theme = match self.settings_tab.saved_configs.board_theme {
            styles::BoardStyle::Grey => styles::TabTheme::Grey,
            styles::BoardStyle::Blue => styles::TabTheme::Blue,
            styles::BoardStyle::Green => styles::TabTheme::Green,
            styles::BoardStyle::Purple => styles::TabTheme::Purple,
            _ => styles::TabTheme::Brown,
        };

        let tabs = Tabs::new(self.active_tab, Message::TabSelected)
                .push(self.search_tab.tab_label(), self.search_tab.view())
                .push(self.settings_tab.tab_label(), self.settings_tab.view())
                .push(self.puzzle_tab.tab_label(), self.puzzle_tab.view())
                .tab_bar_position(iced_aw::TabBarPosition::Top)
                .tab_bar_style(tab_theme);

        layout_row = layout_row.push(tabs);
        Container::new(layout_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}

fn main() -> iced::Result {
    OfflinePuzzles::run(Settings {
        window: iced::window::Settings {
            size: (
                (config::SETTINGS.square_size * 8) as u32 + 450,
                (config::SETTINGS.square_size * 8) as u32 + 117,
            ),
            resizable: true,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}
