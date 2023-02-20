use iced::widget::{Container, Column, Scrollable, Text, TextInput};
use iced::{Element};
use iced::{alignment, Command, Alignment, Length};

use chess::{Color, Piece};
use iced_aw::TabLabel;

use crate::{Message, Tab, config, styles};

#[derive(Debug, Clone)]
pub enum PuzzleMessage {
    ChangeTextInputs(String),
}

#[derive(Debug, Clone)]
pub struct PuzzleTab {
    pub puzzles: Vec<config::Puzzle>,
    pub current_puzzle: usize,
    pub current_puzzle_move: usize,
    pub current_puzzle_side: Color,
    pub is_playing: bool,
}

impl PuzzleTab {
    pub fn new() -> Self {
        PuzzleTab {
            puzzles: Vec::new(),
            current_puzzle: 0,
            current_puzzle_move: 1,
            current_puzzle_side: Color::White,
            is_playing: false,
        }
    }

    pub fn update(&mut self, message: PuzzleMessage) -> Command<Message> {
        match message {
            PuzzleMessage::ChangeTextInputs(_) => {
                Command::none()
            }
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
        TabLabel::IconText('\u{F217}', self.title())
    }

    fn content(&self) -> Element<Message, iced::Renderer<styles::Theme>> {
        let col_puzzle_info = if !self.puzzles.is_empty() && self.current_puzzle < self.puzzles.len() {
            Scrollable::new(Column::new().spacing(10).align_items(Alignment::Center)
                .spacing(10)
                .push(
                    Text::new(String::from("Puzzle link: "))
                ).push(
                    TextInput::new("",
                    &("https://lichess.org/training/".to_owned() + &self.puzzles[self.current_puzzle].puzzle_id),
                    PuzzleMessage::ChangeTextInputs)                        
                ).push(Text::new("FEN:")
                ).push(
                    TextInput::new(
                        &self.puzzles[self.current_puzzle].fen,
                        &self.puzzles[self.current_puzzle].fen,
                        PuzzleMessage::ChangeTextInputs,
                    )
                ).push(Text::new(String::from("Rating: ") + &self.puzzles[self.current_puzzle].rating.to_string())
                ).push(Text::new(String::from("Rating Deviation: ") + &self.puzzles[self.current_puzzle].rating_deviation.to_string())
                ).push(Text::new(String::from("Popularity (-100 to 100): ") + &self.puzzles[self.current_puzzle].popularity.to_string())
                ).push(Text::new(String::from("Times Played (on lichess): ") + &self.puzzles[self.current_puzzle].nb_plays.to_string())
                ).push(Text::new("Themes:")
                ).push(Text::new(&self.puzzles[self.current_puzzle].themes)
                ).push(Text::new("Game url: ")
                ).push(
                    TextInput::new(
                        &self.puzzles[self.current_puzzle].game_url,
                        &self.puzzles[self.current_puzzle].game_url,
                        PuzzleMessage::ChangeTextInputs,
                    )
                )
            )
        } else {
            Scrollable::new(Column::new().spacing(10)
                .spacing(10)
                .push(
                    Text::new("No puzzle loaded")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                ))
        };
        let content: Element<PuzzleMessage, iced::Renderer<styles::Theme>> = Container::new(col_puzzle_info)
            .align_x(alignment::Horizontal::Center).height(Length::Fill).into();

        content.map(Message::PuzzleInfo)
    }
}