use crate::config::Theme;
use crate::db::{Db, NewsItem};
use crate::scraper::Scraper;
use crate::sources::get_categories;
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub enum AppEvent {
    Tick,
    Input(crossterm::event::KeyEvent),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Main,
    Reading,
    GlobalSearch,
}

pub struct App {
    pub db: Arc<Db>,
    pub categories: Vec<&'static str>,
    pub selected_category: usize,
    pub selected_item: usize,
    pub items: Vec<NewsItem>,
    pub stats: (usize, usize),
    pub should_quit: bool,
    pub view_mode: ViewMode,
    pub is_searching: bool,
    pub is_showing_help: bool,
    pub search_query: String,
    pub theme: Theme,
    pub refresh_countdown: i64,
    last_db_change: u64,
    last_fetched_search_query: String,
    last_fetched_category: usize,
}

impl App {
    pub fn new(db: Arc<Db>, theme: Theme) -> Self {
        let categories = get_categories();
        App {
            db,
            categories,
            selected_category: 0,
            selected_item: 0,
            items: Vec::new(),
            stats: (0, 0),
            should_quit: false,
            view_mode: ViewMode::Main,
            is_searching: false,
            is_showing_help: false,
            search_query: String::new(),
            theme,
            refresh_countdown: 0,
            last_db_change: 0,
            last_fetched_search_query: String::new(),
            last_fetched_category: 0,
        }
    }

    pub fn on_tick(&mut self) {
        let current_change = self.db.get_change_count();

        let now = chrono::Utc::now().timestamp();
        let next = self.db.next_fetch_timestamp.load(Ordering::Relaxed);
        self.refresh_countdown = (next - now).max(0);

        if self.view_mode != ViewMode::GlobalSearch {
            let db_changed = current_change > self.last_db_change;
            let search_changed = self.search_query != self.last_fetched_search_query;
            let category_changed = self.selected_category != self.last_fetched_category;

            if db_changed || search_changed || category_changed {
                self.fetch_items_from_db();
                self.last_db_change = current_change;
            }

            if let Some(stats) = (db_changed || self.stats.0 == 0)
                .then(|| self.db.get_stats().ok())
                .flatten()
            {
                self.stats = stats;
            }
        }
    }

    fn fetch_items_from_db(&mut self) {
        let cat = if self.selected_category == 0 {
            None
        } else {
            Some(self.categories[self.selected_category])
        };

        let search = if self.search_query.is_empty() {
            None
        } else {
            Some(self.search_query.as_str())
        };

        if let Ok(new_items) = self.db.get_latest_items(200, cat, search) {
            self.items = new_items;
            self.last_fetched_search_query = self.search_query.clone();
            self.last_fetched_category = self.selected_category;

            if self.items.is_empty() {
                self.selected_item = 0;
            } else if self.selected_item >= self.items.len() {
                self.selected_item = self.items.len() - 1;
            }
        }
    }

    fn perform_global_search(&mut self) {
        let query = self.search_query.clone();
        if query.is_empty() {
            return;
        }

        if let Ok(results) = Scraper::ddg_search(&query) {
            self.items = results
                .into_iter()
                .map(|a| NewsItem {
                    title: a.title,
                    source: "DuckDuckGo".to_string(),
                    category: "Global Search".to_string(),
                    url: a.source_url,
                    description: Some(a.content),
                    timestamp: chrono::Utc::now().timestamp(),
                    formatted_time: "NOW".to_string(),
                    formatted_source: "[SEARCH]".to_string(),
                })
                .collect();
            self.selected_item = 0;
            self.view_mode = ViewMode::GlobalSearch;
        }
    }

    pub fn on_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        if self.is_showing_help {
            self.is_showing_help = false;
            return;
        }

        if self.is_searching {
            match key.code {
                KeyCode::Enter => {
                    self.is_searching = false;
                    if self.view_mode == ViewMode::GlobalSearch
                        || self.search_query.starts_with('!')
                    {
                        if self.search_query.starts_with('!') {
                            self.search_query = self.search_query[1..].to_string();
                        }
                        self.perform_global_search();
                    } else {
                        self.fetch_items_from_db();
                    }
                }
                KeyCode::Esc => {
                    self.is_searching = false;
                    if self.view_mode != ViewMode::GlobalSearch {
                        self.fetch_items_from_db();
                    }
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                }
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') => {
                if self.view_mode == ViewMode::Reading {
                    self.view_mode = if self
                        .items
                        .first()
                        .map(|i| i.source.as_str() == "DuckDuckGo")
                        .unwrap_or(false)
                    {
                        ViewMode::GlobalSearch
                    } else {
                        ViewMode::Main
                    };
                } else if self.view_mode == ViewMode::GlobalSearch {
                    self.view_mode = ViewMode::Main;
                    self.search_query.clear();
                    self.fetch_items_from_db();
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Char('/') => {
                self.is_searching = true;
                self.selected_item = 0;
            }
            KeyCode::Char('s') => {
                self.is_searching = true;
                self.search_query.clear();
                self.view_mode = ViewMode::GlobalSearch;
            }
            KeyCode::Char('?') | KeyCode::F(1) => {
                self.is_showing_help = true;
            }
            KeyCode::Char('t') => {
                self.theme = match self.theme {
                    Theme::Black => Theme::White,
                    Theme::White => Theme::DeepBlue,
                    Theme::DeepBlue => Theme::Matrix,
                    Theme::Matrix => Theme::Black,
                };
            }
            KeyCode::Char('o') => {
                if !self.items.is_empty() {
                    let url = &self.items[self.selected_item].url;
                    let _ = webbrowser::open(url);
                }
            }
            KeyCode::Esc => {
                if self.view_mode == ViewMode::Reading {
                    self.view_mode = if self
                        .items
                        .first()
                        .map(|i| i.source.as_str() == "DuckDuckGo")
                        .unwrap_or(false)
                    {
                        ViewMode::GlobalSearch
                    } else {
                        ViewMode::Main
                    };
                } else if self.view_mode == ViewMode::GlobalSearch {
                    self.view_mode = ViewMode::Main;
                    self.search_query.clear();
                    self.fetch_items_from_db();
                } else if !self.search_query.is_empty() {
                    self.search_query.clear();
                    self.fetch_items_from_db();
                }
            }
            KeyCode::Enter => {
                if !self.items.is_empty()
                    && (self.view_mode == ViewMode::Main
                        || self.view_mode == ViewMode::GlobalSearch)
                {
                    self.view_mode = ViewMode::Reading;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if (self.view_mode == ViewMode::Main || self.view_mode == ViewMode::GlobalSearch)
                    && !self.items.is_empty()
                    && self.selected_item < self.items.len() - 1
                {
                    self.selected_item += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if (self.view_mode == ViewMode::Main || self.view_mode == ViewMode::GlobalSearch)
                    && self.selected_item > 0
                {
                    self.selected_item -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.view_mode == ViewMode::Main
                    && self.selected_category < self.categories.len() - 1
                {
                    self.selected_category += 1;
                    self.selected_item = 0;
                    self.fetch_items_from_db();
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.view_mode == ViewMode::Main && self.selected_category > 0 {
                    self.selected_category -= 1;
                    self.selected_item = 0;
                    self.fetch_items_from_db();
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Theme;
    use crate::db::Db;
    use std::sync::Arc;

    #[test]
    fn test_tick_performance_optimized() {
        let db = Arc::new(Db::new(":memory:").unwrap());
        let mut app = App::new(db, Theme::Black);

        // Initial fetch
        app.on_tick();
        let _initial_items = app.items.len();

        // Change category
        app.selected_category = 1;
        app.on_tick();
        assert_eq!(app.last_fetched_category, 1);

        // Change search query
        app.search_query = "test".to_string();
        app.on_tick();
        assert_eq!(app.last_fetched_search_query, "test");

        // Simulate DB change
        app.db.increment_change_counter();
        let old_change = app.last_db_change;
        app.on_tick();
        assert!(app.last_db_change > old_change);
    }
}
