use iced_aw::tabs;

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        iced::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    }
}

pub const SELECTED_DARK_SQUARE: iced::Color = rgb!(170,162,58);
pub const SELECTED_LIGHT_SQUARE: iced::Color = rgb!(205,210,106);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PieceTheme {
    Cburnett,
    California,
    Cardinal,
    Dubrovny,
    Gioco,
    Icpieces,
    Maestro,
    Staunty,
    Tatiana,
}

impl PieceTheme {

    pub const ALL: [PieceTheme; 9] = [
        PieceTheme::Cburnett,
        PieceTheme::California,
        PieceTheme::Cardinal,
        PieceTheme::Dubrovny,
        PieceTheme::Gioco,
        PieceTheme::Icpieces,
        PieceTheme::Maestro,
        PieceTheme::Staunty,
        PieceTheme::Tatiana,
    ];
}

impl std::fmt::Display for PieceTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceTheme::California => "california",
                PieceTheme::Cardinal => "cardinal",
                PieceTheme::Dubrovny => "dubrovny",
                PieceTheme::Gioco => "gioco",
                PieceTheme::Icpieces => "icpieces",
                PieceTheme::Maestro => "maestro",
                PieceTheme::Staunty => "staunty",
                PieceTheme::Tatiana => "tatiana",
                _ => "cburnett",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum BoardStyle {
    Default,
    Brown,
    Green,
    Purple,
    Grey,
    Blue,
}

impl BoardStyle {

    pub const ALL: [BoardStyle; 5] = [
        BoardStyle::Brown,
        BoardStyle::Green,
        BoardStyle::Purple,
        BoardStyle::Grey,
        BoardStyle::Blue,
    ];

    pub fn light_sqr(&self) -> [f32; 3] {
        match self {
            #[allow(clippy::eq_op)]
            BoardStyle::Green  => [255.0 / 255.0, 255.0 / 255.0, 221.0 / 255.0],
            BoardStyle::Purple => [230.0 / 255.0, 219.0 / 255.0, 241.0 / 255.0],
            BoardStyle::Grey   => [222.0 / 255.0, 227.0 / 255.0, 230.0 / 255.0],
            #[allow(clippy::eq_op)]
            BoardStyle::Blue   => [234.0 / 255.0, 248.0 / 255.0, 255.0 / 255.0],
            _ => [240.0 / 255.0, 217.0 / 255.0, 181.0 / 255.0],
        }
    }
    pub fn dark_sqr(&self) -> [f32; 3] {
        match self {
            BoardStyle::Green  => [134.0 / 255.0, 166.0 / 255.0, 102.0 / 255.0],
            BoardStyle::Purple => [153.0 / 255.0, 125.0 / 255.0, 181.0 / 255.0],
            BoardStyle::Grey   => [140.0 / 255.0, 162.0 / 255.0, 173.0 / 255.0],
            BoardStyle::Blue   => [105.0 / 255.0, 171.0 / 255.0, 211.0 / 255.0],
            _ => [181.0 / 255.0, 136.0 / 255.0, 99.0 / 255.0],
        }
    }
}
impl std::fmt::Display for BoardStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoardStyle::Green => "Green",
                BoardStyle::Purple => "Purple",
                BoardStyle::Grey => "Grey",
                BoardStyle::Blue => "Blue",
                _ => "Brown",
            }
        )
    }
}

//Tab styles from the iced tab example, i'll leave then all here for now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabTheme {
    Default,
    Grey,
    Brown,
    Blue,
    Green,
    Purple,
}

impl TabTheme {
    pub const ALL: [TabTheme; 6] = [
        TabTheme::Default,
        TabTheme::Brown,
        TabTheme::Grey,
        TabTheme::Blue,
        TabTheme::Green,
        TabTheme::Purple,
    ];
}

impl Default for TabTheme {
    fn default() -> TabTheme {
        TabTheme::Default
    }
}

impl From<TabTheme> for String {
    fn from(tab_theme: TabTheme) -> Self {
        String::from(match tab_theme {
            TabTheme::Default => "Default",
            TabTheme::Grey => "Grey",
            TabTheme::Brown => "Brown",
            TabTheme::Blue => "Blue",
            TabTheme::Green => "Green",
            TabTheme::Purple => "Purple",
        })
    }
}

impl From<TabTheme> for Box<dyn tabs::StyleSheet> {
    fn from(tab_theme: TabTheme) -> Self {
        match tab_theme {
            TabTheme::Default => Default::default(),
            TabTheme::Grey => grey::TabBar.into(),
            TabTheme::Brown => brown::TabBar.into(),
            TabTheme::Blue => blue::TabBar.into(),
            TabTheme::Green => green::TabBar.into(),
            TabTheme::Purple => purple::TabBar.into(),
        }
    }
}

mod grey {
    use iced::{Background, Color};
    use iced_aw::tabs::{self, Style};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = if is_selected {
                Background::Color([0.55, 0.64, 0.68].into())
            } else {
                Background::Color([0.87, 0.9, 0.9].into())
            };

            let tab_label_border_color = if is_selected {
                [0.55, 0.64, 0.68].into()
            } else {
                [0.87, 0.9, 0.9].into()
            };

            let text_color = if is_selected {
                Color::WHITE
            } else {
                Color::BLACK
            };

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background,
                tab_label_border_color,
                tab_label_border_width: 1.0,
                icon_color: text_color,
                text_color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = Background::Color([0.35, 0.44, 0.48].into());
            let tab_label_border_color = [0.0, 0.0, 1.0].into();
            let text_color = Color::WHITE;

            Style {
                tab_label_background,
                tab_label_border_color,
                text_color,
                icon_color: text_color,
                ..self.active(is_selected)
            }
        }
    }
}

mod brown {
    use iced::{Background, Color};
    use iced_aw::tabs::{self, Style};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = if is_selected {
                Background::Color([0.71, 0.53, 0.39].into())
            } else {
                Background::Color([0.94, 0.85, 0.71].into())
            };

            let tab_label_border_color = if is_selected {
                [0.71, 0.53, 0.39].into()
            } else {
                [0.94, 0.85, 0.71].into()
            };

            let text_color = if is_selected {
                Color::WHITE
            } else {
                Color::BLACK
            };

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background,
                tab_label_border_color,
                tab_label_border_width: 1.0,
                icon_color: text_color,
                text_color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = Background::Color([0.51, 0.33, 0.19].into());
            let tab_label_border_color = [0.0, 0.0, 1.0].into();
            let text_color = Color::WHITE;

            Style {
                tab_label_background,
                tab_label_border_color,
                text_color,
                icon_color: text_color,
                ..self.active(is_selected)
            }
        }
    }
}

mod blue {
    use iced::{Background, Color};
    use iced_aw::tabs::{self, Style};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = if is_selected {
                Background::Color([0.41, 0.67, 0.82].into())
            } else {
                Background::Color([0.92, 0.97, 1.0].into())
            };

            let tab_label_border_color = if is_selected {
                [0.41, 0.67, 0.82].into()
            } else {
                [0.92, 0.97, 1.0].into()
            };

            let text_color = if is_selected {
                Color::WHITE
            } else {
                Color::BLACK
            };

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background,
                tab_label_border_color,
                tab_label_border_width: 1.0,
                icon_color: text_color,
                text_color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = Background::Color([0.21, 0.47, 0.62].into());
            let tab_label_border_color = [0.0, 0.0, 1.0].into();
            let text_color = Color::WHITE;

            Style {
                tab_label_background,
                tab_label_border_color,
                text_color,
                icon_color: text_color,
                ..self.active(is_selected)
            }
        }
    }
}

mod green {
    use iced::{Background, Color};
    use iced_aw::tabs::{self, Style};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = if is_selected {
                Background::Color([0.52, 0.65, 0.4].into())
            } else {
                Background::Color([1.0, 1.0, 0.87].into())
            };

            let tab_label_border_color = if is_selected {
                [0.52, 0.65, 0.4].into()
            } else {
                [1.0, 1.0, 0.87].into()
            };

            let text_color = if is_selected {
                Color::WHITE
            } else {
                Color::BLACK
            };

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background,
                tab_label_border_color,
                tab_label_border_width: 1.0,
                icon_color: text_color,
                text_color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = Background::Color([0.32, 0.45, 0.2].into());
            let tab_label_border_color = [0.0, 0.0, 1.0].into();
            let text_color = Color::WHITE;

            Style {
                tab_label_background,
                tab_label_border_color,
                text_color,
                icon_color: text_color,
                ..self.active(is_selected)
            }
        }
    }
}

mod purple {
    use iced::{Background, Color};
    use iced_aw::tabs::{self, Style};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = if is_selected {
                Background::Color([0.6, 0.50, 0.7].into())
            } else {
                Background::Color([0.9, 0.86, 0.94].into())
            };

            let tab_label_border_color = if is_selected {
                [0.6, 0.50, 0.7].into()
            } else {
                [0.9, 0.86, 0.94].into()
            };

            let text_color = if is_selected {
                Color::WHITE
            } else {
                Color::BLACK
            };

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background,
                tab_label_border_color,
                tab_label_border_width: 1.0,
                icon_color: text_color,
                text_color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = Background::Color([0.4, 0.30, 0.5].into());
            let tab_label_border_color = [0.0, 0.0, 1.0].into();
            let text_color = Color::WHITE;

            Style {
                tab_label_background,
                tab_label_border_color,
                text_color,
                icon_color: text_color,
                ..self.active(is_selected)
            }
        }
    }
}
