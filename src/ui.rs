use crate::app::{App, ViewMode};
use crate::config::Theme;
use chrono::{TimeZone, Timelike, Utc};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

struct ThemeColors {
    cyan: Color,
    green: Color,
    gray: Color,
    bg_selected: Color,
    text: Color,
    bg: Color,
}

impl ThemeColors {
    fn from_theme(theme: Theme) -> Self {
        match theme {
            Theme::Black => ThemeColors {
                cyan: Color::Cyan,
                green: Color::Green,
                gray: Color::DarkGray,
                bg_selected: Color::Rgb(40, 44, 52),
                text: Color::White,
                bg: Color::Black,
            },
            Theme::White => ThemeColors {
                cyan: Color::Blue,
                green: Color::DarkGray,
                gray: Color::Rgb(100, 100, 100),
                bg_selected: Color::Rgb(220, 220, 220),
                text: Color::Black,
                bg: Color::White,
            },
            Theme::DeepBlue => ThemeColors {
                cyan: Color::Rgb(0, 191, 255),
                green: Color::Rgb(144, 238, 144),
                gray: Color::Rgb(112, 128, 144),
                bg_selected: Color::Rgb(25, 25, 112),
                text: Color::Rgb(240, 248, 255),
                bg: Color::Rgb(0, 0, 128),
            },
            Theme::Matrix => ThemeColors {
                cyan: Color::Rgb(0, 255, 0),
                green: Color::Rgb(0, 128, 0),
                gray: Color::Rgb(0, 100, 0),
                bg_selected: Color::Rgb(0, 50, 0),
                text: Color::Rgb(0, 255, 0),
                bg: Color::Black,
            },
        }
    }
}

pub fn draw(f: &mut Frame, app: &mut App) {
    let colors = ThemeColors::from_theme(app.theme);

    f.render_widget(
        Block::default().style(Style::default().bg(colors.bg)),
        f.area(),
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    match app.view_mode {
        ViewMode::Reading => draw_reading_view(f, app, chunks[0], &colors),
        ViewMode::Main => draw_main_view(f, app, chunks[0], &colors),
    }

    draw_help_bar(f, app, chunks[1], &colors);

    if app.is_searching {
        draw_search_popup(f, app, &colors);
    }

    if app.is_showing_help {
        draw_help_popup(f, app, &colors);
    }
}

fn draw_main_view(f: &mut Frame, app: &mut App, area: Rect, colors: &ThemeColors) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    draw_header(f, app, main_layout[0], colors);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(16), Constraint::Min(0)])
        .split(main_layout[1]);

    // Categories
    let categories: Vec<ListItem> = app
        .categories
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let mut style = Style::default().fg(colors.text);
            if i == app.selected_category {
                style = style.fg(colors.cyan).add_modifier(Modifier::BOLD);
            }
            ListItem::new(format!(" {}", c)).style(style)
        })
        .collect();

    let categories_block = Block::default()
        .title(" Categories ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(colors.gray));

    f.render_widget(
        List::new(categories).block(categories_block),
        content_chunks[0],
    );

    // News Feed
    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let mut style = Style::default();
            if i == app.selected_item {
                style = style.bg(colors.bg_selected).add_modifier(Modifier::BOLD);
            }

            let content = Line::from(vec![
                Span::styled(
                    format!(" {} ", item.formatted_time),
                    Style::default().fg(colors.gray),
                ),
                Span::styled(
                    format!(" {} ", item.formatted_source),
                    Style::default().fg(colors.green),
                ),
                Span::styled(&item.title, Style::default().fg(colors.text)),
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let feed_title = if app.search_query.is_empty() {
        format!(
            " {} - Stealthy Feed ",
            app.categories[app.selected_category]
        )
    } else {
        format!(" Search: '{}' ", app.search_query)
    };

    let news_block = Block::default()
        .title(feed_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(colors.gray));

    let mut state = ListState::default();
    state.select(Some(app.selected_item));
    f.render_stateful_widget(
        List::new(items).block(news_block),
        content_chunks[1],
        &mut state,
    );
}

fn draw_header(f: &mut Frame, app: &App, area: Rect, colors: &ThemeColors) {
    let now = Utc::now();
    let hour = now.hour();
    let is_active = hour >= 6 && hour < 22;
    let mode_str = if is_active { "ACTIVE" } else { "IDLE" };
    let mode_color = if is_active {
        colors.green
    } else {
        Color::Yellow
    };

    let header_content = Line::from(vec![
        Span::styled(
            " LIVE NEWS TUI ",
            Style::default()
                .fg(colors.bg)
                .bg(colors.cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            format!(" {} ", mode_str),
            Style::default()
                .fg(colors.bg)
                .bg(mode_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Items: {} | Sources: {}", app.stats.0, app.stats.1),
            Style::default().fg(colors.gray),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Sync: {}s", app.refresh_countdown),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Theme: {:?}", app.theme),
            Style::default().fg(colors.cyan),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(colors.gray));

    f.render_widget(Paragraph::new(header_content).block(block), area);
}

fn draw_help_bar(f: &mut Frame, app: &App, area: Rect, colors: &ThemeColors) {
    let help_text = if app.view_mode == ViewMode::Reading {
        " [Esc/q] back "
    } else if app.is_searching {
        " [Enter/Esc] close "
    } else {
        " [/] search | [t] theme | [Enter] read | [h/l] category | [j/k] navigate | [q] quit | [?] help "
    };

    let help_content = Line::from(vec![Span::styled(
        help_text,
        Style::default().fg(colors.bg).bg(colors.gray),
    )]);

    f.render_widget(Paragraph::new(help_content), area);
}

fn draw_reading_view(f: &mut Frame, app: &App, area: Rect, colors: &ThemeColors) {
    let item = &app.items[app.selected_item];

    let mut text = vec![
        Line::from(Span::styled(
            &item.title,
            Style::default()
                .fg(colors.cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Source: ", Style::default().fg(colors.gray)),
            Span::styled(&item.source, Style::default().fg(colors.green)),
            Span::raw(" | "),
            Span::styled("Time: ", Style::default().fg(colors.gray)),
            Span::raw(
                Utc.timestamp_opt(item.timestamp, 0)
                    .latest()
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            ),
        ]),
        Line::from(vec![
            Span::styled("URL: ", Style::default().fg(colors.gray)),
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
            text.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(colors.text),
            )));
        }
    }

    let block = Block::default()
        .title(" Article Details ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(colors.cyan));

    f.render_widget(
        Paragraph::new(text).block(block).wrap(Wrap { trim: true }),
        area,
    );
}

fn draw_search_popup(f: &mut Frame, app: &App, colors: &ThemeColors) {
    let area = centered_rect(50, 10, f.area());

    let block = Block::default()
        .title(" Stealthy Search ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(colors.cyan))
        .style(Style::default().bg(colors.bg));

    let search_text = Paragraph::new(format!(" > {}", app.search_query))
        .block(block)
        .style(Style::default().fg(colors.text));

    f.render_widget(Clear, area);
    f.render_widget(search_text, area);
}

fn draw_help_popup(f: &mut Frame, _app: &App, colors: &ThemeColors) {
    let area = centered_rect(60, 45, f.area());

    let text = vec![
        Line::from(Span::styled(
            "Stealthy Terminal Shortcuts",
            Style::default()
                .fg(colors.cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(" [/] ", Style::default().fg(colors.green)),
            Span::raw(" Adaptive Search"),
        ]),
        Line::from(vec![
            Span::styled(" [t] ", Style::default().fg(colors.green)),
            Span::raw(" Toggle Theme Engine"),
        ]),
        Line::from(vec![
            Span::styled(" [Enter] ", Style::default().fg(colors.green)),
            Span::raw(" Read Article"),
        ]),
        Line::from(vec![
            Span::styled(" [Esc/q] ", Style::default().fg(colors.green)),
            Span::raw(" Close Popup"),
        ]),
        Line::from(vec![
            Span::styled(" [h/l] ", Style::default().fg(colors.green)),
            Span::raw(" Switch Categories"),
        ]),
        Line::from(vec![
            Span::styled(" [j/k] ", Style::default().fg(colors.green)),
            Span::raw(" Navigate Feed"),
        ]),
        Line::from(vec![
            Span::styled(" [?] ", Style::default().fg(colors.green)),
            Span::raw(" Show Help"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Powered by Rust, SQLite & Scrapling",
            Style::default().fg(colors.gray),
        )),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(colors.cyan))
        .style(Style::default().bg(colors.bg));

    let help_text = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(colors.text));

    f.render_widget(Clear, area);
    f.render_widget(help_text, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
