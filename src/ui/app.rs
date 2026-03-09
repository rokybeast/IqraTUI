use crate::config::AppConfig;
use crate::core::models::{Ayah, Bookmark, Popup, Surah, ViewMode};
use crate::theme::Theme;

pub struct App {
    pub running: bool,
    pub config: AppConfig,
    pub theme: Theme,
    pub surahs: Vec<Surah>,
    pub current_surah_index: usize,
    pub current_ayah_index: usize,
    pub current_ayahs: Vec<Ayah>,
    pub view_mode: ViewMode,
    pub popup: Popup,
    pub show_arabic: bool,
    pub show_romanized: bool,
    pub show_english: bool,
    pub status_message: String,
    pub surah_list_index: usize,
    pub bookmark_list_index: usize,
    pub bookmarks: Vec<Bookmark>,
    pub scroll_offset: u16,
    pub loading: bool,
    pub surah_search: String,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let theme = Theme::from_type(&config.theme);
        let view_mode = config.default_view_mode();
        let show_arabic = config.show_arabic;
        let show_romanized = config.show_romanized;
        let show_english = config.show_english;

        Self {
            running: true,
            config,
            theme,
            surahs: Vec::new(),
            current_surah_index: 0,
            current_ayah_index: 0,
            current_ayahs: Vec::new(),
            view_mode,
            popup: Popup::None,
            show_arabic,
            show_romanized,
            show_english,
            status_message: String::from("Loading..."),
            surah_list_index: 0,
            bookmark_list_index: 0,
            bookmarks: Vec::new(),
            scroll_offset: 0,
            loading: false,
            surah_search: String::new(),
        }
    }

    pub fn current_surah(&self) -> Option<&Surah> {
        self.surahs.get(self.current_surah_index)
    }

    pub fn current_ayah(&self) -> Option<&Ayah> {
        self.current_ayahs.get(self.current_ayah_index)
    }

    pub fn total_ayahs(&self) -> usize {
        self.current_ayahs.len()
    }

    pub fn next_ayah(&mut self) {
        if self.current_ayah_index + 1 < self.current_ayahs.len() {
            self.current_ayah_index += 1;
        }
    }

    pub fn prev_ayah(&mut self) {
        if self.current_ayah_index > 0 {
            self.current_ayah_index -= 1;
        }
    }

    pub fn first_ayah(&mut self) {
        self.current_ayah_index = 0;
    }

    pub fn last_ayah(&mut self) {
        if !self.current_ayahs.is_empty() {
            self.current_ayah_index = self.current_ayahs.len() - 1;
        }
    }

    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Paginated => ViewMode::Scroll,
            ViewMode::Scroll => ViewMode::Paginated,
        };
        self.scroll_offset = 0;
    }

    pub fn filtered_surahs(&self) -> Vec<(usize, &Surah)> {
        if self.surah_search.is_empty() {
            self.surahs.iter().enumerate().collect()
        } else {
            let search = self.surah_search.to_lowercase();
            self.surahs
                .iter()
                .enumerate()
                .filter(|(_, s)| {
                    s.name_en.to_lowercase().contains(&search)
                        || s.id.to_string().contains(&search)
                })
                .collect()
        }
    }
}
