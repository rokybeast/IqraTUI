use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeType {
    Dark,
    Light,
    Terminal,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub arabic: Color,
    pub romanized: Color,
    pub english: Color,
    pub verse_number: Color,
    pub divider: Color,
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
    pub highlight: Color,
    pub error: Color,
    pub background: Color,
    pub foreground: Color,
    pub surah_name: Color,
    pub border: Color,
    pub popup_bg: Color,
    pub popup_fg: Color,
}

impl Theme {
    pub fn from_type(theme_type: &ThemeType) -> Self {
        match theme_type {
            ThemeType::Dark => Self::dark(),
            ThemeType::Light => Self::light(),
            ThemeType::Terminal => Self::terminal(),
        }
    }

    pub fn dark() -> Self {
        Self {
            arabic: Color::Rgb(220, 220, 170),
            romanized: Color::Rgb(150, 180, 220),
            english: Color::Rgb(200, 200, 200),
            verse_number: Color::Rgb(130, 170, 130),
            divider: Color::Rgb(60, 60, 60),
            status_bar_bg: Color::Rgb(30, 30, 40),
            status_bar_fg: Color::Rgb(180, 180, 190),
            highlight: Color::Rgb(100, 140, 200),
            error: Color::Rgb(220, 80, 80),
            background: Color::Rgb(15, 15, 20),
            foreground: Color::Rgb(210, 210, 215),
            surah_name: Color::Rgb(180, 150, 100),
            border: Color::Rgb(50, 50, 60),
            popup_bg: Color::Rgb(25, 25, 35),
            popup_fg: Color::Rgb(210, 210, 215),
        }
    }

    pub fn light() -> Self {
        Self {
            arabic: Color::Rgb(60, 60, 20),
            romanized: Color::Rgb(40, 80, 140),
            english: Color::Rgb(40, 40, 40),
            verse_number: Color::Rgb(60, 120, 60),
            divider: Color::Rgb(200, 200, 200),
            status_bar_bg: Color::Rgb(230, 230, 235),
            status_bar_fg: Color::Rgb(60, 60, 70),
            highlight: Color::Rgb(50, 100, 180),
            error: Color::Rgb(200, 50, 50),
            background: Color::Rgb(250, 250, 245),
            foreground: Color::Rgb(30, 30, 35),
            surah_name: Color::Rgb(140, 100, 50),
            border: Color::Rgb(180, 180, 190),
            popup_bg: Color::Rgb(245, 245, 240),
            popup_fg: Color::Rgb(30, 30, 35),
        }
    }

    pub fn terminal() -> Self {
        Self {
            arabic: Color::Yellow,
            romanized: Color::Cyan,
            english: Color::White,
            verse_number: Color::Green,
            divider: Color::DarkGray,
            status_bar_bg: Color::DarkGray,
            status_bar_fg: Color::White,
            highlight: Color::Blue,
            error: Color::Red,
            background: Color::Reset,
            foreground: Color::Reset,
            surah_name: Color::Magenta,
            border: Color::DarkGray,
            popup_bg: Color::Black,
            popup_fg: Color::White,
        }
    }
}
