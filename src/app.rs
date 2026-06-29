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
        }
    }

    pub fn on_tick(&mut self) {
        self.fetch_items_from_db();
        if let Ok(stats) = self.db.get_stats() {
            self.stats = stats;
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
