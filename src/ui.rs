use crate::app::App;
use chrono::{TimeZone, Timelike, Utc};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

pub fn draw(f: &mut Frame, app: &mut App) {
    // Determine dynamic layout based on search state
    let header_height = 3;
    let footer_height = 1;
    let search_height = if app.is_searching { 3 } else { 0 };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_height),
            Constraint::Length(search_height),
            Constraint::Min(0),
            Constraint::Length(footer_height),
        ])
        .split(f.area());

    // Header
    let now = Utc::now();
    let hour = now.hour();
    let is_active = hour >= 6 && hour < 22;

    let mode_str = if is_active { "ACTIVE" } else { "IDLE" };
    let mode_color = if is_active {
        Color::Green
    } else {
        Color::Yellow
    };

    let header_content = Line::from(vec![
        Span::styled(
            " LIVE NEWS TUI ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            format!(" {} ", mode_str),
            Style::default()
                .fg(Color::Black)
                .bg(mode_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("v{}", env!("CARGO_PKG_VERSION")),
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let header = Paragraph::new(header_content).block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Search Bar
    if app.is_searching {
        let search_block = Paragraph::new(format!(" > {}", app.search_query)).block(
            Block::default()
                .title(" Search (Enter/Esc to close) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
        f.render_widget(search_block, chunks[1]);
    }

    // Main Body
    if app.is_reading {
        draw_reading_view(f, app, chunks[2]);
    } else {
        draw_main_view(f, app, chunks[2]);
    }

    // Status Bar
    let mut status_spans = vec![
        Span::styled(
            format!(" [Items: {}]", app.stats.0),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(
            format!(" [Sources: {}]", app.stats.1),
            Style::default().fg(Color::Green),
        ),
    ];

    if !app.search_query.is_empty() {
        status_spans.push(Span::styled(
            format!(" [Filter: {}]", app.search_query),
            Style::default().fg(Color::Yellow),
        ));
    }

    status_spans.push(Span::raw(" | "));
    status_spans.push(Span::styled(
        "/: Search | q: Quit | Enter: Read | h/l: Category",
        Style::default().fg(Color::DarkGray),
    ));

    let status_content = Line::from(status_spans);
    f.render_widget(Paragraph::new(status_content), chunks[3]);
}

fn draw_main_view(f: &mut Frame, app: &mut App, area: Rect) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(20), // Categories
                Constraint::Min(0),     // News Feed
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
            let prefix = if i == app.selected_category {
                style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                "> "
            } else {
                "  "
            };
            ListItem::new(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(*c, style),
            ]))
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
            let mut style = Style::default();
            if i == app.selected_item {
                style = style
                    .bg(Color::Rgb(40, 44, 52))
                    .add_modifier(Modifier::BOLD);
            }

            let content = Line::from(vec![
                Span::styled(&item.formatted_time, Style::default().fg(Color::DarkGray)),
                Span::raw(" "),
                Span::styled(&item.formatted_source, Style::default().fg(Color::Green)),
                Span::raw(" "),
                Span::styled(&item.title, Style::default()),
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let feed_title = if app.search_query.is_empty() {
        format!(" {} - Latest News ", app.categories[app.selected_category])
    } else {
        format!(
            " {} - Search Results for '{}' ",
            app.categories[app.selected_category], app.search_query
        )
    };

    let news_list =
        List::new(items).block(Block::default().title(feed_title).borders(Borders::ALL));

    let mut state = ListState::default();
    state.select(Some(app.selected_item));
    f.render_stateful_widget(news_list, body_chunks[1], &mut state);
}

fn draw_reading_view(f: &mut Frame, app: &App, area: Rect) {
    let item = &app.items[app.selected_item];

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
            Span::styled(&item.source, Style::default().fg(Color::Green)),
            Span::raw(" | "),
            Span::styled("Time: ", Style::default().fg(Color::DarkGray)),
            Span::raw(
                Utc.timestamp_opt(item.timestamp, 0)
                    .latest()
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            ),
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

    if let Some(desc) = &item.content_summary {
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
