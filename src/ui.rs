use crate::app::App;
use chrono::{TimeZone, Utc, Timelike};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main body
                Constraint::Length(3), // Footer
            ]
            .as_ref(),
        )
        .split(f.area());

    // Header
    let now = Utc::now();
    let hour = now.hour();
    let is_active = hour >= 6 && hour < 22; // Hardcoded for UI display consistency with default config

    let mode_str = if is_active { "Active (High Frequency)" } else { "Idle (Low Power)" };
    let mode_color = if is_active { Color::Green } else { Color::Yellow };

    let header_content = Line::from(vec![
        Span::styled(
            "LIVE NEWS TUI",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(mode_str, Style::default().fg(mode_color)),
        Span::raw(format!(" | Items: {} | Sources: {}", app.stats.0, app.stats.1)),
    ]);

    let header = Paragraph::new(header_content)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    if app.is_reading {
        draw_reading_view(f, app, chunks[1]);
    } else {
        draw_main_view(f, app, chunks[1]);
    }

    // Footer
    let footer_text = if app.is_reading {
        "Esc/q: Back"
    } else {
        "q: Quit | ⬆⬇/j k: Navigate | ⬅➡/h l: Category | Enter: Read"
    };
    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}

fn draw_main_view(f: &mut Frame, app: &mut App, area: Rect) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20), // Categories
                Constraint::Percentage(80), // News Feed
            ]
            .as_ref(),
        )
        .split(area);

    // Categories Sidebar
    let categories: Vec<ListItem> = app
        .categories
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let mut style = Style::default();
            if i == app.selected_category {
                style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
            }
            ListItem::new(Span::styled(*c, style))
        })
        .collect();

    let categories_list =
        List::new(categories).block(Block::default().title(" Categories ").borders(Borders::ALL));
    f.render_widget(categories_list, body_chunks[0]);

    // News Feed
    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let datetime = Utc.timestamp_opt(item.timestamp, 0).latest().unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap()).naive_local();
            let time_str = datetime.format("%H:%M:%S").to_string();

            let mut style = Style::default();
            if i == app.selected_item {
                style = style.bg(Color::DarkGray);
            }

            let content = Line::from(vec![
                Span::styled(
                    format!("[{}] ", time_str),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("[{}] ", item.source),
                    Style::default().fg(Color::Green),
                ),
                Span::raw(&item.title),
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let feed_title = format!(" {} - Live Feed ", app.categories[app.selected_category]);
    let news_list =
        List::new(items).block(Block::default().title(feed_title).borders(Borders::ALL));

    let mut state = ListState::default();
    state.select(Some(app.selected_item));
    f.render_stateful_widget(news_list, body_chunks[1], &mut state);
}

fn draw_reading_view(f: &mut Frame, app: &App, area: Rect) {
    let item = &app.items[app.selected_item];

    let datetime = Utc.timestamp_opt(item.timestamp, 0).latest().unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap()).naive_local();
    let time_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut text = vec![
        Line::from(Span::styled(
            &item.title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Source: ", Style::default().fg(Color::DarkGray)),
            Span::raw(&item.source),
            Span::raw(" | "),
            Span::styled("Time: ", Style::default().fg(Color::DarkGray)),
            Span::raw(time_str),
        ]),
        Line::from(vec![
            Span::styled("URL: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                &item.url,
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ]),
        Line::from(""),
    ];

    if let Some(desc) = &item.description {
        for line in desc.lines() {
            text.push(Line::from(line.to_string()));
        }
    } else {
        text.push(Line::from(Span::styled(
            "No description available.",
            Style::default().fg(Color::Red),
        )));
    }

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" Article Details ")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
