#![windows_subsystem = "windows"]

use eval::{Engine, EngineStatus};
use std::io::BufReader;
use std::path::Path;
use std::fs::File as StdFile;
use std::str::FromStr;
use tokio::sync::mpsc::{self, Sender};
use iced::widget::{Svg, Container, Button, row, Row, Column, Text, Radio};
use iced::{Application, Element, Size, Subscription};
use iced::{executor, alignment, Command, Alignment, Length, Settings };
use iced::window;
use iced_lazy::responsive;
use iced_native::{Event};

use iced_aw::{TabLabel, Tabs};
use chess::{Board, BoardStatus, ChessMove, Color, Piece, Rank, Square, File, Game};

use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{Source, Buffered};

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

mod eval;

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
    ShowHint,
    ShowNextPuzzle,
    GoBackMove,
    RedoPuzzle,
    LoadPuzzle(Option<Vec<config::Puzzle>>),
    ChangeSettings(Option<config::OfflinePuzzlesConfig>),
    EventOccurred(iced_native::Event),
    StartEngine,
    EngineStopped(bool),
    UpdateEval((Option<String>, Option<String>)),
    EngineReady(mpsc::Sender<String>),
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
    engine_state: EngineStatus,
    engine_btn_label: String,
    engine_eval: String,
    engine: Engine,
    engine_sender: Option<Sender<String>>,
    engine_move: String,

    active_tab: usize,
    search_tab: SearchTab,
    settings_tab: SettingsTab,
    puzzle_tab: PuzzleTab,
    game_mode: config::GameMode,
    two_pieces_sound: Option<Buffered<Decoder<BufReader<StdFile>>>>,
    one_piece_sound: Option<Buffered<Decoder<BufReader<StdFile>>>>
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
            engine_state: EngineStatus::TurnedOff,
            engine_btn_label: String::from("Start Engine"),
            engine_eval: String::new(),
            engine: Engine::new(
                config::SETTINGS.engine_path.clone(),
                config::SETTINGS.engine_limit.clone(),
                String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            ),
            engine_sender: None,
            engine_move: String::new(),

            puzzle_status: String::from("Use the search."),
            search_tab: SearchTab::new(),
            settings_tab: SettingsTab::new(),
            puzzle_tab: PuzzleTab::new(),
            active_tab: 0,

            game_mode: config::GameMode::Puzzle,
            two_pieces_sound: load_two_pieces_sound(),
            one_piece_sound: load_one_piece_sound(),
        }
    }
}

fn load_two_pieces_sound() -> Option<Buffered<Decoder<BufReader<StdFile>>>> {
    let two_pieces_sound = BufReader::new(StdFile::open("2pieces.ogg").unwrap());
    match Decoder::new(two_pieces_sound) {
        Err(_) => None,
        Ok(dec) => Some(dec.buffered())
    }
}

fn load_one_piece_sound() -> Option<Buffered<Decoder<BufReader<StdFile>>>> {
    let one_pieces_sound = BufReader::new(StdFile::open("1piece.ogg").unwrap());
    match Decoder::new(one_pieces_sound) {
        Err(_) => None,
        Ok(dec) => Some(dec.buffered())
    }
}

// The chess crate has a bug on how it returns the en passant square
// https://github.com/jordanbray/chess/issues/36
// For communication with the engine we need to pass the correct value,
// so this ugly solution is needed.
fn san_correct_ep(fen: String) -> String {
    let mut tokens_vec: Vec<&str> = fen.split_whitespace().collect::<Vec<&str>>();
    let mut new_ep_square = String::from("-");
    if let Some(en_passant) = tokens_vec.get(3) {
        if en_passant != &"-" {
            let rank = if String::from(&en_passant[1..2]).parse::<usize>().unwrap() == 4 {
                3
            } else {
                6
            };
            new_ep_square = String::from(&en_passant[0..1]) + &rank.to_string();
        }
    }
    tokens_vec[3] = &new_ep_square;
    tokens_vec.join(" ")
}

fn coord_to_san(board: Board, coords: String) -> Option<String> {
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
        if coords == "e1g1" || coords == "e8g8" {
            san = Some(String::from("0-0"));
        } else if coords == "e1c1" || coords == "e8c8" {
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
    type Theme = styles::Theme;
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
                        self.engine.position = self.analysis.current_position().to_string();
                        if let Some(sender) = &self.engine_sender {
                            if let Err(e) = sender.blocking_send(san_correct_ep(self.analysis.current_position().to_string())) {
                                eprintln!("Lost contact with the engine: {}", e);
                            }
                        }
                        if self.settings_tab.saved_configs.play_sound {
                            if let Some(audio) = self.one_piece_sound.clone() {
                                std::thread::spawn(move || {
                                    if let Ok((_stream, handle)) = OutputStream::try_default() {
                                        if let Ok(sink) = Sink::try_new(&handle) {
                                            sink.append(audio);
                                            sink.sleep_until_end();
                                        }
                                    }
                                });
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
                                if let Some(audio) = self.one_piece_sound.clone() {
                                    std::thread::spawn(move || {
                                        if let Ok((_stream, handle)) = OutputStream::try_default() {
                                            if let Ok(sink) = Sink::try_new(&handle) {
                                                sink.append(audio);
                                                sink.sleep_until_end();
                                            }
                                        }
                                    });
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
                                if let Some(audio) = self.two_pieces_sound.clone() {
                                    std::thread::spawn(move || {
                                        if let Ok((_stream, handle)) = OutputStream::try_default() {
                                            if let Ok(sink) = Sink::try_new(&handle) {
                                                sink.append(audio);
                                                sink.sleep_until_end();
                                            }
                                        }
                                    });
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
            } (_, Message::ShowHint) => {
                let moves = self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].moves.split_whitespace().collect::<Vec<&str>>();
                if !moves.is_empty() && moves.len() > self.puzzle_tab.current_puzzle_move {
                    self.hint_square = Some(PositionGUI::chesssquare_to_posgui(Square::from_str(&moves[self.puzzle_tab.current_puzzle_move][..2]).unwrap()));
                } else {
                    self.hint_square = None;
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
                    if let Some(sender) = &self.engine_sender {
                        if let Err(e) = sender.blocking_send(san_correct_ep(self.analysis.current_position().to_string())) {
                            eprintln!("Lost contact with the engine: {}", e);
                        }
                    }
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
                self.game_mode = config::GameMode::Puzzle;
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
                    self.engine.engine_path = self.settings_tab.engine_path.clone();
                }
                Command::none()
            }
             (_, Message::PuzzleInfo(message)) => {
                self.puzzle_tab.update(message)
            } (_, Message::Search(message)) => {
                self.search_tab.update(message)
            } (_, Message::EventOccurred(event)) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    match self.engine_state {
                        EngineStatus::TurnedOff => {
                            SettingsTab::save_window_size(self.settings_tab.window_width, self.settings_tab.window_height);
                            window::close()        
                        } _ => {
                            if let Some(sender) = &self.engine_sender {
                                sender.blocking_send(String::from(eval::EXIT_APP_COMMAND)).expect("Error stopping engine.");
                            }
                            Command::none()
                        }
                    }                        
                } else if let Event::Window(window::Event::Resized { width, height }) = event {
                    self.settings_tab.window_width = width;
                    self.settings_tab.window_height = height;
                    Command::none()
                } else {
                    Command::none()
                }
            } (_, Message::StartEngine) => {
                match self.engine_state {
                    EngineStatus::TurnedOff => {
                        //Check if the path is correct first
                        if Path::new(&self.engine.engine_path).exists() {
                            self.engine.position = san_correct_ep(self.analysis.current_position().to_string());
                            self.engine_state = EngineStatus::Started;
                            self.engine_btn_label = String::from("Stop Engine");
                        }
                    } _ => {
                        if let Some(sender) = &self.engine_sender {
                            sender.blocking_send(String::from(eval::STOP_COMMAND)).expect("Error stopping engine.");
                            drop(sender);
                            self.engine_sender = None;
                        }
                    }
                }
                Command::none()
            } (_, Message::EngineStopped(exit)) => {
                self.engine_state = EngineStatus::TurnedOff;
                if exit {
                    SettingsTab::save_window_size(self.settings_tab.window_width, self.settings_tab.window_height);
                    window::close()
                } else {
                    self.engine_eval = String::new();
                    self.engine_move = String::new();
                    self.engine_btn_label = String::from("Start Engine");
                    Command::none()
                }
            } (_, Message::EngineReady(sender)) => {
                self.engine_sender = Some(sender);
                Command::none()
            } (_, Message::UpdateEval(eval)) => {
                match self.engine_state {
                    EngineStatus::TurnedOff => {
                        Command::none()
                    } _ => {
                        let (eval, best_move) = eval;                
                        if let Some(eval_str) = eval {
                            //Keep the values relative to white, like it's usually done in GUIs
                            if !eval_str.contains("Mate") && self.analysis.side_to_move() != Color::White {
                                let eval = (eval_str.parse::<f32>().unwrap() * -1.).to_string();
                                self.engine_eval = eval.to_string().clone();    
                            } else if eval_str.contains("Mate") && self.analysis.side_to_move() != Color::White {
                                let tokens: Vec<&str> = eval_str.split_whitespace().collect();
                                let distance_to_mate = (tokens[2].parse::<f32>().unwrap() * -1.).to_string();
                                self.engine_eval = String::from("Mate in ") + &distance_to_mate;
                            } else {
                                self.engine_eval = eval_str;
                            }
                        }
                        if let Some(best_move) = best_move {
                            if let Some(best_move) = coord_to_san(self.analysis.current_position(), best_move) {
                                self.engine_move = best_move;
                            }
                        }
                        Command::none()
                    }
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.engine_state {
            EngineStatus::TurnedOff => {
                iced_native::subscription::events().map(Message::EventOccurred)
            } _ => {
                Subscription::batch(vec![
                    self.engine.run(),
                    iced_native::subscription::events().map(Message::EventOccurred)
                ])
            }
        }
    }

    fn view(&self) -> Element<Message, iced::Renderer<styles::Theme>> {
        let resp = responsive(move |size| {
            gen_view(
                self.game_mode,
                self.puzzle_tab.current_puzzle_side,
                self.settings_tab.flip_board,
                self.board.clone(),
                self.analysis.current_position().clone(),
                self.from_square,
                self.last_move_from,
                self.last_move_to,
                self.hint_square,
                self.settings_tab.saved_configs.piece_theme,
                self.puzzle_status.clone(),
                //has_puzzles:
                !self.puzzle_tab.puzzles.is_empty() && self.puzzle_tab.current_puzzle < self.puzzle_tab.puzzles.len() - 1,
                self.analysis_history.len(),
                self.puzzle_tab.current_puzzle_move,
                !self.puzzle_tab.is_playing,
                self.active_tab,
                self.engine_eval.clone(),
                self.engine_move.clone(),

                self.engine_btn_label.clone(),
                self.search_tab.tab_label(),
                self.settings_tab.tab_label(),
                self.puzzle_tab.tab_label(),
                self.search_tab.view(),
                self.settings_tab.view(),
                self.puzzle_tab.view(),
                size,
            )});
        Container::new(resp)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        self.settings_tab.board_theme
    }
}

fn gen_view<'a>(
    game_mode: config::GameMode,
    current_puzzle_side: Color,
    flip_board: bool,
    board: Board,
    analysis: Board,
    from_square: Option<PositionGUI>,
    last_move_from: Option<PositionGUI>,
    last_move_to: Option<PositionGUI>,
    hint_square: Option<PositionGUI>,
    piece_theme: styles::PieceTheme,
    puzzle_status: String,
    has_puzzles: bool,
    analysis_history_len: usize,
    current_puzzle_move: usize,
    is_playing: bool,
    active_tab: usize,
    engine_eval: String,
    engine_move: String,

    engine_btn_label: String,
    search_tab_label: TabLabel,
    settings_tab_label: TabLabel,
    puzzle_tab_label: TabLabel,
    search_tab: Element<'a, Message, iced::Renderer<styles::Theme>>,
    settings_tab: Element<'a, Message, iced::Renderer<styles::Theme>>,
    puzzle_tab: Element<'a, Message, iced::Renderer<styles::Theme>>,
    size: Size
) -> Element<'a, Message, iced::Renderer<styles::Theme>> {

    let mut board_col = Column::new().spacing(0).align_items(Alignment::Center);
    let mut board_row = Row::new().spacing(0).align_items(Alignment::Center);
    let mut i = 0;

    let is_white = (current_puzzle_side == Color::White) ^ flip_board;

    for _ in 0..64 {

        let rol: i32 = if is_white { 7 - i / 8 } else { i / 8 };
        let col: i32 = if is_white { i % 8 } else { 7 - (i % 8) };

        let pos = PositionGUI::new(rol, col);

        let (piece, color) =
            match game_mode {
                config::GameMode::Analysis => {
                    (analysis.piece_on(pos.posgui_to_square()),
                    analysis.color_on(pos.posgui_to_square()))
                } config::GameMode::Puzzle => {
                    (board.piece_on(pos.posgui_to_square()),
                    board.color_on(pos.posgui_to_square()))
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
            if game_mode == config::GameMode::Puzzle {
                from_square == Some(pos)    ||
                last_move_from == Some(pos) ||
                last_move_to == Some(pos)   ||
                hint_square == Some(pos)
            } else {
                from_square == Some(pos)
            };

        let square_style :styles::ButtonStyle = if (pos.get_row() * 9 + pos.get_col()) % 2 == 1 {
            if selected {
                styles::ButtonStyle::SelectedLightSquare
            } else {   
                styles::ButtonStyle::LightSquare
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if selected {
                styles::ButtonStyle::SelectedDarkSquare
            } else {
                styles::ButtonStyle::DarkSquare
            }
        };

        //Reserve more space below the board if we'll show the engine eval
        let board_height = if engine_eval.is_empty() {
            ((size.height - 110.) / 8.) as u16
        } else {
            ((size.height - 140.) / 8.) as u16
        };

        board_row = board_row.push(Button::new(
                Svg::from_path(
                    String::from("pieces/") + &piece_theme.to_string() + text)
            )
            .width(board_height)
            .height(board_height)
            .on_press(Message::SelectSquare(pos))
            .style(square_style)
        );

        i += 1;
        if i % 8 == 0 {
            board_col = board_col.push(board_row);
            board_row = Row::new().spacing(0).align_items(Alignment::Center);
        }
    }

    let game_mode_row = row![
        Text::new("Mode:"),
        Radio::new(config::GameMode::Puzzle, "Puzzle", Some(game_mode), Message::SelectMode),
        Radio::new(config::GameMode::Analysis, "Analysis", Some(game_mode), Message::SelectMode)
    ].spacing(10).padding(10).align_items(Alignment::Center);

    let mut navigation_row = Row::new().padding(3).spacing(50);
    if has_puzzles {
        navigation_row = navigation_row.push(
                Button::new(Text::new("Next puzzle")).on_press(Message::ShowNextPuzzle));
    } else {
        navigation_row = navigation_row.push(Button::new(Text::new("Next puzzle")));
    }
    if game_mode == config::GameMode::Analysis {
        if analysis_history_len > current_puzzle_move {
            navigation_row = navigation_row.push(
                Button::new(Text::new("Takeback move")).on_press(Message::GoBackMove));
        } else {
            navigation_row = navigation_row.push(
                Button::new(Text::new("Takeback move")));
        }
        navigation_row = navigation_row
            .push(Button::new(Text::new(engine_btn_label)).on_press(Message::StartEngine));
    } else if has_puzzles && !is_playing {
        navigation_row = navigation_row
            .push(Button::new(Text::new("Redo Puzzle")).on_press(Message::RedoPuzzle))
            .push(Button::new(Text::new("Hint")).on_press(Message::ShowHint));
    }

    board_col = board_col.push(Text::new(puzzle_status)).push(game_mode_row).push(navigation_row);
    if !engine_eval.is_empty() {
        board_col = board_col.push(
            row![
                Text::new(String::from("Eval: ") + &engine_eval),
                Text::new(String::from("Best move: ") + &engine_move)
            ].padding(5).spacing(15)
        );
    }

    let tabs = Tabs::new(active_tab, Message::TabSelected)
            .push(search_tab_label, search_tab)
            .push(settings_tab_label, settings_tab)
            .push(puzzle_tab_label, puzzle_tab)
            .tab_bar_position(iced_aw::TabBarPosition::Top);    

    row![board_col,tabs].spacing(30).align_items(Alignment::Start).into()
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<Message, iced::Renderer<styles::Theme>> {
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

    fn content(&self) -> Element<Message, iced::Renderer<styles::Theme>>;
}

fn main() -> iced::Result {
    OfflinePuzzles::run(Settings {
        window: iced::window::Settings {
            size: (
                config::SETTINGS.window_width, //(config::SETTINGS.square_size * 8) as u32 + 450,
                config::SETTINGS.window_height,//(config::SETTINGS.square_size * 8) as u32 + 120,
            ),
            resizable: true,
            ..iced::window::Settings::default()
        },
        exit_on_close_request:false,
        ..Settings::default()
    })
}
