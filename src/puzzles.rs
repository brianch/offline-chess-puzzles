use iced::widget::{Container, column as col, row, Scrollable, Text, TextInput, Button};
use iced::{Element};
use iced::{alignment, Command, Alignment, Length};
use chess::{Color, Piece};
use iced_aw::TabLabel;

use crate::{Message, Tab, config, styles, lang};

#[derive(Debug, Clone)]
pub enum PuzzleMessage {
    ChangeTextInputs(String),
    CopyText(String),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameStatus {
    Playing, PuzzleEnded, NoPuzzles, 
}

#[derive(Debug, Clone)]
pub struct PuzzleTab {
    pub puzzles: Vec<config::Puzzle>,
    pub current_puzzle: usize,
    pub current_puzzle_move: usize,
    pub current_puzzle_side: Color,
    pub game_status: GameStatus,
    pub current_puzzle_fen: String,
    pub lang: lang::Language,
}

impl PuzzleTab {
    pub fn new() -> Self {
        PuzzleTab {
            puzzles: Vec::new(),
            current_puzzle: 0,
            current_puzzle_move: 1,
            current_puzzle_side: Color::White,
            game_status: GameStatus::NoPuzzles,
            current_puzzle_fen: String::new(),
            lang: config::SETTINGS.lang,
        }
    }

    pub fn update(&mut self, message: PuzzleMessage) -> Command<Message> {
        match message {
            PuzzleMessage::ChangeTextInputs(_) => {
                Command::none()
            } PuzzleMessage::CopyText(text) => {
                iced::clipboard::write::<Message>(text)
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

    pub fn is_playing(&self) -> bool {
        self.game_status != GameStatus::NoPuzzles
    }
}

impl Tab for PuzzleTab {
    type Message = Message;

    fn title(&self) -> String {
        lang::tr(&self.lang, "current_puzzle")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText('▾', self.title())
    }

    fn content(&self) -> Element<Message, iced::Renderer<styles::Theme>> {
        let col_puzzle_info = if !self.puzzles.is_empty() && self.current_puzzle < self.puzzles.len() {
            Scrollable::new(col![
                Text::new(lang::tr(&self.lang, "puzzle_link")),
                row![
                    TextInput::new("",
                        &("https://lichess.org/training/".to_owned() + &self.puzzles[self.current_puzzle].puzzle_id),
                    ).on_input(PuzzleMessage::ChangeTextInputs),
                    Button::new(Text::new(lang::tr(&self.lang, "copy"))).on_press(PuzzleMessage::CopyText("https://lichess.org/training/".to_owned() + &self.puzzles[self.current_puzzle].puzzle_id)),
                ],
                Text::new(lang::tr(&self.lang, "fen")),
                row![
                    TextInput::new(
                        &self.current_puzzle_fen,
                        &self.current_puzzle_fen,
                    ).on_input(PuzzleMessage::ChangeTextInputs),
                    Button::new(Text::new(lang::tr(&self.lang, "copy"))).on_press(PuzzleMessage::CopyText(self.current_puzzle_fen.clone())),
                ],
                Text::new(String::from(lang::tr(&self.lang, "rating")) + &self.puzzles[self.current_puzzle].rating.to_string()),
                Text::new(String::from(lang::tr(&self.lang, "rd")) + &self.puzzles[self.current_puzzle].rating_deviation.to_string()),
                Text::new(String::from(lang::tr(&self.lang, "popularity")) + &self.puzzles[self.current_puzzle].popularity.to_string()),
                Text::new(String::from(lang::tr(&self.lang, "times_played")) + &self.puzzles[self.current_puzzle].nb_plays.to_string()),
                Text::new(lang::tr(&self.lang, "themes")),
                Text::new(&self.puzzles[self.current_puzzle].themes),
                Text::new(lang::tr(&self.lang, "url")),
                row![
                    TextInput::new(
                        &self.puzzles[self.current_puzzle].game_url,
                        &self.puzzles[self.current_puzzle].game_url,
                    ).on_input(PuzzleMessage::ChangeTextInputs),
                    Button::new(Text::new(lang::tr(&self.lang, "copy"))).on_press(PuzzleMessage::CopyText(self.puzzles[self.current_puzzle].game_url.clone())),
                ],
            ].spacing(10).align_items(Alignment::Center))
        } else {
            Scrollable::new(col![
                    Text::new(lang::tr(&self.lang, "no_puzzle"))
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                ].spacing(10))
        };
        let content: Element<PuzzleMessage, iced::Renderer<styles::Theme>> = Container::new(col_puzzle_info)
            .align_x(alignment::Horizontal::Center).height(Length::Fill).into();

        content.map(Message::PuzzleInfo)
    }
}
