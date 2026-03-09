use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::models::ViewMode;
use crate::ui::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    let mode_label = match app.view_mode {
        ViewMode::Paginated => "PAGE",
        ViewMode::Scroll => "SCROLL",
    };

    let bookmark_indicator = if let Some(_ayah) = app.current_ayah() {
        "🔖"
    } else {
        ""
    };

    let downloaded = app
        .surahs
        .iter()
        .filter(|s| s.status == crate::core::models::SurahStatus::Downloaded)
        .count();

    let line = Line::from(vec![
        Span::styled(
            format!(" {} ", mode_label),
            Style::default()
                .bg(theme.highlight)
                .fg(theme.popup_bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {} ", app.status_message),
            Style::default().fg(theme.status_bar_fg),
        ),
        Span::styled(
            format!(" {} ", bookmark_indicator),
            Style::default().fg(theme.highlight),
        ),
        Span::styled(
            format!(" 💾 {}/114 ", downloaded),
            Style::default().fg(theme.verse_number),
        ),
        Span::styled(
            " ? help ",
            Style::default().fg(theme.status_bar_fg),
        ),
    ]);

    let bar = Paragraph::new(line)
        .style(Style::default().bg(theme.status_bar_bg));

    frame.render_widget(bar, area);
}
