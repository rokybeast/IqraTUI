use std::io;
use std::sync::Arc;

use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio::sync::Mutex;

use iqra_tui::api::quran::QuranApi;
use iqra_tui::config::AppConfig;
use iqra_tui::core::service::QuranService;
use iqra_tui::storage::db::Database;
use iqra_tui::ui::app::App;
use iqra_tui::ui::event::{handle_action, map_key_to_action};
use iqra_tui::ui::render;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load().unwrap_or_default();
    let db_path = Database::default_path();
    let db = Database::new(&db_path)?;
    let api = QuranApi::new();
    let service = Arc::new(Mutex::new(QuranService::new(db, api)));

    let mut app = App::new(config);

    {
        let svc = service.lock().await;
        match svc.get_surah_list() {
            Ok(surahs) if !surahs.is_empty() => {
                app.surahs = surahs;
                app.status_message = String::from("Press 's' to select a Surah");
            }
            _ => {
                app.status_message = String::from("Fetching surah list...");
                drop(svc);
                let svc = service.lock().await;
                match svc.fetch_and_cache_surah_list().await {
                    Ok(_) => {
                        app.surahs = svc.get_surah_list()?;
                        app.status_message = String::from("Press 's' to select a Surah");
                    }
                    Err(e) => {
                        app.status_message = format!("Failed to load surahs: {}", e);
                    }
                }
            }
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    while app.running {
        terminal.draw(|frame| {
            render::draw(frame, &app);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                let action = map_key_to_action(key.code, &app);
                handle_action(action, &mut app, &service).await?;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
