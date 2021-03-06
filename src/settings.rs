use iced::pure::widget::{Button, Container, Checkbox, Column, Text, TextInput, Row, PickList};
use iced::pure::{Element};
use iced::{alignment, Command, Alignment, Length};

use iced_aw::pure::TabLabel;

use crate::{Message, Tab, config, styles};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ChangeSquareSize(String),
    CheckPlaySound(bool),
    CheckAutoLoad(bool),
    SelectPieceTheme(styles::PieceTheme),
    SelectBoardTheme(styles::BoardStyle),
    ChangePuzzleDbLocation(String),
    //ChangePieceTheme(String),
    ChangeSearchResultLimit(String),
    ChangePressed
}

#[derive(Debug, Clone)]
pub struct SettingsTab {
    square_size_value: String,

    piece_theme: styles::PieceTheme,
    board_theme: styles::BoardStyle,
    play_sound: bool,
    auto_load_next: bool,

    puzzle_db_location_value: String,
    search_results_limit_value: String,

    settings_status: String,
}

impl SettingsTab {
    pub fn new() -> Self {
        SettingsTab {
            square_size_value: config::SETTINGS.square_size.to_string(),
            piece_theme: config::SETTINGS.piece_theme,
            board_theme: config::SETTINGS.board_theme,
            play_sound: config::SETTINGS.play_sound,
            auto_load_next: config::SETTINGS.auto_load_next,
            puzzle_db_location_value: String::from(&config::SETTINGS.puzzle_db_location),
            search_results_limit_value: config::SETTINGS.search_results_limit.to_string(),
            settings_status: String::new(),
        }
    }

    pub fn update(&mut self, message: SettingsMessage) -> Command<Message> {
        match message {
            SettingsMessage::ChangeSquareSize(value) => {
                if value == "" {
                    self.square_size_value = String::from("0");
                } else {
                    if let Ok(new_val) = value.parse::<u16>() {
                        self.square_size_value = new_val.to_string();
                        self.settings_status = String::from("");
                    }
                }
                Command::none()
            }
            SettingsMessage::SelectPieceTheme(value) => {
                self.piece_theme = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.piece_theme, self.board_theme), Message::ChangeSettings)
            }
            SettingsMessage::SelectBoardTheme(value) => {
                self.board_theme = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.piece_theme, self.board_theme), Message::ChangeSettings)
            }
            SettingsMessage::ChangePuzzleDbLocation(value) => {
                self.puzzle_db_location_value = value;
                Command::none()
            }
            SettingsMessage::ChangeSearchResultLimit(value) => {
                if value == "" {
                    self.search_results_limit_value = String::from("0");
                } else {
                    if let Ok(new_val) = value.parse::<usize>() {
                        self.search_results_limit_value = new_val.to_string();
                        self.settings_status = String::from("");
                    }
                }
                Command::none()
            }
            SettingsMessage::CheckPlaySound(value) => {
                self.play_sound = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.piece_theme, self.board_theme), Message::ChangeSettings)
            }
            SettingsMessage::CheckAutoLoad(value) => {
                self.auto_load_next = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.piece_theme, self.board_theme), Message::ChangeSettings)
            }
            SettingsMessage::ChangePressed => {
                let config = config::OfflinePuzzlesConfig {
                    square_size: self.square_size_value.parse().unwrap(),
                    puzzle_db_location: String::from(&self.puzzle_db_location_value),
                    piece_theme: self.piece_theme,
                    search_results_limit: self.search_results_limit_value.parse().unwrap(),
                    play_sound: self.play_sound,
                    auto_load_next: self.auto_load_next,
                    board_theme: self.board_theme,
                    light_squares_color: self.board_theme.light_sqr(),
                    dark_squares_color: self.board_theme.dark_sqr(),
                };
                let file = std::fs::File::create("settings.json");
                match file {
                    Ok(file) => {
                        if let Ok(_) = serde_json::to_writer_pretty(file, &config) {                
                            self.settings_status = String::from("Settings saved!");
                        } else {
                            self.settings_status = String::from("Error saving config file.");
                        }
                    } Err(_) => self.settings_status = String::from("Error reading config file.")
                }
                Command::none()
            }
        }
    }

    pub async fn send_changes(play_sound: bool, auto_load: bool, pieces: styles::PieceTheme, board: styles::BoardStyle) -> Option<config::OfflinePuzzlesConfig> {
        let mut config = config::load_config();
        config.board_theme = board;
        config.piece_theme = pieces;
        config.play_sound = play_sound;
        config.auto_load_next = auto_load;
        config.light_squares_color = board.light_sqr();
        config.dark_squares_color = board.dark_sqr();
        Some(config)
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText('\u{F217}'.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let col_settings = Column::new().spacing(10).align_items(Alignment::Center)
            .spacing(10)
            .push(
                Text::new("(Size and search limit REQUIRE restart)")
                .width(Length::Shrink)
                .horizontal_alignment(alignment::Horizontal::Center),    
            )
            .push(
                Row::new().spacing(5).align_items(Alignment::Center)
                .push(
                    Text::new("Piece Theme:")
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    PickList::new(
                        &styles::PieceTheme::ALL[..],
                        Some(self.piece_theme),
                        SettingsMessage::SelectPieceTheme
                    )
                )
            )
            .push(
                Row::new().spacing(5).align_items(Alignment::Center)
                .push(
                    Text::new("Board Theme:")
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    PickList::new(
                        &styles::BoardStyle::ALL[..],
                        Some(self.board_theme),
                        SettingsMessage::SelectBoardTheme
                    )
                )
            )
            .push(
                Row::new().spacing(5).align_items(Alignment::Center)
                    .push(
                        Text::new("Square size:")
                        .width(Length::Shrink)
                        .horizontal_alignment(alignment::Horizontal::Center),    
                    )
                    .push(
                        TextInput::new(
                            &self.square_size_value.to_string(),
                            &self.square_size_value.to_string(),
                            SettingsMessage::ChangeSquareSize,
                        )
                        .padding(10)
                        .size(20),
                    )
                )
            .push(
                Row::new().spacing(5).align_items(Alignment::Center)
                    .push(
                        Text::new("Play sound on moves:")
                        .width(Length::Shrink)
                        .horizontal_alignment(alignment::Horizontal::Center),    
                    )
                    .push(
                        Checkbox::new(
                            self.play_sound,
                            "",
                            SettingsMessage::CheckPlaySound,
                        )
                        .size(20),
                    )
                )
            .push(
                Row::new().spacing(5).align_items(Alignment::Center)
                    .push(
                        Text::new("Auto load next puzzle:")
                        .width(Length::Shrink)
                        .horizontal_alignment(alignment::Horizontal::Center),    
                    )
                    .push(
                        Checkbox::new(
                            self.auto_load_next,
                            "",
                            SettingsMessage::CheckAutoLoad,
                        )
                        .size(20),
                    )
                )
        
            /*
            .push(
                Text::new("Puzzle DB location")
                .width(Length::Shrink)
                .horizontal_alignment(HorizontalAlignment::Center),    
            )
            .push(
                TextInput::new(
                    &mut self.puzzle_db_location,
                    "Username",
                    &self.puzzle_db_location_value,
                    SettingsMessage::ChangePuzzleDbLocation,
                )
                .padding(10)
                .size(32),
            )
            */
            .push(
                Row::new().spacing(5).align_items(Alignment::Center)
    
                .push(
                    Text::new("Search Result limit:")
                    .width(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Center),    
                )
                .push(
                    TextInput::new(
                        &self.search_results_limit_value,
                        &self.search_results_limit_value,
                        SettingsMessage::ChangeSearchResultLimit,
                    )
                    .padding(10)
                    .size(20),
                )
            )
            .push(
                Button::new(
                    Text::new("Save Changes")).on_press(SettingsMessage::ChangePressed)
            )
            .push(
                Text::new(&self.settings_status)
                .width(Length::Shrink)
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Bottom),
            );
        let content: Element<'_, SettingsMessage> = Container::new(col_settings).align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into();

        content.map(Message::Settings)
    }
}
