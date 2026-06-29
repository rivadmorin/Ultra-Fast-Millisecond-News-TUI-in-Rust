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
    last_db_change_count: u64,
    last_fetched_category: Option<usize>,
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
            last_db_change_count: u64::MAX, // Force initial fetch
            last_fetched_category: None,
        }
    }

    pub fn on_tick(&mut self) {
        let current_change_count = self.db.get_change_count();
        if current_change_count != self.last_db_change_count
            || Some(self.selected_category) != self.last_fetched_category
        {
            self.fetch_items_from_db();
            if let Ok(stats) = self.db.get_stats() {
                self.stats = stats;
            }
            self.last_db_change_count = current_change_count;
            self.last_fetched_category = Some(self.selected_category);
        }
    }

    fn fetch_items_from_db(&mut self) {
        let cat = if self.selected_category == 0 {
            None
        } else {
            Some(self.categories[self.selected_category])
        };

        if let Ok(new_items) = self.db.get_latest_items(50, cat) {
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

        match key.code {
            KeyCode::Char('q') => {
                if self.is_reading {
                    self.is_reading = false;
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Esc => {
                self.is_reading = false;
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
                    // No longer need to fetch here, on_tick will handle it
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if !self.is_reading && self.selected_category > 0 {
                    self.selected_category -= 1;
                    self.selected_item = 0;
                    // No longer need to fetch here, on_tick will handle it
                }
            }
            _ => {}
        }
    }
}
