use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::core::models::SurahStatus;
use crate::icons::Icons;
use crate::ui::app::App;
use crate::ui::render::centered_rect;

pub fn draw(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = centered_rect(70, 70, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Surahs ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.popup_bg));

    let filtered = app.filtered_surahs();

    let mut lines: Vec<Line> = Vec::new();

    if !app.surah_search.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("  Search: ", Style::default().fg(theme.verse_number)),
            Span::styled(
                app.surah_search.clone(),
                Style::default()
                    .fg(theme.highlight)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(""));
    }

    for (display_idx, (_, surah)) in filtered.iter().enumerate() {
        let is_selected = display_idx == app.surah_list_index;

        let style = if is_selected {
            Style::default()
                .fg(theme.highlight)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.popup_fg)
        };

        let prefix = if is_selected {
            format!(" {} ", Icons::arrow_right(&app.config.icon_style))
        } else {
            "   ".to_string()
        };

        let status_icon = match surah.status {
            SurahStatus::NotDownloaded => Icons::not_downloaded(&app.config.icon_style),
            SurahStatus::Downloaded => Icons::downloaded(&app.config.icon_style),
            SurahStatus::Downloading => Icons::downloading(&app.config.icon_style),
            SurahStatus::Failed => Icons::failed(&app.config.icon_style),
        };

        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{} ", status_icon), style),
            Span::styled(
                format!("{:>3}. ", surah.id),
                Style::default().fg(theme.verse_number),
            ),
            Span::styled(format!("{} ", surah.name_en), style),
            Span::styled(
                format!("({}) ", surah.name_ar),
                Style::default().fg(theme.arabic),
            ),
            Span::styled(
                format!("[{} ayahs]", surah.total_ayahs),
                Style::default().fg(theme.verse_number),
            ),
        ]));
    }

    if filtered.is_empty() {
        lines.push(Line::styled(
            "  No matching surahs",
            Style::default().fg(theme.error),
        ));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}
