use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::core::models::{Popup, ViewMode};
use crate::ui::app::App;
use crate::ui::widgets::{help, status_bar, surah_list};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(frame.area());

    draw_header(frame, app, chunks[0]);

    match app.view_mode {
        ViewMode::Paginated => draw_paginated(frame, app, chunks[1]),
        ViewMode::Scroll => draw_scroll(frame, app, chunks[1]),
    }

    status_bar::draw(frame, app, chunks[2]);

    match app.popup {
        Popup::SurahList => surah_list::draw(frame, app),
        Popup::Help => help::draw(frame, app),
        Popup::BookmarkList => draw_bookmark_list(frame, app),
        Popup::None => {}
    }
}

fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    let (surah_name, ayah_info) = if let Some(surah) = app.current_surah() {
        let name = format!("  {}  {}", surah.name_ar, surah.name_en);
        let info = if !app.current_ayahs.is_empty() {
            format!(
                "{}:{}  [{} / {}]",
                surah.id,
                app.current_ayah_index + 1,
                app.current_ayah_index + 1,
                app.total_ayahs()
            )
        } else {
            String::from("Not downloaded")
        };
        (name, info)
    } else {
        (String::from("  Iqra"), String::from("Select a Surah"))
    };

    let header = Line::from(vec![
        Span::styled(
            surah_name,
            Style::default()
                .fg(theme.surah_name)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            ayah_info,
            Style::default().fg(theme.verse_number),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(theme.border));

    let header_widget = Paragraph::new(header)
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(header_widget, area);
}

fn draw_paginated(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    let ayah = match app.current_ayah() {
        Some(a) => a,
        None => {
            let msg = if app.current_surah().is_some() {
                "Press 'd' to download this Surah"
            } else {
                "Press 's' to select a Surah"
            };
            let p = Paragraph::new(msg)
                .style(Style::default().fg(theme.foreground))
                .alignment(Alignment::Center);
            frame.render_widget(p, area);
            return;
        }
    };

    let mut lines: Vec<Line> = Vec::new();

    if app.show_arabic {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            ayah.arabic.clone(),
            Style::default()
                .fg(theme.arabic)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        "─".repeat(area.width as usize),
        Style::default().fg(theme.divider),
    )));

    if app.show_romanized {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            ayah.romanized.clone(),
            Style::default()
                .fg(theme.romanized)
                .add_modifier(Modifier::ITALIC),
        )));
        lines.push(Line::from(""));
    }

    if app.show_english {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            ayah.english.clone(),
            Style::default().fg(theme.english),
        )));
        lines.push(Line::from(""));
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn draw_scroll(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    if app.current_ayahs.is_empty() {
        let msg = "Press 'd' to download this Surah";
        let p = Paragraph::new(msg)
            .style(Style::default().fg(theme.foreground))
            .alignment(Alignment::Center);
        frame.render_widget(p, area);
        return;
    }

    let mut lines: Vec<Line> = Vec::new();

    for ayah in &app.current_ayahs {
        let num_span = Span::styled(
            format!("[{}] ", ayah.ayah_number),
            Style::default().fg(theme.verse_number),
        );

        if app.show_arabic {
            lines.push(Line::from(vec![
                num_span.clone(),
                Span::styled(
                    ayah.arabic.clone(),
                    Style::default()
                        .fg(theme.arabic)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        if app.show_romanized {
            lines.push(Line::from(Span::styled(
                ayah.romanized.clone(),
                Style::default()
                    .fg(theme.romanized)
                    .add_modifier(Modifier::ITALIC),
            )));
        }

        if app.show_english {
            lines.push(Line::from(Span::styled(
                ayah.english.clone(),
                Style::default().fg(theme.english),
            )));
        }

        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(theme.divider),
        )));
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .scroll((app.scroll_offset, 0))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn draw_bookmark_list(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = centered_rect(60, 70, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Bookmarks ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.popup_bg));

    let items: Vec<Line> = app
        .bookmarks
        .iter()
        .enumerate()
        .map(|(i, bm)| {
            let surah_name = app
                .surahs
                .iter()
                .find(|s| s.id == bm.surah_id)
                .map(|s| s.name_en.as_str())
                .unwrap_or("Unknown");

            let style = if i == app.bookmark_list_index {
                Style::default()
                    .fg(theme.highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.popup_fg)
            };

            Line::styled(
                format!("  {} {}:{}", surah_name, bm.surah_id, bm.ayah_number),
                style,
            )
        })
        .collect();

    if items.is_empty() {
        let p = Paragraph::new("  No bookmarks yet")
            .block(block)
            .style(Style::default().fg(theme.popup_fg));
        frame.render_widget(p, area);
    } else {
        let p = Paragraph::new(items)
            .block(block);
        frame.render_widget(p, area);
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
