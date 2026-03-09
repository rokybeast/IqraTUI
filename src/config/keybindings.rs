use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Keybindings {
    #[serde(default = "default_quit")]
    pub quit: String,
    #[serde(default = "default_next_ayah")]
    pub next_ayah: String,
    #[serde(default = "default_prev_ayah")]
    pub prev_ayah: String,
    #[serde(default = "default_download")]
    pub download: String,
    #[serde(default = "default_delete")]
    pub delete: String,
    #[serde(default = "default_bookmark")]
    pub bookmark: String,
    #[serde(default = "default_surah_list")]
    pub surah_list: String,
    #[serde(default = "default_tts_toggle")]
    pub tts_toggle: String,
    #[serde(default = "default_scroll_mode")]
    pub scroll_mode: String,
    #[serde(default = "default_toggle_arabic")]
    pub toggle_arabic: String,
    #[serde(default = "default_toggle_romanized")]
    pub toggle_romanized: String,
    #[serde(default = "default_toggle_english")]
    pub toggle_english: String,
    #[serde(default = "default_help")]
    pub help: String,
    #[serde(default = "default_bookmark_list")]
    pub bookmark_list: String,
}

fn default_quit() -> String { "q".into() }
fn default_next_ayah() -> String { "Right".into() }
fn default_prev_ayah() -> String { "Left".into() }
fn default_download() -> String { "d".into() }
fn default_delete() -> String { "x".into() }
fn default_bookmark() -> String { "b".into() }
fn default_surah_list() -> String { "s".into() }
fn default_tts_toggle() -> String { "Space".into() }
fn default_scroll_mode() -> String { "m".into() }
fn default_toggle_arabic() -> String { "a".into() }
fn default_toggle_romanized() -> String { "r".into() }
fn default_toggle_english() -> String { "e".into() }
fn default_help() -> String { "?".into() }
fn default_bookmark_list() -> String { "B".into() }

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: default_quit(),
            next_ayah: default_next_ayah(),
            prev_ayah: default_prev_ayah(),
            download: default_download(),
            delete: default_delete(),
            bookmark: default_bookmark(),
            surah_list: default_surah_list(),
            tts_toggle: default_tts_toggle(),
            scroll_mode: default_scroll_mode(),
            toggle_arabic: default_toggle_arabic(),
            toggle_romanized: default_toggle_romanized(),
            toggle_english: default_toggle_english(),
            help: default_help(),
            bookmark_list: default_bookmark_list(),
        }
    }
}

impl Keybindings {
    pub fn matches(&self, key: KeyCode, binding: &str) -> bool {
        match key {
            KeyCode::Char(c) => {
                let s = c.to_string();
                s == binding
            }
            KeyCode::Right => binding == "Right",
            KeyCode::Left => binding == "Left",
            KeyCode::Up => binding == "Up",
            KeyCode::Down => binding == "Down",
            KeyCode::Home => binding == "Home",
            KeyCode::End => binding == "End",
            KeyCode::PageUp => binding == "PageUp",
            KeyCode::PageDown => binding == "PageDown",
            KeyCode::Enter => binding == "Enter",
            KeyCode::Esc => binding == "Esc",
            KeyCode::Tab => binding == "Tab",
            _ => false,
        }
    }
}
