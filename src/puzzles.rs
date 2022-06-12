use iced::{
    button, text_input, alignment, Alignment, Button, Command, Column, Container, Element, Length,
    Text, TextInput
};
use std::str::FromStr;
use chess::{Color, Square, Piece};
use iced_aw::TabLabel;

use crate::{Message, Tab, config};

#[derive(Debug, Clone)]
pub enum PuzzleMessage {
    ChangeTextInputs(String),
    ShowHint
}

#[derive(Debug, Clone)]
pub struct PuzzleTab {
    //puzzle_info: config::Puzzle,
    pub puzzles: Vec<config::Puzzle>,
    pub current_puzzle: usize,
    pub current_puzzle_move: usize,
    pub current_puzzle_side: Color,
    pub is_playing: bool,
    hint_button: button::State,
    fen_state: text_input::State,
    url_state: text_input::State,
}

impl PuzzleTab {
    pub fn new() -> Self {
        PuzzleTab {
            //puzzle_info: config::Puzzle::new(),
            puzzles: Vec::new(),
            current_puzzle: 0,
            current_puzzle_move: 1,
            current_puzzle_side: Color::White,
            is_playing: false,
            hint_button: button::State::new(),
            fen_state: text_input::State::default(),
            url_state: text_input::State::default(),
        }
    }

    pub fn update(&mut self, message: PuzzleMessage) -> Command<Message> {
        match message {
            PuzzleMessage::ShowHint => {
                Command::perform(
                    PuzzleTab::get_hint(
                        String::from(&self.puzzles[self.current_puzzle].moves),
                        self.current_puzzle_move), Message::ShowHint)
            }
            PuzzleMessage::ChangeTextInputs(_) => {
                Command::none()
            }
        }
    }
    pub async fn get_hint(puzzle_moves: String, move_number: usize) -> Option<Square> {        
        let moves = puzzle_moves.split_whitespace().collect::<Vec<&str>>();
        if !moves.is_empty() && moves.len() > move_number {
            Some(Square::from_str(&moves[move_number][..2]).unwrap())
        } else {
            None
        }
    }

    // Checks if the notation indicates a promotion and return the piece
    // if that's the case.
    pub fn check_promotion(notation: &str) -> Option<Piece> {
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
}

impl Tab for PuzzleTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Current Puzzle")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText('\u{F217}'.into(), self.title())
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let col_puzzle_info;
        if self.is_playing {
            col_puzzle_info = Column::new().spacing(10).align_items(Alignment::Center)
                .spacing(10)
                .push(
                    Text::new(String::from("Puzzle ID: ") + &self.puzzles[self.current_puzzle].puzzle_id)
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),
                )
                .push(
                    Text::new("FEN:")
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    TextInput::new(
                        &mut self.fen_state,
                        &self.puzzles[self.current_puzzle].fen,
                        &self.puzzles[self.current_puzzle].fen,
                        PuzzleMessage::ChangeTextInputs,
                    )
                )                
                .push(
                    Text::new(String::from("Rating: ") + &self.puzzles[self.current_puzzle].rating.to_string())
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    Text::new(String::from("Rating Deviation: ") + &self.puzzles[self.current_puzzle].rating_deviation.to_string())
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    Text::new(String::from("Popularity: ") + &self.puzzles[self.current_puzzle].popularity.to_string())
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    Text::new(String::from("Times Played (on lichess): ") + &self.puzzles[self.current_puzzle].nb_plays.to_string())
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    Text::new("Themes:")
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    Text::new(&self.puzzles[self.current_puzzle].themes)
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    Text::new("URL: ")
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    TextInput::new(
                        &mut self.url_state,
                        &self.puzzles[self.current_puzzle].game_url,
                        &self.puzzles[self.current_puzzle].game_url,
                        PuzzleMessage::ChangeTextInputs,
                    )
                )
                .push(
                    Button::new(&mut self.hint_button,
                        Text::new("Hint")).on_press(PuzzleMessage::ShowHint)
                );
        } else {
            col_puzzle_info = Column::new().spacing(10).align_items(Alignment::Center)
                .spacing(10)
                .push(
                    Text::new("No puzzle loaded")
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
        }
        let content: Element<'_, PuzzleMessage> = Container::new(col_puzzle_info).align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into();

        content.map(Message::PuzzleInfo)
    }
}