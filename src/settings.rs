use iced::{
    button, text_input, pick_list, Align, Button, Column, Container, Element, HorizontalAlignment, VerticalAlignment, Length,
    Text, TextInput, PickList, Command, Row
};
use iced_aw::TabLabel;

use crate::{Message, Tab, config, styles};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ChangeSquareSize(String),
    SelectBoardTheme(styles::BoardStyle),
    ChangePuzzleDbLocation(String),
    //ChangePieceTheme(String),
    ChangeSearchResultLimit(String),
    ChangePressed
}

#[derive(Debug, Clone)]
pub struct SettingsTab {
    square_size_value: String,
    square_size: text_input::State,

    board_theme_list: pick_list::State<styles::BoardStyle>,
    board_theme: styles::BoardStyle,

    puzzle_db_location_value: String,
    puzzle_db_location: text_input::State,
    //puzzle_db_location: text_input::State,

    search_results_limit_value: String,
    search_results_limit: text_input::State,

    change_button: button::State,

    settings_status: String,
}

impl SettingsTab {
    pub fn new() -> Self {
        SettingsTab {
            square_size_value: config::SETTINGS.square_size.to_string(),
            square_size: text_input::State::default(),

            board_theme_list: pick_list::State::default(),
            board_theme: config::SETTINGS.board_theme,

            puzzle_db_location_value: String::from(&config::SETTINGS.puzzle_db_location),
            puzzle_db_location: text_input::State::default(),

            search_results_limit_value: config::SETTINGS.search_results_limit.to_string(),
            search_results_limit: text_input::State::default(),

            change_button: button::State::default(),
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
            SettingsMessage::SelectBoardTheme(value) => {
                self.board_theme = value;
                Command::perform(SettingsTab::send_theme_change(self.board_theme), Message::ChangeSettings)
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
            SettingsMessage::ChangePressed => {
                let config = config::OfflinePuzzlesConfig {
                    square_size: self.square_size_value.parse().unwrap(),
                    puzzle_db_location: String::from(&self.puzzle_db_location_value),
                    piece_theme: String::from("cburnett"),
                    search_results_limit: self.search_results_limit_value.parse().unwrap(),
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

    pub async fn send_theme_change(style: styles::BoardStyle) -> Option<config::OfflinePuzzlesConfig> {
        let mut config = config::load_config();
        config.board_theme = style;
        config.light_squares_color = style.light_sqr();
        config.dark_squares_color = style.dark_sqr();        
        Some(config)
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText('\u{E800}'.into(), self.title())
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let col_settings = Column::new().spacing(10).align_items(Align::Center)
            .spacing(10)
            .push(
                Text::new("(Size and search limit REQUIRE restart)")
                .width(Length::Shrink)
                .horizontal_alignment(HorizontalAlignment::Center),    
            )
            .push(
                Row::new().spacing(5).align_items(Align::Center)
                .push(
                    Text::new("Board Theme:")
                    .width(Length::Shrink)
                    .horizontal_alignment(HorizontalAlignment::Center),    
                )
                .push(
                    PickList::new(
                        &mut self.board_theme_list,
                        &styles::BoardStyle::ALL[..],
                        Some(self.board_theme),
                        SettingsMessage::SelectBoardTheme
                    )
                )
            )
            .push(
            Row::new().spacing(5).align_items(Align::Center)
                .push(
                    Text::new("Square size:")
                    .width(Length::Shrink)
                    .horizontal_alignment(HorizontalAlignment::Center),    
                )
                .push(
                    TextInput::new(
                        &mut self.square_size,
                        &self.square_size_value.to_string(),
                        &self.square_size_value.to_string(),
                        SettingsMessage::ChangeSquareSize,
                    )
                    .padding(10)
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
                Row::new().spacing(5).align_items(Align::Center)
    
                .push(
                    Text::new("Search Result limit:")
                    .width(Length::Shrink)
                    .horizontal_alignment(HorizontalAlignment::Center),    
                )
                .push(
                    TextInput::new(
                        &mut self.search_results_limit,
                        &self.search_results_limit_value,
                        &self.search_results_limit_value,
                        SettingsMessage::ChangeSearchResultLimit,
                    )
                    .padding(10)
                    .size(20),
                )
            )
            .push(
                Button::new(&mut self.change_button,
                    Text::new("Save Changes")).on_press(SettingsMessage::ChangePressed)
            )
            .push(
                Text::new(&self.settings_status)
                .width(Length::Shrink)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Bottom),
            );
        let content: Element<'_, SettingsMessage> = Container::new(col_settings).align_x(Align::Center)
            .align_y(Align::Center)
            .into();

        content.map(Message::Settings)
    }
}
