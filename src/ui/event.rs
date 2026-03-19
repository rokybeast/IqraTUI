use std::sync::Arc;

use anyhow::Result;
use crossterm::event::KeyCode;
use tokio::sync::Mutex;

use crate::core::models::Popup;
use crate::core::service::QuranService;
use crate::theme::{Theme, ThemeType};
use crate::tts::{TtsPlayer, TtsState};
use crate::ui::app::App;

pub enum Action {
    Quit,
    NextAyah,
    PrevAyah,
    FirstAyah,
    LastAyah,
    PageDown,
    PageUp,
    ScrollDown,
    ScrollUp,
    Download,
    Delete,
    Bookmark,
    OpenSurahList,
    ToggleScrollMode,
    ToggleArabic,
    ToggleRomanized,
    ToggleEnglish,
    Help,
    OpenBookmarkList,
    CycleTheme,
    TtsToggle,
    Confirm,
    Escape,
    SearchChar(char),
    SearchBackspace,
    None,
}

pub fn map_key_to_action(key: KeyCode, app: &App) -> Action {
    let kb = &app.config.keybindings;

    if app.popup != Popup::None {
        return match key {
            KeyCode::Esc => Action::Escape,
            KeyCode::Enter => Action::Confirm,
            KeyCode::Up => Action::ScrollUp,
            KeyCode::Down => Action::ScrollDown,
            KeyCode::Char(c) => {
                if app.popup == Popup::SurahList {
                    Action::SearchChar(c)
                } else if kb.matches(key, &kb.tts_toggle) {
                    Action::TtsToggle
                } else if kb.matches(key, &kb.quit) {
                    Action::Escape
                } else {
                    Action::None
                }
            }
            KeyCode::Backspace => Action::SearchBackspace,
            _ => Action::None,
        };
    }

    if kb.matches(key, &kb.quit) {
        Action::Quit
    } else if kb.matches(key, &kb.next_ayah) {
        Action::NextAyah
    } else if kb.matches(key, &kb.prev_ayah) {
        Action::PrevAyah
    } else if kb.matches(key, &kb.download) {
        Action::Download
    } else if kb.matches(key, &kb.delete) {
        Action::Delete
    } else if kb.matches(key, &kb.bookmark) {
        Action::Bookmark
    } else if kb.matches(key, &kb.surah_list) {
        Action::OpenSurahList
    } else if kb.matches(key, &kb.scroll_mode) {
        Action::ToggleScrollMode
    } else if kb.matches(key, &kb.toggle_arabic) {
        Action::ToggleArabic
    } else if kb.matches(key, &kb.toggle_romanized) {
        Action::ToggleRomanized
    } else if kb.matches(key, &kb.toggle_english) {
        Action::ToggleEnglish
    } else if kb.matches(key, &kb.help) {
        Action::Help
    } else if kb.matches(key, &kb.bookmark_list) {
        Action::OpenBookmarkList
    } else if kb.matches(key, &kb.cycle_theme) {
        Action::CycleTheme
    } else if kb.matches(key, &kb.tts_toggle) {
        Action::TtsToggle
    } else {
        match key {
            KeyCode::Home => Action::FirstAyah,
            KeyCode::End => Action::LastAyah,
            KeyCode::PageDown => Action::PageDown,
            KeyCode::PageUp => Action::PageUp,
            KeyCode::Up => Action::ScrollUp,
            KeyCode::Down => Action::ScrollDown,
            _ => Action::None,
        }
    }
}

pub async fn handle_action(
    action: Action,
    app: &mut App,
    service: &Arc<Mutex<QuranService>>,
    tts: &mut TtsPlayer,
) -> Result<()> {
    match action {
        Action::Quit => {
            app.running = false;
        }
        Action::NextAyah => {
            app.next_ayah();
            update_status(app);
        }
        Action::PrevAyah => {
            app.prev_ayah();
            update_status(app);
        }
        Action::FirstAyah => {
            app.first_ayah();
            update_status(app);
        }
        Action::LastAyah => {
            app.last_ayah();
            update_status(app);
        }
        Action::PageDown => {
            for _ in 0..10 {
                app.next_ayah();
            }
            update_status(app);
        }
        Action::PageUp => {
            for _ in 0..10 {
                app.prev_ayah();
            }
            update_status(app);
        }
        Action::ScrollDown => {
            if app.popup == Popup::SurahList {
                let filtered = app.filtered_surahs();
                if app.surah_list_index + 1 < filtered.len() {
                    app.surah_list_index += 1;
                }
            } else if app.popup == Popup::BookmarkList {
                if app.bookmark_list_index + 1 < app.bookmarks.len() {
                    app.bookmark_list_index += 1;
                }
            } else {
                app.scroll_offset = app.scroll_offset.saturating_add(1);
            }
        }
        Action::ScrollUp => {
            if app.popup == Popup::SurahList {
                app.surah_list_index = app.surah_list_index.saturating_sub(1);
            } else if app.popup == Popup::BookmarkList {
                app.bookmark_list_index = app.bookmark_list_index.saturating_sub(1);
            } else {
                app.scroll_offset = app.scroll_offset.saturating_sub(1);
            }
        }
        Action::Download => {
            if let Some(surah) = app.current_surah() {
                let surah_id = surah.id;
                let surah_name = surah.name_en.clone();
                app.status_message = format!("Downloading {}...", surah_name);
                app.loading = true;
                let svc = service.lock().await;
                match svc.download_surah(surah_id).await {
                    Ok(_) => {
                        let ayahs = svc.get_all_ayahs(surah_id)?;
                        app.current_ayahs = ayahs;
                        app.current_ayah_index = 0;
                        app.surahs = svc.get_surah_list()?;
                        app.status_message = format!("Downloaded {}", surah_name);
                    }
                    Err(e) => {
                        app.status_message = format!("Download failed: {}", e);
                    }
                }
                app.loading = false;
            }
        }
        Action::Delete => {
            if let Some(surah) = app.current_surah() {
                let surah_id = surah.id;
                let surah_name = surah.name_en.clone();
                let svc = service.lock().await;
                svc.delete_surah(surah_id)?;
                app.current_ayahs.clear();
                app.current_ayah_index = 0;
                app.surahs = svc.get_surah_list()?;
                app.status_message = format!("Deleted {}", surah_name);
            }
        }
        Action::Bookmark => {
            if let Some(ayah) = app.current_ayah() {
                let sid = ayah.surah_id;
                let anum = ayah.ayah_number;
                let svc = service.lock().await;
                svc.toggle_bookmark(sid, anum)?;
                if svc.is_bookmarked(sid, anum) {
                    app.status_message = format!("Bookmarked {}:{}", sid, anum);
                } else {
                    app.status_message = format!("Removed bookmark {}:{}", sid, anum);
                }
            }
        }
        Action::OpenSurahList => {
            app.popup = Popup::SurahList;
            app.surah_list_index = app.current_surah_index;
            app.surah_search.clear();
        }
        Action::ToggleScrollMode => {
            app.toggle_view_mode();
            let mode_name = match app.view_mode {
                crate::core::models::ViewMode::Paginated => "Paginated",
                crate::core::models::ViewMode::Scroll => "Scroll",
            };
            app.status_message = format!("Mode: {}", mode_name);
        }
        Action::ToggleArabic => {
            app.show_arabic = !app.show_arabic;
        }
        Action::ToggleRomanized => {
            app.show_romanized = !app.show_romanized;
        }
        Action::ToggleEnglish => {
            app.show_english = !app.show_english;
        }
        Action::Help => {
            app.popup = if app.popup == Popup::Help {
                Popup::None
            } else {
                Popup::Help
            };
        }
        Action::CycleTheme => {
            let next = match app.config.theme {
                ThemeType::Dark => ThemeType::Light,
                ThemeType::Light => ThemeType::Terminal,
                ThemeType::Terminal => ThemeType::Dark,
            };
            app.config.theme = next.clone();
            app.theme = Theme::from_type(&next);
            let name = match next {
                ThemeType::Dark => "Dark",
                ThemeType::Light => "Light",
                ThemeType::Terminal => "Terminal",
            };
            app.status_message = format!("Theme: {}", name);
        }
        Action::TtsToggle => {
            match tts.state {
                TtsState::Idle | TtsState::Loading => {
                    if let Some(ayah) = app.current_ayah() {
                        let sid = ayah.surah_id;
                        let anum = ayah.ayah_number;
                        app.status_message = format!("Loading audio {}:{}...", sid, anum);
                        match tts.play(sid, anum).await {
                            Ok(_) => {
                                app.status_message = format!("Playing {}:{}", sid, anum);
                            }
                            Err(e) => {
                                app.status_message = format!("TTS error: {}", e);
                            }
                        }
                    }
                }
                TtsState::Playing => {
                    tts.pause();
                    app.status_message = String::from("Paused");
                }
                TtsState::Paused => {
                    tts.resume();
                    app.status_message = String::from("Resumed");
                }
            }
        }
        Action::OpenBookmarkList => {
            let svc = service.lock().await;
            app.bookmarks = svc.get_bookmarks()?;
            app.bookmark_list_index = 0;
            app.popup = Popup::BookmarkList;
        }
        Action::Confirm => {
            match app.popup {
                Popup::SurahList => {
                    let filtered = app.filtered_surahs();
                    if let Some(&(original_index, _)) = filtered.get(app.surah_list_index) {
                        app.current_surah_index = original_index;
                        app.popup = Popup::None;
                        app.surah_search.clear();
                        let surah = &app.surahs[original_index];
                        let surah_id = surah.id;
                        let svc = service.lock().await;
                        if svc.is_surah_downloaded(surah_id) {
                            app.current_ayahs = svc.get_all_ayahs(surah_id)?;
                        } else {
                            app.current_ayahs.clear();
                            app.status_message = format!(
                                "Surah not downloaded. Press 'd' to download."
                            );
                        }
                        app.current_ayah_index = 0;
                        app.scroll_offset = 0;
                        update_status(app);
                    }
                }
                Popup::BookmarkList => {
                    if let Some(bm) = app.bookmarks.get(app.bookmark_list_index) {
                        let sid = bm.surah_id;
                        let anum = bm.ayah_number;
                        app.popup = Popup::None;
                        if let Some(idx) = app.surahs.iter().position(|s| s.id == sid) {
                            app.current_surah_index = idx;
                            let svc = service.lock().await;
                            if svc.is_surah_downloaded(sid) {
                                app.current_ayahs = svc.get_all_ayahs(sid)?;
                                app.current_ayah_index =
                                    (anum as usize).saturating_sub(1);
                            }
                        }
                        update_status(app);
                    }
                }
                _ => {
                    app.popup = Popup::None;
                }
            }
        }
        Action::Escape => {
            app.popup = Popup::None;
            app.surah_search.clear();
        }
        Action::SearchChar(c) => {
            app.surah_search.push(c);
            app.surah_list_index = 0;
        }
        Action::SearchBackspace => {
            app.surah_search.pop();
            app.surah_list_index = 0;
        }
        Action::None => {}
    }
    Ok(())
}

fn update_status(app: &mut App) {
    if let Some(surah) = app.current_surah() {
        let name = surah.name_en.clone();
        let total = app.total_ayahs();
        let current = app.current_ayah_index + 1;
        app.status_message = format!("{} — Ayah {} / {}", name, current, total);
    }
}
