use iced::widget::{Button, Container, Checkbox, column, Column, Text, TextInput, row, PickList, Scrollable};
use iced::{alignment, Command, Alignment, Element, Length};

use iced_aw::TabLabel;

use crate::{Message, Tab, config, styles, lang, lang::PickListWrapper};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    CheckPlaySound(bool),
    CheckAutoLoad(bool),
    CheckFlipBoard(bool),
    CheckShowCoords(bool),
    SelectPieceTheme(styles::PieceTheme),
    SelectBoardTheme(styles::Theme),
    SelectLanguage(PickListWrapper<lang::Language>),
    ChangePDFExportPgs(String),
    ChangePuzzleDbLocation(String),
    ChangeSearchResultLimit(String),
    ChangeEnginePath(String),
    ChangePressed
}

pub struct SettingsTab {
    pub engine_path: String,
    pub window_width: u32,
    pub window_height: u32,
    pub piece_theme: styles::PieceTheme,
    pub board_theme: styles::Theme,
    pub lang: PickListWrapper<lang::Language>,
    pub export_pgs: String,
    theme: styles::Theme,
    play_sound: bool,
    auto_load_next: bool,
    pub flip_board: bool,
    pub show_coordinates: bool,

    puzzle_db_location_value: String,
    search_results_limit_value: String,

    settings_status: String,
    pub saved_configs: config::OfflinePuzzlesConfig,
}

impl SettingsTab {
    pub fn new() -> Self {
        SettingsTab {
            engine_path: config::SETTINGS.engine_path.clone().unwrap_or_default(),
            window_width: config::SETTINGS.window_width,
            window_height: config::SETTINGS.window_width,
            piece_theme: config::SETTINGS.piece_theme,
            board_theme: config::SETTINGS.board_theme,
            lang: PickListWrapper::new_lang(config::SETTINGS.lang, config::SETTINGS.lang),
            export_pgs: config::SETTINGS.export_pgs.to_string(),
            theme: styles::Theme::Blue,
            play_sound: config::SETTINGS.play_sound,
            auto_load_next: config::SETTINGS.auto_load_next,
            flip_board: config::SETTINGS.flip_board,
            show_coordinates: config::SETTINGS.show_coordinates,
            puzzle_db_location_value: String::from(&config::SETTINGS.puzzle_db_location),
            search_results_limit_value: config::SETTINGS.search_results_limit.to_string(),
            settings_status: String::new(),
            saved_configs: config::load_config(),
        }
    }

    pub fn update(&mut self, message: SettingsMessage) -> Command<Message> {
        match message {
            SettingsMessage::SelectPieceTheme(value) => {
                self.piece_theme = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.show_coordinates, self.piece_theme, self.board_theme, self.engine_path.clone(), self.lang.lang), Message::ChangeSettings)
            }
            SettingsMessage::SelectBoardTheme(value) => {
                self.board_theme = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.show_coordinates, self.piece_theme, self.theme, self.engine_path.clone(), self.lang.lang), Message::ChangeSettings)
            }
            SettingsMessage::SelectLanguage(value) => {
                self.lang = value;
                self.lang.lang = self.lang.item;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.show_coordinates, self.piece_theme, self.theme, self.engine_path.clone(), self.lang.lang), Message::ChangeSettings)
            }
            SettingsMessage::ChangePuzzleDbLocation(value) => {
                self.puzzle_db_location_value = value;
                Command::none()
            }
            SettingsMessage::ChangeEnginePath(value) => {
                self.engine_path = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.show_coordinates, self.piece_theme, self.board_theme, self.engine_path.clone(), self.lang.lang), Message::ChangeSettings)
            }
            SettingsMessage::ChangeSearchResultLimit(value) => {
                if value.is_empty() {
                    self.search_results_limit_value = String::from("0");
                } else if let Ok(new_val) = value.parse::<usize>() {
                    self.search_results_limit_value = new_val.to_string();
                    self.settings_status = String::from("");
                }
                Command::none()
            }
            SettingsMessage::CheckPlaySound(value) => {
                self.play_sound = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.show_coordinates, self.piece_theme, self.board_theme, self.engine_path.clone(), self.lang.lang), Message::ChangeSettings)
            }
            SettingsMessage::CheckAutoLoad(value) => {
                self.auto_load_next = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.show_coordinates, self.piece_theme, self.board_theme, self.engine_path.clone(), self.lang.lang), Message::ChangeSettings)
            }
            SettingsMessage::CheckFlipBoard(value) => {
                self.flip_board = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.show_coordinates, self.piece_theme, self.board_theme, self.engine_path.clone(), self.lang.lang), Message::ChangeSettings)
            }
            SettingsMessage::CheckShowCoords(value) => {
                self.show_coordinates = value;
                Command::none()
            }
            SettingsMessage::ChangePDFExportPgs(value) => {
                if let Ok(_) = value.parse::<i32>() {
                    self.export_pgs = value;
                } else if value == "" {
                    self.export_pgs = String::from("0");
                }
                Command::none()
            } SettingsMessage::ChangePressed => {
                let engine_path = if self.engine_path.is_empty() {
                    None
                } else {
                    Some(self.engine_path.clone())
                };
                let config = config::OfflinePuzzlesConfig {
                    engine_path: engine_path,
                    engine_limit: self.saved_configs.engine_limit.clone(),
                    window_width: self.window_width,
                    window_height: self.window_height,
                    puzzle_db_location: String::from(&self.puzzle_db_location_value),
                    piece_theme: self.piece_theme,
                    search_results_limit: self.search_results_limit_value.parse().unwrap(),
                    play_sound: self.play_sound,
                    auto_load_next: self.auto_load_next,
                    flip_board: self.flip_board,
                    show_coordinates: self.show_coordinates,
                    board_theme: self.board_theme,
                    lang: self.lang.lang,
                    export_pgs: self.export_pgs.parse().unwrap(),
                    last_min_rating: self.saved_configs.last_min_rating,
                    last_max_rating: self.saved_configs.last_max_rating,
                    last_theme: self.saved_configs.last_theme,
                    last_opening: self.saved_configs.last_opening,
                    last_variation: self.saved_configs.last_variation.clone(),
                    last_opening_side: self.saved_configs.last_opening_side,
                };
                let file = std::fs::File::create("settings.json");
                match file {
                    Ok(file) => {
                        if serde_json::to_writer_pretty(file, &config).is_ok() {
                            self.settings_status = lang::tr(&self.lang.lang, "settings_saved");
                        } else {
                            self.settings_status = lang::tr(&self.lang.lang, "error_saving");
                        }
                    } Err(_) => self.settings_status = lang::tr(&self.lang.lang, "error_reading_config")
                }
                Command::none()
            }
        }
    }

    pub fn save_window_size(width: u32, height: u32) {
        let mut config = config::load_config();
        config.window_width = width;
        config.window_height = height;
        let file = std::fs::File::create("settings.json");
        match file {
            Ok(file) => {
                if !serde_json::to_writer_pretty(file, &config).is_ok() {
                    println!("Error saving config file.");
                }
            } Err(_) => println!("Error opening settings file")
        }
    }

    pub async fn send_changes(play_sound: bool, auto_load: bool, flip: bool, coords: bool, pieces: styles::PieceTheme, theme: styles::Theme, engine: String, lang: lang::Language) -> Option<config::OfflinePuzzlesConfig> {
        let engine = if engine.is_empty() {
            None
        } else {
            Some(engine)
        };
        let mut config = config::load_config();
        config.board_theme = theme;
        config.piece_theme = pieces;
        config.lang = lang;
        config.play_sound = play_sound;
        config.auto_load_next = auto_load;
        config.flip_board = flip;
        config.show_coordinates = coords;
        config.engine_path = engine;
        Some(config)
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        lang::tr(&self.lang.lang, "settings")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<Message, iced::Renderer<styles::Theme>> {
        let col_settings = column![
            row![
                Text::new(lang::tr(&self.lang.lang, "piece_theme")),
                PickList::new(
                    &styles::PieceTheme::ALL[..],
                    Some(self.piece_theme),
                    SettingsMessage::SelectPieceTheme
                )
            ].spacing(5).align_items(Alignment::Center),
            row![
                Text::new(lang::tr(&self.lang.lang, "board_theme")),
                PickList::new(
                    &styles::Theme::ALL[..],
                    Some(self.board_theme),
                    SettingsMessage::SelectBoardTheme
                )
            ].spacing(5).align_items(Alignment::Center),
            row![
                Text::new(lang::tr(&self.lang.lang, "language")),
                PickList::new(
                    PickListWrapper::get_langs(self.lang.lang.clone()),
                    Some(self.lang.clone()),
                    SettingsMessage::SelectLanguage
                )
            ].spacing(5).align_items(Alignment::Center),
            row![
                Text::new(lang::tr(&self.lang.lang, "play_sound")),
                Checkbox::new(
                    "",
                    self.play_sound,
                    SettingsMessage::CheckPlaySound,
                ).size(20),
            ].spacing(5).align_items(Alignment::Center),
            row![
                Text::new(lang::tr(&self.lang.lang, "auto_load")),
                Checkbox::new(
                    "",
                    self.auto_load_next,
                    SettingsMessage::CheckAutoLoad,
                ).size(20),
            ].spacing(5).align_items(Alignment::Center),
            row![
                Text::new(lang::tr(&self.lang.lang, "flip_board")),
                Checkbox::new(
                    "",
                    self.flip_board,
                    SettingsMessage::CheckFlipBoard,
                ).size(20),
            ].spacing(5).align_items(Alignment::Center),
            row![
                Text::new(lang::tr(&self.lang.lang, "show_coords")),
                Checkbox::new(
                    "",
                    self.show_coordinates,
                    SettingsMessage::CheckShowCoords,
                ).size(20),
            ].spacing(5).align_items(Alignment::Center),
            row![
                Text::new(lang::tr(&self.lang.lang, "pdf_number_of_pages")),
                TextInput::new(
                    &self.export_pgs,
                    &self.export_pgs,
                ).on_input(SettingsMessage::ChangePDFExportPgs).width(60).padding(10).size(20),
            ].spacing(5).align_items(Alignment::Center),
            row![
                Text::new(lang::tr(&self.lang.lang, "get_first_puzzles1")),
                TextInput::new(
                    &self.search_results_limit_value,
                    &self.search_results_limit_value,
                ).on_input(SettingsMessage::ChangeSearchResultLimit).width(80).padding(10).size(20),
                Text::new(lang::tr(&self.lang.lang, "get_first_puzzles2"))
            ].spacing(5).align_items(Alignment::Center),
            Text::new(lang::tr(&self.lang.lang, "engine_path")),
            TextInput::new(
                &self.engine_path,
                &self.engine_path,
            ).on_input(SettingsMessage::ChangeEnginePath).width(200).padding(10).size(20),
            Button::new(Text::new(lang::tr(&self.lang.lang, "save"))).padding(5).on_press(SettingsMessage::ChangePressed),
            Text::new(&self.settings_status).vertical_alignment(alignment::Vertical::Bottom),

        ].spacing(10).align_items(Alignment::Center);
        let content: Element<SettingsMessage, iced::Renderer<styles::Theme>> = Container::new(
            Scrollable::new(
                Column::new().spacing(10).push(col_settings)
            )
        ).align_x(alignment::Horizontal::Center).height(Length::Fill).width(Length::Fill).into();

        content.map(Message::Settings)
    }
}
