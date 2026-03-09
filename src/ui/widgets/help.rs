use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::ui::app::App;
use crate::ui::render::centered_rect;

pub fn draw(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = centered_rect(60, 70, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Help — Keybindings ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.popup_bg));

    let kb = &app.config.keybindings;

    let bindings = vec![
        ("←  / →", "Previous / Next Ayah"),
        ("↑  / ↓", "Scroll Up / Down"),
        ("Home / End", "First / Last Ayah"),
        ("PgUp / PgDn", "Jump 10 Ayahs"),
        (&kb.download, "Download current Surah"),
        (&kb.delete, "Delete downloaded Surah"),
        (&kb.bookmark, "Toggle Bookmark"),
        (&kb.surah_list, "Open Surah List"),
        (&kb.scroll_mode, "Toggle Scroll Mode"),
        (&kb.toggle_arabic, "Toggle Arabic"),
        (&kb.toggle_romanized, "Toggle Romanized"),
        (&kb.toggle_english, "Toggle English"),
        (&kb.bookmark_list, "Open Bookmark List"),
        (&kb.help, "Toggle Help"),
        (&kb.quit, "Quit"),
    ];

    let lines: Vec<Line> = bindings
        .iter()
        .map(|(key, desc)| {
            Line::from(vec![
                Span::styled(
                    format!("  {:>14}", key),
                    Style::default()
                        .fg(theme.highlight)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  {}", desc),
                    Style::default().fg(theme.popup_fg),
                ),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
