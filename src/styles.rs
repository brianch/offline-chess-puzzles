use iced::overlay::menu;

use iced::theme::Palette;
use iced::widget::{button, container, pick_list};
use iced::{Border, Color};
use iced_aw::style::tab_bar;

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        iced::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    };
}

pub const SELECTED_LIGHT_SQUARE: iced::Color = rgb!(205, 210, 106);
pub const SELECTED_DARK_SQUARE: iced::Color = rgb!(170, 162, 58);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PieceTheme {
    Cburnett,
    Alpha,
    Merida,
    California,
    Cardinal,
    Governor,
    Dubrovny,
    Gioco,
    Icpieces,
    Maestro,
    Staunty,
    Tatiana,
    FontAlpha,
}

impl PieceTheme {
    pub const ALL: [PieceTheme; 13] = [
        PieceTheme::Cburnett,
        PieceTheme::Alpha,
        PieceTheme::Merida,
        PieceTheme::California,
        PieceTheme::Cardinal,
        PieceTheme::Governor,
        PieceTheme::Dubrovny,
        PieceTheme::Gioco,
        PieceTheme::Icpieces,
        PieceTheme::Maestro,
        PieceTheme::Staunty,
        PieceTheme::Tatiana,
        PieceTheme::FontAlpha,
    ];
}

impl std::fmt::Display for PieceTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceTheme::Alpha => "alpha",
                PieceTheme::Merida => "merida",
                PieceTheme::California => "california",
                PieceTheme::Cardinal => "cardinal",
                PieceTheme::Governor => "governor",
                PieceTheme::Dubrovny => "dubrovny",
                PieceTheme::Gioco => "gioco",
                PieceTheme::Icpieces => "icpieces",
                PieceTheme::Maestro => "maestro",
                PieceTheme::Staunty => "staunty",
                PieceTheme::Tatiana => "tatiana",
                PieceTheme::FontAlpha => "Paper - chess alpha",
                _ => "cburnett",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BoardTheme {
    #[default]
    Blue,
    Green,
    Brown,
    Purple,
    Red,
    Grey,
    MonochromeGrey,
    BlueDark,
    GreenDark,
    BrownDark,
    PurpleDark,
    RedDark,
    GreyDark,
    MonochromeGreyDark,
    Trans,
    Enby,
}

impl BoardTheme {
    pub fn palette(&self) -> OCPPalette {
        match self {
            Self::Blue => OCPPalette::BLUE,
            Self::Green => OCPPalette::GREEN,
            Self::Brown => OCPPalette::BROWN,
            Self::Purple => OCPPalette::PURPLE,
            Self::Red => OCPPalette::RED,
            Self::Grey => OCPPalette::GREY,
            Self::MonochromeGrey => OCPPalette::MONOCHROME_GREY,
            Self::BlueDark => OCPPalette::BLUE_DARK,
            Self::GreenDark => OCPPalette::GREEN_DARK,
            Self::BrownDark => OCPPalette::BROWN_DARK,
            Self::PurpleDark => OCPPalette::PURPLE_DARK,
            Self::RedDark => OCPPalette::RED_DARK,
            Self::GreyDark => OCPPalette::GREY_DARK,
            Self::MonochromeGreyDark => OCPPalette::MONOCHROME_GREY_DARK,
            Self::Trans => OCPPalette::TRANS,
            Self::Enby => OCPPalette::ENBY,
        }
    }
    pub const ALL: [BoardTheme; 16] = [
        BoardTheme::Blue,
        BoardTheme::Green,
        BoardTheme::Brown,
        BoardTheme::Purple,
        BoardTheme::Red,
        BoardTheme::Grey,
        BoardTheme::MonochromeGrey,
        BoardTheme::BlueDark,
        BoardTheme::GreenDark,
        BoardTheme::BrownDark,
        BoardTheme::PurpleDark,
        BoardTheme::RedDark,
        BoardTheme::GreyDark,
        BoardTheme::MonochromeGreyDark,
        BoardTheme::Trans,
        BoardTheme::Enby,
    ];
}

impl std::fmt::Display for BoardTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoardTheme::Blue => "Blue",
                BoardTheme::Green => "Green",
                BoardTheme::Brown => "Brown",
                BoardTheme::Purple => "Purple",
                BoardTheme::Red => "Red",
                BoardTheme::Grey => "Grey",
                BoardTheme::MonochromeGrey => "Monochrome Grey",
                BoardTheme::BlueDark => "Blue - Dark Mode",
                BoardTheme::GreenDark => "Green - Dark Mode",
                BoardTheme::BrownDark => "Brown - Dark Mode",
                BoardTheme::PurpleDark => "Purple - Dark Mode",
                BoardTheme::RedDark => "Red - Dark Mode",
                BoardTheme::GreyDark => "Grey - Dark Mode",
                BoardTheme::MonochromeGreyDark => "Monochrome Grey - Dark",
                BoardTheme::Trans => "Trans colors",
                BoardTheme::Enby => "NB colors",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OCPTheme {
    #[default]
    Blue,
    Green,
    Brown,
    Purple,
    Grey,
    MonochromeGrey,
    BlueDark,
    GreenDark,
    BrownDark,
    PurpleDark,
    GreyDark,
    MonochromeGreyDark,
    Trans,
}

impl std::fmt::Display for OCPTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OCPTheme::Blue => "Blue",
                OCPTheme::Green => "Green",
                OCPTheme::Brown => "Brown",
                OCPTheme::Purple => "Purple",
                OCPTheme::Grey => "Grey",
                OCPTheme::MonochromeGrey => "Monochrome Grey",
                OCPTheme::BlueDark => "Blue - Dark Mode",
                OCPTheme::GreenDark => "Green - Dark Mode",
                OCPTheme::BrownDark => "Brown - Dark Mode",
                OCPTheme::PurpleDark => "Purple - Dark Mode",
                OCPTheme::GreyDark => "Grey - Dark Mode",
                OCPTheme::MonochromeGreyDark => "Monochrome Grey - Dark",
                OCPTheme::Trans => "Trans",
            }
        )
    }
}

pub type ChessBtn = fn(&iced::Theme, iced::widget::button::Status) -> button::Style;
pub fn btn_style_light_square(theme: &iced::Theme, _status: iced::widget::button::Status) -> button::Style {
    let palette = theme.palette();
    button::Style {
        background: Some(iced::Background::Color(palette.primary)),
        text_color: rgb!(45., 45., 45.),
        ..Default::default()
    }
}

pub fn btn_style_dark_square(theme: &iced::Theme, _status: iced::widget::button::Status) -> button::Style {
    let palette = theme.palette();
    button::Style {
        background: Some(iced::Background::Color(palette.success)),
        text_color: rgb!(45., 45., 45.),
        ..Default::default()
    }
}

pub fn btn_style_selected_light_square(_theme: &iced::Theme, _status: iced::widget::button::Status) -> button::Style {
    //let palette = theme.extended_palette();
    button::Style {
        //background: Some(iced::Background::Color(palette.primary.strong.color)),
        background: Some(iced::Background::Color(SELECTED_LIGHT_SQUARE)),
        text_color: rgb!(45., 45., 45.),
        ..Default::default()
    }
}

pub fn btn_style_selected_dark_square(_theme: &iced::Theme, _status: iced::widget::button::Status) -> button::Style {
    //let palette = theme.extended_palette();
    button::Style {
        //background: Some(iced::Background::Color(palette.success.weak.color)),
        background: Some(iced::Background::Color(SELECTED_DARK_SQUARE)),
        text_color: rgb!(45., 45., 45.),
        ..Default::default()
    }
}

pub fn btn_style_paper(_theme: &iced::Theme, _status: iced::widget::button::Status) -> button::Style {
    //let palette = theme.palette();
    button::Style {
        background: Some(iced::Background::Color(rgb!(245., 245., 245.))),
        text_color: rgb!(45., 45., 45.),
        border: Border {
            color: iced::Color::BLACK,
            width: 0.,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub type ChessboardContainer = fn(&iced::Theme) -> container::Style;
pub fn container_style_light_square(theme: &iced::Theme) -> container::Style {
    let palette = theme.palette();
    container::Style {
        background: Some(iced::Background::Color(palette.primary)),
        text_color: Some(rgb!(45., 45., 45.)),
        ..Default::default()
    }
}

pub fn container_style_dark_square(theme: &iced::Theme) -> container::Style {
    let palette = theme.palette();
    container::Style {
        background: Some(iced::Background::Color(palette.success)),
        text_color: Some(rgb!(45., 45., 45.)),
        ..Default::default()
    }
}

pub fn container_style_selected_light_square(_theme: &iced::Theme) -> container::Style {
    //let palette = theme.extended_palette();
    container::Style {
        //background: Some(iced::Background::Color(palette.primary.strong.color)),
        background: Some(iced::Background::Color(SELECTED_LIGHT_SQUARE)),
        text_color: Some(rgb!(45., 45., 45.)),
        ..Default::default()
    }
}

pub fn container_style_selected_dark_square(_theme: &iced::Theme) -> container::Style {
    //let palette = theme.extended_palette();
    container::Style {
        //background: Some(iced::Background::Color(palette.success.weak.color)),
        background: Some(iced::Background::Color(SELECTED_DARK_SQUARE)),
        text_color: Some(rgb!(45., 45., 45.)),
        ..Default::default()
    }
}

pub fn _container_style_paper(_theme: &iced::Theme) -> container::Style {
    //let palette = theme.palette();
    container::Style {
        background: Some(iced::Background::Color(rgb!(245., 245., 245.))),
        text_color: Some(rgb!(45., 45., 45.)),
        border: Border {
            color: iced::Color::BLACK,
            width: 0.,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn tab_style(theme: &iced::Theme, status: iced_aw::style::Status) -> tab_bar::Style {
    let palette = theme.extended_palette();
    match status {
        iced_aw::style::Status::Active => tab_bar::Style {
            tab_label_background: iced::Background::Color(palette.success.base.color),
            background: Some(iced::Background::Color(palette.success.base.color)),
            text_color: Color::WHITE,
            ..Default::default()
        },
        iced_aw::style::Status::Selected =>  tab_bar::Style {
            tab_label_background: iced::Background::Color(palette.primary.base.color),
            background: Some(iced::Background::Color(palette.primary.base.color)),
            text_color: Color::WHITE,
            ..Default::default()
        },
        iced_aw::style::Status::Focused => tab_bar::Style {
            tab_label_background: iced::Background::Color(palette.primary.base.color),
            background: Some(iced::Background::Color(palette.primary.base.color)),
            text_color: Color::WHITE,
            ..Default::default()
        },
        iced_aw::style::Status::Hovered => tab_bar::Style {
            tab_label_background: iced::Background::Color(palette.success.strong.color),
            background: Some(iced::Background::Color(palette.success.strong.color)),
            text_color: Color::WHITE,
            ..Default::default()
        },
        iced_aw::style::Status::Pressed => tab_bar::Style {
            tab_label_background: iced::Background::Color(palette.success.weak.color),
            background: Some(iced::Background::Color(palette.success.weak.color)),
            text_color: Color::WHITE,
            ..Default::default()
        },
        _ => tab_bar::Style {
            tab_label_background: iced::Background::Color(palette.primary.base.color),
            background: Some(iced::Background::Color(palette.primary.base.color)),
            text_color: rgb!(45., 45., 45.),
            ..Default::default()
        }
    }
}

pub fn pick_list_style(theme: &iced::Theme, _status: iced::widget::pick_list::Status) -> pick_list::Style {
    let palette = theme.extended_palette();

    pick_list::Style {
        text_color: palette.danger.base.color,
        placeholder_color: palette.success.weak.color,
        handle_color: palette.success.strong.color,
        background: iced::Background::Color(palette.primary.base.color),
        border: Border {
            color: palette.success.strong.color,
            width: 1.,
            radius:  0.3.into(),
        }
    }
}

pub fn menu_style(theme: &iced::Theme) -> menu::Style {
    let palette = theme.extended_palette();

    menu::Style {
        background: iced::Background::Color(palette.primary.base.color),
        border: Border {
            color: palette.success.strong.color,
            width: 1.,
            radius:  0.3.into(),
        },
        selected_background: iced::Background::Color(palette.success.base.color),
        selected_text_color: iced::Color::WHITE,
        text_color: rgb!(45., 45., 45.),
        shadow: iced::Shadow::default(),
    }
}

/// Offline Chess Puzzles Palette
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OCPPalette {
    pub container_bg: Color,
    pub simple_text: Color,
    pub label_selected: Color,
    pub light_square: Color,
    pub dark_square: Color,
    pub selected_light_square: Color,
    pub selected_dark_square: Color,
    pub tab_label: Color,
}

impl Into<Palette> for OCPPalette {
    fn into(self) -> Palette {
        Palette {
            background: self.container_bg,
            text: self.simple_text,
            primary: self.light_square,
            success: self.dark_square,
            danger: self.tab_label,
            warning: self.tab_label,
        }
    }
}

impl OCPPalette {
    pub const BLUE: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(235.0, 249.0, 255),
        dark_square: rgb!(110.0, 174.0, 213.0),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };

    pub const BLUE_DARK: Self = Self {
        container_bg: rgb!(70., 99., 117.),
        light_square: rgb!(235.0, 249.0, 255),
        dark_square: rgb!(110.0, 174.0, 213.0),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const RED: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(249.0, 234.0, 246),
        dark_square: rgb!(230.0, 133.0, 141.0),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };

    pub const RED_DARK: Self = Self {
        container_bg: rgb!(98.0, 64.0, 64.0),
        light_square: rgb!(249.0, 234.0, 246),
        dark_square: rgb!(230.0, 133.0, 141.0),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const GREEN_DARK: Self = Self {
        container_bg: rgb!(87., 99., 76.),
        light_square: rgb!(238.0, 240.0, 203.0),
        dark_square: rgb!(136.0, 161.0, 111.0),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const BROWN_DARK: Self = Self {
        container_bg: rgb!(116., 99., 86.),
        light_square: rgb!(241., 221., 186.),
        dark_square: rgb!(186., 142., 107.),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const PURPLE_DARK: Self = Self {
        container_bg: rgb!(89., 77., 101.),
        light_square: rgb!(233., 223., 242.),
        dark_square: rgb!(162., 136., 188.),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const MONOCHROME_GREY_DARK: Self = Self {
        container_bg: rgb!(90., 90., 90.),
        light_square: rgb!(235., 235., 235.),
        dark_square: rgb!(155., 155., 155.),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const GREY_DARK: Self = Self {
        container_bg: rgb!(71., 86., 92.),
        light_square: rgb!(222., 227., 230.),
        dark_square: rgb!(140., 162., 173.),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const GREEN: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(238.0, 240.0, 203.0),
        dark_square: rgb!(136.0, 161.0, 111.0),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const BROWN: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(241., 221., 186.),
        dark_square: rgb!(186., 142., 107.),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const PURPLE: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(233., 223., 242.),
        dark_square: rgb!(162., 136., 188.),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const MONOCHROME_GREY: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(235., 235., 235.),
        dark_square: rgb!(155., 155., 155.),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const GREY: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(222., 227., 230.),
        dark_square: rgb!(140., 162., 173.),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const TRANS: Self = Self {
        container_bg: rgb!(154.0, 223.0, 250.0),
        light_square: rgb!(252.0, 252.0, 252.0),
        dark_square: rgb!(245.0, 183.0, 195.0),
        selected_light_square: rgb!(205, 210, 106),
        selected_dark_square: rgb!(170, 162, 58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const ENBY: Self = Self {
        container_bg: rgb!(142.0, 101.0, 161.0),
        light_square: rgb!(246.0, 246.0, 246.0),
        dark_square: rgb!(219.0, 190.0, 53.0),
        selected_light_square: rgb!(172.0, 131.0, 191.0),
        selected_dark_square: rgb!(132.0, 91.0, 151.0),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
}
