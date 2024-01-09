#![windows_subsystem = "windows"]

use eval::{Engine, EngineStatus};
use iced::widget::text::LineHeight;
use styles::PieceTheme;
use std::io::BufReader;
use std::path::Path;
use std::fs::File as StdFile;
use std::str::FromStr;
use tokio::sync::mpsc::{self, Sender};
use iced::widget::{Svg, Container, Button, row, Row, Column, Text, Radio, responsive};
use iced::{Application, Element, Size, Subscription};
use iced::{executor, alignment, Command, Alignment, Length, Settings };
use iced::window;
use iced::Event;
use std::borrow::Cow;

use iced_aw::{TabLabel, Tabs};
use chess::{Board, BoardStatus, ChessMove, Color, Piece, Rank, Square, File, Game};

use rodio::{Decoder, OutputStream, OutputStreamHandle};
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
use puzzles::{PuzzleMessage, PuzzleTab, GameStatus};

mod eval;
mod export;
mod lang;
mod openings;

pub mod models;
pub mod schema;
mod db;

#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositionGUI {
    row: i32,
    col: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TabId {
    Search,
    Settings,
    CurrentPuzzle,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChessFontLoaded(Result<(), iced::font::Error>),
    SelectSquare(Square),
    Search(SearchMesssage),
    Settings(SettingsMessage),
    PuzzleInfo(PuzzleMessage),
    SelectMode(config::GameMode),
    TabSelected(TabId),
    ShowHint,
    ShowNextPuzzle,
    ShowPreviousPuzzle,
    GoBackMove,
    RedoPuzzle,
    ExportPDF(bool),
    LoadPuzzle(Option<Vec<config::Puzzle>>),
    ChangeSettings(Option<config::OfflinePuzzlesConfig>),
    EventOccurred(iced::Event),
    StartEngine,
    EngineStopped(bool),
    UpdateEval((Option<String>, Option<String>)),
    EngineReady(mpsc::Sender<String>),
    FavoritePuzzle,
    MinimizeUI,
}

struct SoundPlayback {
    // it's not directly used, but we need to keep it: https://github.com/RustAudio/rodio/issues/330
    stream: OutputStream,
    handle: OutputStreamHandle,
    one_piece_sound: Buffered<Decoder<BufReader<StdFile>>>,
    two_pieces_sound: Buffered<Decoder<BufReader<StdFile>>>,
}

impl SoundPlayback {
    pub const ONE_PIECE_SOUND: u8 = 0;
    pub const TWO_PIECE_SOUND: u8 = 1;
    pub fn init_sound() -> Option<Self> {
        let mut sound_playback = None;
        if let Ok((stream, handle)) = OutputStream::try_default() {
            let one_pieces_sound = StdFile::open("1piece.ogg");
            let two_pieces_sound = StdFile::open("2pieces.ogg");

            if let (Ok(one_piece), Ok(two_piece)) = (one_pieces_sound, two_pieces_sound) {
                sound_playback = Some(
                    SoundPlayback {
                        stream: stream,
                        handle: handle,
                        one_piece_sound: Decoder::new(BufReader::new(one_piece)).unwrap().buffered(),
                        two_pieces_sound: Decoder::new(BufReader::new(two_piece)).unwrap().buffered()
                    }
                );
            }
        }
        sound_playback
    }
    pub fn play_audio(&self, audio: u8) {
        let audio = match audio {
            SoundPlayback::ONE_PIECE_SOUND => self.one_piece_sound.clone(),
            _ => self.two_pieces_sound.clone(),
        };
        if let Err(e) = self.handle.play_raw(audio.convert_samples()) {
            eprintln!("{e}");
        }
    }
}

//#[derive(Clone)]
struct OfflinePuzzles {
    from_square: Option<Square>,
    board: Board,
    last_move_from: Option<Square>,
    last_move_to: Option<Square>,
    hint_square: Option<Square>,
    puzzle_status: String,

    analysis: Game,
    analysis_history: Vec<Board>,
    engine_state: EngineStatus,
    engine_eval: String,
    engine: Engine,
    engine_sender: Option<Sender<String>>,
    engine_move: String,

    active_tab: TabId,
    search_tab: SearchTab,
    settings_tab: SettingsTab,
    puzzle_tab: PuzzleTab,
    game_mode: config::GameMode,
    sound_playback: Option<SoundPlayback>,
    lang: lang::Language,
    mini_ui: bool,
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
            engine_eval: String::new(),
            engine: Engine::new(
                config::SETTINGS.engine_path.clone(),
                config::SETTINGS.engine_limit.clone(),
                String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            ),
            engine_sender: None,
            engine_move: String::new(),

            puzzle_status: String::from(lang::tr(&config::SETTINGS.lang, "use_search")),
            search_tab: SearchTab::new(),
            settings_tab: SettingsTab::new(),
            puzzle_tab: PuzzleTab::new(),
            active_tab: TabId::Search,

            game_mode: config::GameMode::Puzzle,
            sound_playback: SoundPlayback::init_sound(),
            lang: config::SETTINGS.lang,
            mini_ui: false,
        }
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

fn get_notation_string(board: Board, promo_piece: Piece, from: Square, to: Square) -> String {

    let mut move_made_notation = from.to_string() + &to.to_string();
    let piece = board.piece_on(from);
    let color = board.color_on(from);

    // Check for promotion and adjust the notation accordingly
    if let (Some(piece), Some(color)) = (piece, color) {
        if piece == Piece::Pawn && ((color == Color::White && to.get_rank() == Rank::Eighth) ||
                                   (color == Color::Black && to.get_rank() == Rank::First)) {
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
            iced::font::load(Cow::from(config::CHESS_ALPHA_BYTES)).map(Message::ChessFontLoaded),
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
                        config::GameMode::Analysis => { self.analysis.current_position().color_on(pos) }
                        config::GameMode::Puzzle => { self.board.color_on(pos) }
                    };

                if (self.puzzle_tab.is_playing() || self.game_mode == config::GameMode::Analysis) && color == Some(side) {
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
                        config::GameMode::Analysis => { self.analysis.current_position().color_on(to) }
                        config::GameMode::Puzzle => { self.board.color_on(to) }
                    };
                // If the user clicked on another piece of his own side,
                // just replace the previous selection and exit
                if self.puzzle_tab.is_playing() && color == Some(side) {
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
                            if let Some(audio) = &self.sound_playback {
                                audio.play_audio(SoundPlayback::ONE_PIECE_SOUND);
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
                                if let Some(audio) = &self.sound_playback {
                                    audio.play_audio(SoundPlayback::ONE_PIECE_SOUND);
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

                                    self.last_move_from = Some(movement.get_source());
                                    self.last_move_to = Some(movement.get_dest());

                                    self.board = self.board.make_move_new(movement);
                                    self.analysis_history = vec![self.board];

                                    if self.board.side_to_move() == Color::White {
                                        self.puzzle_status = lang::tr(&self.lang, "white_to_move");
                                    } else {
                                        self.puzzle_status = lang::tr(&self.lang, "black_to_move");
                                    }

                                    self.puzzle_tab.current_puzzle_side = self.board.side_to_move();
                                    self.puzzle_tab.current_puzzle_fen = san_correct_ep(self.board.to_string());
                                } else {
                                    self.puzzle_tab.game_status = GameStatus::PuzzleEnded;
                                    self.puzzle_status = lang::tr(&self.lang, "correct_puzzle");
                                }
                            } else {
                                if self.settings_tab.saved_configs.auto_load_next {
                                    self.board = Board::default();
                                    // quite meaningless but allows the user to use the takeback button
                                    // to analyze a full game in analysis mode after the puzzles ended.
                                    self.analysis_history = vec![self.board];
                                    self.puzzle_tab.current_puzzle_move = 1;
                                    self.puzzle_tab.game_status = GameStatus::NoPuzzles;
                                } else {
                                    self.puzzle_tab.game_status = GameStatus::PuzzleEnded;
                                }
                                self.last_move_from = None;
                                self.last_move_to = None;
                                self.puzzle_status = lang::tr(&self.lang, "all_puzzles_done");
                            }
                        } else {
                            if self.settings_tab.saved_configs.play_sound {
                                if let Some(audio) = &self.sound_playback {
                                    audio.play_audio(SoundPlayback::TWO_PIECE_SOUND);
                                }
                            }
                            movement = ChessMove::new(
                                Square::from_str(&String::from(&correct_moves[self.puzzle_tab.current_puzzle_move][..2])).unwrap(),
                                Square::from_str(&String::from(&correct_moves[self.puzzle_tab.current_puzzle_move][2..4])).unwrap(), PuzzleTab::check_promotion(correct_moves[self.puzzle_tab.current_puzzle_move]));

                            self.last_move_from = Some(movement.get_source());
                            self.last_move_to = Some(movement.get_dest());

                            self.board = self.board.make_move_new(movement);
                            self.analysis_history.push(self.board);

                            self.puzzle_tab.current_puzzle_move += 1;
                            self.puzzle_status = lang::tr(&self.lang, "correct_move");
                        }
                    } else {
                        #[allow(clippy::collapsible_else_if)]
                        if self.board.side_to_move() == Color::White {
                            self.puzzle_status = lang::tr(&self.lang, "wrong_move_white_play");
                        } else {
                            self.puzzle_status = lang::tr(&self.lang, "wrong_move_black_play");
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
                    if self.engine_state != EngineStatus::TurnedOff {
                        if let Some(sender) = &self.engine_sender {
                            sender.blocking_send(String::from(eval::STOP_COMMAND)).expect("Error stopping engine.");
                        }
                    }
                    self.analysis_history.truncate(self.puzzle_tab.current_puzzle_move);
                }
                Command::none()
            } (_, Message::ShowHint) => {
                let moves = self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].moves.split_whitespace().collect::<Vec<&str>>();
                if !moves.is_empty() && moves.len() > self.puzzle_tab.current_puzzle_move {
                    self.hint_square = Some(Square::from_str(&moves[self.puzzle_tab.current_puzzle_move][..2]).unwrap());
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

                self.last_move_from = Some(movement.get_source());
                self.last_move_to = Some(movement.get_dest());

                self.board = self.board.make_move_new(movement);
                self.analysis_history = vec![self.board];

                if self.board.side_to_move() == Color::White {
                    self.puzzle_status = lang::tr(&self.lang, "white_to_move");
                } else {
                    self.puzzle_status = lang::tr(&self.lang, "black_to_move");
                }

                self.puzzle_tab.current_puzzle_fen = san_correct_ep(self.board.to_string());
                self.puzzle_tab.current_puzzle_side = self.board.side_to_move();
                self.puzzle_tab.game_status = GameStatus::Playing;
                self.game_mode = config::GameMode::Puzzle;
                Command::none()
            } (_, Message::ShowPreviousPuzzle) => {
                if self.puzzle_tab.current_puzzle > 0 && self.game_mode == config::GameMode::Puzzle {
                    self.puzzle_tab.current_puzzle -= 1;
                    self.puzzle_tab.current_puzzle_move = 1;

                    let puzzle_moves: Vec<&str> = self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].moves.split_whitespace().collect();

                    // The opponent's last move (before the puzzle starts)
                    // is in the "moves" field of the cvs, so we need to apply it.
                    self.board = Board::from_str(&self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].fen).unwrap();

                    let movement = ChessMove::new(
                        Square::from_str(&String::from(&puzzle_moves[0][..2])).unwrap(),
                        Square::from_str(&String::from(&puzzle_moves[0][2..4])).unwrap(), PuzzleTab::check_promotion(puzzle_moves[0]));

                    self.last_move_from = Some(movement.get_source());
                    self.last_move_to = Some(movement.get_dest());

                    self.board = self.board.make_move_new(movement);
                    self.analysis_history = vec![self.board];

                    if self.board.side_to_move() == Color::White {
                        self.puzzle_status = lang::tr(&self.lang, "white_to_move");
                    } else {
                        self.puzzle_status = lang::tr(&self.lang, "black_to_move");
                    }

                    self.puzzle_tab.current_puzzle_fen = san_correct_ep(self.board.to_string());
                    self.puzzle_tab.current_puzzle_side = self.board.side_to_move();
                    self.puzzle_tab.game_status = GameStatus::Playing;
                }
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

                self.last_move_from = Some(movement.get_source());
                self.last_move_to = Some(movement.get_dest());

                self.board = self.board.make_move_new(movement);
                self.analysis_history = vec![self.board];

                if self.board.side_to_move() == Color::White {
                    self.puzzle_status = lang::tr(&self.lang, "white_to_move");
                } else {
                    self.puzzle_status = lang::tr(&self.lang, "black_to_move");
                }
                self.puzzle_tab.current_puzzle_side = self.board.side_to_move();
                self.puzzle_tab.game_status = GameStatus::Playing;
                Command::none()
            } (_, Message::LoadPuzzle(puzzles_vec)) => {
                self.from_square = None;
                self.search_tab.show_searching_msg = false;
                self.game_mode = config::GameMode::Puzzle;
                if self.engine_state != EngineStatus::TurnedOff {
                    if let Some(sender) = &self.engine_sender {
                        sender.blocking_send(String::from(eval::STOP_COMMAND)).expect("Error stopping engine.");
                    }
                }
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

                        self.last_move_from = Some(movement.get_source());
                        self.last_move_to = Some(movement.get_dest());

                        self.board = self.board.make_move_new(movement);
                        self.analysis_history = vec![self.board];

                        if self.board.side_to_move() == Color::White {
                            self.puzzle_status = lang::tr(&self.lang, "white_to_move");
                        } else {
                            self.puzzle_status = lang::tr(&self.lang, "black_to_move");
                        }
                        self.puzzle_tab.current_puzzle_fen = san_correct_ep(self.board.to_string());
                        self.puzzle_tab.current_puzzle_side = self.board.side_to_move();
                        self.puzzle_tab.game_status = GameStatus::Playing;
                    } else {
                        // Just putting the default position to make it obvious the search ended.
                        self.board = Board::default();
                        self.last_move_from = None;
                        self.last_move_to = None;
                        self.puzzle_tab.game_status = GameStatus::NoPuzzles;
                        self.puzzle_status = lang::tr(&self.lang, "no_puzzle_found");
                    }
                } else {
                    self.board = Board::default();
                    self.last_move_from = None;
                    self.last_move_to = None;
                    self.puzzle_tab.game_status = GameStatus::NoPuzzles;
                    self.puzzle_status = lang::tr(&self.lang, "no_puzzle_found");
                }
                Command::none()
            } (_, Message::ChangeSettings(message)) => {
                if let Some(settings) = message {
                    self.search_tab.piece_theme_promotion = self.settings_tab.piece_theme;
                    self.engine.engine_path = self.settings_tab.engine_path.clone();
                    self.lang = settings.lang;
                    self.search_tab.lang = self.lang;
                    self.search_tab.theme.lang = self.lang;
                    self.search_tab.opening.lang = self.lang;
                    self.puzzle_tab.lang = self.lang;
                    self.settings_tab.saved_configs = settings;
                }
                Command::none()
            }
             (_, Message::PuzzleInfo(message)) => {
                self.puzzle_tab.update(message)
            } (_, Message::Search(message)) => {
                self.search_tab.update(message)
            } (_, Message::ExportPDF(_)) => {
                export::to_pdf(&self.puzzle_tab.puzzles, self.settings_tab.export_pgs.parse::<i32>().unwrap(), &self.lang);
                Command::none()
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
                    if !self.mini_ui {
                        self.settings_tab.window_width = width;
                        self.settings_tab.window_height = height;
                    }
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
                        }
                    } _ => {
                        if let Some(sender) = &self.engine_sender {
                            sender.blocking_send(String::from(eval::STOP_COMMAND)).expect("Error stopping engine.");
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
                            if eval_str.contains("Mate") {
                                let tokens: Vec<&str> = eval_str.split_whitespace().collect();
                                let distance_to_mate_num = tokens[2].parse::<i32>().unwrap();
                                self.engine_eval = if distance_to_mate_num < 0 {
                                    lang::tr(&self.lang, "mate_in") + &(distance_to_mate_num * -1).to_string()
                                } else if distance_to_mate_num > 0 {
                                    lang::tr(&self.lang, "mate_in") + &distance_to_mate_num.to_string()
                                } else {
                                    lang::tr(&self.lang, "mate")
                                };
                            } else if self.analysis.side_to_move() == Color::White {
                                self.engine_eval = eval_str;
                            } else {
                                // Invert to keep the values relative to white,
                                // like it's usually done in GUIs.
                                let eval = (eval_str.parse::<f32>().unwrap() * -1.).to_string();
                                self.engine_eval = eval.to_string().clone();
                            }
                        }
                        if let Some(best_move) = best_move {
                            if let Some(best_move) = config::coord_to_san(&self.analysis.current_position(), best_move) {
                                self.engine_move = best_move;
                            }
                        }
                        Command::none()
                    }
                }
            } (_, Message::FavoritePuzzle) => {
                db::toggle_favorite(self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].clone());
                Command::none()
            } (_, Message::ChessFontLoaded(_)) => {
                Command::none()
            } (_, Message::MinimizeUI) => {
                if self.mini_ui {
                    self.mini_ui = false;
                    let new_size = Size::new(self.settings_tab.window_width,self.settings_tab.window_height);
                    iced::window::resize(new_size)
                } else {
                    self.mini_ui = true;
                    let new_size =
                        // "110" accounts for the buttons below the board, since the board
                        // is a square, we make the width the same as the height,
                        // with just a bit extra for the > button
                        Size::new((self.settings_tab.window_height - 110) + 25,
                        self.settings_tab.window_height);
                    iced::window::resize(new_size)
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.engine_state {
            EngineStatus::TurnedOff => {
                iced::subscription::events().map(Message::EventOccurred)
            } _ => {
                Subscription::batch(vec![
                    Engine::run_engine(self.engine.clone()),
                    iced::subscription::events().map(Message::EventOccurred)
                ])
            }
        }
    }

    fn view(&self) -> Element<Message, iced::Renderer<styles::Theme>> {
        let has_previous = !self.puzzle_tab.puzzles.is_empty() && self.puzzle_tab.current_puzzle > 0;
        let has_more_puzzles = !self.puzzle_tab.puzzles.is_empty() && self.puzzle_tab.current_puzzle < self.puzzle_tab.puzzles.len() - 1;
        let is_fav = if self.puzzle_tab.puzzles.is_empty() {
            false
        } else {
            db::is_favorite(&self.puzzle_tab.puzzles[self.puzzle_tab.current_puzzle].puzzle_id)
        };
        let resp = responsive(move |size| {
            gen_view(
                self.game_mode,
                self.puzzle_tab.current_puzzle_side,
                self.settings_tab.flip_board,
                self.settings_tab.show_coordinates,
                &self.board,
                &self.analysis.current_position(),
                self.from_square,
                self.last_move_from,
                self.last_move_to,
                self.hint_square,
                self.settings_tab.saved_configs.piece_theme,
                &self.puzzle_status,
                is_fav,
                has_more_puzzles,
                has_previous,
                self.analysis_history.len(),
                self.puzzle_tab.current_puzzle_move,
                self.puzzle_tab.game_status,
                &self.active_tab,
                &self.engine_eval,
                &self.engine_move,

                self.engine_state != EngineStatus::TurnedOff,
                self.search_tab.tab_label(),
                self.settings_tab.tab_label(),
                self.puzzle_tab.tab_label(),
                self.search_tab.view(),
                self.settings_tab.view(),
                self.puzzle_tab.view(),
                &self.lang,
                size,
                self.mini_ui,
            )});
        Container::new(resp)
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
    show_coordinates: bool,
    board: &Board,
    analysis: &Board,
    from_square: Option<Square>,
    last_move_from: Option<Square>,
    last_move_to: Option<Square>,
    hint_square: Option<Square>,
    piece_theme: styles::PieceTheme,
    puzzle_status: &'a str,
    is_fav: bool,
    has_more_puzzles: bool,
    has_previous: bool,
    analysis_history_len: usize,
    current_puzzle_move: usize,
    game_status: GameStatus,
    active_tab: &TabId,
    engine_eval: &str,
    engine_move: &str,

    engine_started: bool,
    search_tab_label: TabLabel,
    settings_tab_label: TabLabel,
    puzzle_tab_label: TabLabel,
    search_tab: Element<'a, Message, iced::Renderer<styles::Theme>>,
    settings_tab: Element<'a, Message, iced::Renderer<styles::Theme>>,
    puzzle_tab: Element<'a, Message, iced::Renderer<styles::Theme>>,
    lang: &lang::Language,
    size: Size,
    mini_ui: bool,
) -> Element<'a, Message, iced::Renderer<styles::Theme>> {

    let font = piece_theme == PieceTheme::FontAlpha;
    let mut board_col = Column::new().spacing(0).align_items(Alignment::Center);
    let mut board_row = Row::new().spacing(0).align_items(Alignment::Center);

    let is_white = (current_puzzle_side == Color::White) ^ flip_board;

    //Reserve more space below the board if we'll show the engine eval
    let board_height = if engine_eval.is_empty() {
        if show_coordinates {
            ((size.height - 120.) / 8.) as u16
        } else {
            ((size.height - 110.) / 8.) as u16
        }
    } else {
        if show_coordinates {
            ((size.height - 150.) / 8.) as u16
        } else {
            ((size.height - 140.) / 8.) as u16
        }
    };

    let ranks;
    let files;
    if is_white {
        ranks = (0..8).rev().collect::<Vec<i32>>();
        files = (0..8).collect::<Vec<i32>>();
    } else {
        ranks = (0..8).collect::<Vec<i32>>();
        files = (0..8).rev().collect::<Vec<i32>>();
    };
    for rank in ranks {
        for file in &files {
            let pos = Square::make_square(Rank::from_index(rank as usize), File::from_index(*file as usize));

            let (piece, color) =
                match game_mode {
                    config::GameMode::Analysis => {
                        (analysis.piece_on(pos),
                        analysis.color_on(pos))
                    } config::GameMode::Puzzle => {
                        (board.piece_on(pos),
                        board.color_on(pos))
                    }
                };

            let mut text;
            let light_square = (rank + file) % 2 != 0;

            let selected =
                if game_mode == config::GameMode::Puzzle {
                    from_square == Some(pos)    ||
                    last_move_from == Some(pos) ||
                    last_move_to == Some(pos)   ||
                    hint_square == Some(pos)
                } else {
                    from_square == Some(pos)
                };
            if font {
                let square_style :styles::ButtonStyle = if selected {
                    styles::ButtonStyle::SelectedPaper
                } else {
                    styles::ButtonStyle::Paper
                };

                if let Some(piece) = piece {
                    if color.unwrap() == Color::White {
                        text = match piece {
                            Piece::Pawn => String::from("P"),
                            Piece::Rook => String::from("R"),
                            Piece::Knight => String::from("H"),
                            Piece::Bishop => String::from("B"),
                            Piece::Queen => String::from("Q"),
                            Piece::King => String::from("K"),
                        };
                    } else {
                        text = match piece {
                            Piece::Pawn => String::from("O"),
                            Piece::Rook => String::from("T"),
                            Piece::Knight => String::from("J"),
                            Piece::Bishop => String::from("N"),
                            Piece::Queen => String::from("W"),
                            Piece::King => String::from("L"),
                        };
                    }
                    if light_square {
                        text = text.to_lowercase();
                    }
                } else {
                    if light_square {
                        text = String::from("0");
                    } else {
                        text = String::from("+");
                    }
                }
                board_row =
                    board_row.push(Button::new(
                        Text::new(text)
                            .width(board_height)
                            .height(board_height)
                            .font(config::CHESS_ALPHA)
                            .size(board_height)
                            .vertical_alignment(alignment::Vertical::Center)
                            .line_height(LineHeight::Absolute(board_height.into())
                        ))
                    .padding(0)
                    .on_press(Message::SelectSquare(pos))
                    .style(square_style)
                );
            } else {
                let square_style :styles::ButtonStyle = if light_square {
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
                if let Some(piece) = piece {
                    let text;
                    if color.unwrap() == Color::White {
                        text = match piece {
                            Piece::Pawn => "/wP.svg",
                            Piece::Rook => "/wR.svg",
                            Piece::Knight => "/wN.svg",
                            Piece::Bishop => "/wB.svg",
                            Piece::Queen => "/wQ.svg",
                            Piece::King => "/wK.svg"
                        }
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
                    board_row = board_row.push(
                        Button::new(
                            Svg::from_path(String::from("pieces/") + &piece_theme.to_string() + text)
                        ).width(board_height)
                        .height(board_height)
                        .on_press(Message::SelectSquare(pos))
                        .style(square_style)
                    );
                } else {
                    board_row = board_row.push(Button::new(Text::new(""))
                        .width(board_height)
                        .height(board_height)
                        .on_press(Message::SelectSquare(pos))
                        .style(square_style)
                    );
                }
            }
        }

        if show_coordinates {
            board_row = board_row.push(
                Container::new(
                    Text::new((rank + 1).to_string()).size(15)
                ).align_y(iced::alignment::Vertical::Bottom)
                .align_x(iced::alignment::Horizontal::Right)
                .padding(3)
                .height(board_height)
            );
        }
        board_col = board_col.push(board_row);
        board_row = Row::new().spacing(0).align_items(Alignment::Center);
    }
    if show_coordinates {
        if is_white {
            board_col = board_col.push(row![
                Text::new("a").size(15).width(board_height),
                Text::new("b").size(15).width(board_height),
                Text::new("c").size(15).width(board_height),
                Text::new("d").size(15).width(board_height),
                Text::new("e").size(15).width(board_height),
                Text::new("f").size(15).width(board_height),
                Text::new("g").size(15).width(board_height),
                Text::new("h").size(15).width(board_height),
            ]);
        } else {
            board_col = board_col.push(row![
                Text::new("h").size(15).width(board_height),
                Text::new("g").size(15).width(board_height),
                Text::new("f").size(15).width(board_height),
                Text::new("e").size(15).width(board_height),
                Text::new("d").size(15).width(board_height),
                Text::new("c").size(15).width(board_height),
                Text::new("b").size(15).width(board_height),
                Text::new("a").size(15).width(board_height),
            ]);
        }
    }

    let game_mode_row = row![
        Text::new(lang::tr(lang, "mode")),
        Radio::new(lang::tr(lang, "mode_puzzle"), config::GameMode::Puzzle, Some(game_mode), Message::SelectMode),
        Radio::new(lang::tr(lang, "mode_analysis"), config::GameMode::Analysis, Some(game_mode), Message::SelectMode)
    ].spacing(10).padding(10).align_items(Alignment::Center);

    let fav_label = if is_fav {
        lang::tr(lang, "unfav")
    } else {
        lang::tr(lang, "fav")
    };
    let mut navigation_row = Row::new().padding(3).spacing(10);
    if game_mode == config::GameMode::Analysis {
        if analysis_history_len > current_puzzle_move {
            navigation_row = navigation_row.push(Button::new(Text::new(lang::tr(lang, "takeback"))).on_press(Message::GoBackMove));
        } else {
            navigation_row = navigation_row.push(Button::new(Text::new(lang::tr(lang, "takeback"))));
        }
        if engine_started {
            navigation_row = navigation_row.push(Button::new(Text::new(lang::tr(lang, "stop_engine"))).on_press(Message::StartEngine));
        } else {
            navigation_row = navigation_row.push(Button::new(Text::new(lang::tr(lang, "start_engine"))).on_press(Message::StartEngine));
        }
    } else {
        if has_previous {
            navigation_row = navigation_row.push(Button::new(Text::new(lang::tr(lang, "previous"))).on_press(Message::ShowPreviousPuzzle))
        } else {
            navigation_row = navigation_row.push(Button::new(Text::new(lang::tr(lang, "previous"))));
        }
        if has_more_puzzles {
            navigation_row = navigation_row.push(Button::new(Text::new(lang::tr(lang, "next"))).on_press(Message::ShowNextPuzzle))
        } else {
            navigation_row = navigation_row.push(Button::new(Text::new(lang::tr(lang, "next"))));
        }
        if game_status == GameStatus::NoPuzzles {
            navigation_row = navigation_row
                .push(Button::new(Text::new(lang::tr(lang, "redo"))))
                .push(Button::new(Text::new(fav_label)))
                .push(Button::new(Text::new(lang::tr(lang, "hint"))));
        } else if game_status == GameStatus::PuzzleEnded {
            navigation_row = navigation_row
                .push(Button::new(Text::new(lang::tr(lang, "redo"))).on_press(Message::RedoPuzzle))
                .push(Button::new(Text::new(fav_label)).on_press(Message::FavoritePuzzle))
                .push(Button::new(Text::new(lang::tr(lang, "hint"))));
        } else {
            navigation_row = navigation_row
                .push(Button::new(Text::new(lang::tr(lang, "redo"))).on_press(Message::RedoPuzzle))
                .push(Button::new(Text::new(fav_label)).on_press(Message::FavoritePuzzle))
                .push(Button::new(Text::new(lang::tr(lang, "hint"))).on_press(Message::ShowHint));
        }
    }

    board_col = board_col.push(Text::new(puzzle_status)).push(game_mode_row).push(navigation_row);
    if !engine_eval.is_empty() {
        board_col = board_col.push(
            row![
                Text::new(String::from(lang::tr(lang, "eval")) + &engine_eval),
                Text::new(String::from(lang::tr(lang, "best_move")) + &engine_move)
            ].padding(5).spacing(15)
        );
    }
    if  mini_ui {
        let button_mini = Button::new(Text::new(">")).on_press(Message::MinimizeUI);
        row![board_col,button_mini].spacing(5).align_items(Alignment::Start).into()
    } else {
        let button_mini = Button::new(Text::new("<")).on_press(Message::MinimizeUI);
        let tabs = Tabs::new(Message::TabSelected)
                .push(TabId::Search, search_tab_label, search_tab)
                .push(TabId::Settings, settings_tab_label, settings_tab)
                .push(TabId::CurrentPuzzle ,puzzle_tab_label, puzzle_tab)
                .tab_bar_position(iced_aw::TabBarPosition::Top)
                .set_active_tab(active_tab);

        row![board_col,button_mini,tabs].spacing(5).align_items(Alignment::Start).into()
    }
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
