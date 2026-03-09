use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IconStyle {
    NerdFont,
    Emoji,
}

impl Default for IconStyle {
    fn default() -> Self {
        IconStyle::NerdFont
    }
}

pub struct Icons;

impl Icons {
    pub fn not_downloaded(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f0ac} ",
            IconStyle::Emoji => "🌐",
        }
    }

    pub fn downloaded(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f0c7} ",
            IconStyle::Emoji => "💾",
        }
    }

    pub fn downloading(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f110} ",
            IconStyle::Emoji => "⏳",
        }
    }

    pub fn failed(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f00d} ",
            IconStyle::Emoji => "❌",
        }
    }

    pub fn bookmark(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f02e}",
            IconStyle::Emoji => "🔖",
        }
    }

    pub fn help(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f059}",
            IconStyle::Emoji => "?",
        }
    }

    pub fn surah_list(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f03a}",
            IconStyle::Emoji => "☰",
        }
    }

    pub fn mode_paginated(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f15c}",
            IconStyle::Emoji => "📄",
        }
    }

    pub fn mode_scroll(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f0dc}",
            IconStyle::Emoji => "📜",
        }
    }

    pub fn play(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f04b}",
            IconStyle::Emoji => "▶",
        }
    }

    pub fn pause(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f04c}",
            IconStyle::Emoji => "⏸",
        }
    }

    pub fn mosque(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f279}",
            IconStyle::Emoji => "🕌",
        }
    }

    pub fn arrow_right(style: &IconStyle) -> &'static str {
        match style {
            IconStyle::NerdFont => "\u{f054}",
            IconStyle::Emoji => "▸",
        }
    }
}
