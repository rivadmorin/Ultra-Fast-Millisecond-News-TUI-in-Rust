use crate::db::{Db, NewsItem};
use crate::sources::get_categories;
use std::sync::Arc;

pub enum AppEvent {
    Tick,
    Input(crossterm::event::KeyEvent),
}

pub struct App {
    pub db: Arc<Db>,
    pub categories: Vec<&'static str>,
    pub selected_category: usize,
    pub selected_item: usize,
    pub items: Vec<NewsItem>,
    pub stats: (usize, usize),
    pub should_quit: bool,
    pub is_reading: bool,
    pub is_searching: bool,
    pub search_query: String,
    last_db_change: u64,
}

impl App {
    pub fn new(db: Arc<Db>) -> Self {
        let categories = get_categories();
        App {
            db,
            categories,
            selected_category: 0,
            selected_item: 0,
            items: Vec::new(),
            stats: (0, 0),
            should_quit: false,
            is_reading: false,
            is_searching: false,
            search_query: String::new(),
            last_db_change: 0,
        }
    }

    pub fn on_tick(&mut self) {
        let current_change = self.db.get_change_count();
        // Always refresh if searching, otherwise check for changes
        if self.is_searching || current_change > self.last_db_change {
            self.fetch_items_from_db();
            if let Ok(stats) = self.db.get_stats() {
                self.stats = stats;
            }
            self.last_db_change = current_change;
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

        if let Ok(new_items) = self.db.get_latest_items(100, cat, search) {
            self.items = new_items;

            if self.items.is_empty() {
                self.selected_item = 0;
            } else if self.selected_item >= self.items.len() {
                self.selected_item = self.items.len() - 1;
            }
        }
    }

    pub fn on_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        if self.is_searching {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.is_searching = false;
                    self.fetch_items_from_db();
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                    self.fetch_items_from_db();
                }
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                    self.fetch_items_from_db();
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') => {
                if self.is_reading {
                    self.is_reading = false;
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Char('/') => {
                self.is_searching = true;
                self.selected_item = 0;
            }
            KeyCode::Esc => {
                if self.is_reading {
                    self.is_reading = false;
                } else if !self.search_query.is_empty() {
                    self.search_query.clear();
                    self.fetch_items_from_db();
                }
            }
            KeyCode::Enter => {
                if !self.items.is_empty() {
                    self.is_reading = true;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.is_reading
                    && !self.items.is_empty()
                    && self.selected_item < self.items.len() - 1
                {
                    self.selected_item += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if !self.is_reading && self.selected_item > 0 {
                    self.selected_item -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if !self.is_reading && self.selected_category < self.categories.len() - 1 {
                    self.selected_category += 1;
                    self.selected_item = 0;
                    self.fetch_items_from_db();
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if !self.is_reading && self.selected_category > 0 {
                    self.selected_category -= 1;
                    self.selected_item = 0;
                    self.fetch_items_from_db();
                }
            }
            _ => {}
        }
    }
}
