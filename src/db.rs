use crate::config::RetentionPolicy;
use chrono::{TimeZone, Utc};
use log::info;
use rusqlite::{Connection, Result, params};
use std::path::Path;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub struct Db {
    conn: Arc<Mutex<Connection>>,
    change_counter: Arc<AtomicU64>,
    pub next_fetch_timestamp: Arc<AtomicI64>,
}

#[derive(Debug, Clone)]
pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub category: String,
    pub url: String,
    pub description: Option<String>,
    pub timestamp: i64,
    pub formatted_time: String,
    pub formatted_source: String,
}

impl Db {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;

        // Performance Pragmas (Corrected to use pragma_update)
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "cache_size", -64000)?; // 64MB cache

        conn.execute(
            "CREATE TABLE IF NOT EXISTS news (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                source TEXT NOT NULL,
                category TEXT NOT NULL,
                url TEXT NOT NULL UNIQUE,
                description TEXT,
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON news (timestamp)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_category ON news (category)",
            [],
        )?;

        info!("Database initialized with performance optimizations");

        Ok(Db {
            conn: Arc::new(Mutex::new(conn)),
            change_counter: Arc::new(AtomicU64::new(1)),
            next_fetch_timestamp: Arc::new(AtomicI64::new(0)),
        })
    }

    pub fn get_change_count(&self) -> u64 {
        self.change_counter.load(Ordering::Relaxed)
    }

    pub fn increment_change_counter(&self) {
        self.change_counter.fetch_add(1, Ordering::SeqCst);
    }

    pub fn insert_items(&self, items: &[NewsItem]) -> Result<usize> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        let mut count = 0;

        {
            let mut stmt = tx.prepare(
                "INSERT OR IGNORE INTO news (title, source, category, url, description, timestamp)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )?;

            for item in items {
                let res = stmt.execute(params![
                    item.title,
                    item.source,
                    item.category,
                    item.url,
                    item.description,
                    item.timestamp,
                ])?;
                count += res;
            }
        }

        tx.commit()?;
        if count > 0 {
            self.increment_change_counter();
        }
        Ok(count)
    }

    pub fn get_latest_items(
        &self,
        limit: usize,
        category: Option<&str>,
        search: Option<&str>,
    ) -> Result<Vec<NewsItem>> {
        let conn = self.conn.lock().unwrap();

        let mut query = String::from(
            "SELECT title, source, category, url, description, timestamp FROM news WHERE 1=1",
        );
        let mut params_vec: Vec<rusqlite::types::Value> = Vec::new();

        if let Some(cat) = category {
            query.push_str(" AND category = ?");
            params_vec.push(rusqlite::types::Value::Text(cat.to_string()));
        }

        if let Some(q) = search {
            query.push_str(" AND (title LIKE ? OR description LIKE ?)");
            let pattern = format!("%{}%", q);
            params_vec.push(rusqlite::types::Value::Text(pattern.clone()));
            params_vec.push(rusqlite::types::Value::Text(pattern));
        }

        query.push_str(" ORDER BY timestamp DESC LIMIT ?");
        params_vec.push(rusqlite::types::Value::Integer(limit as i64));

        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query(rusqlite::params_from_iter(params_vec))?;

        let mut items = Vec::new();
        while let Some(row) = rows.next()? {
            let timestamp: i64 = row.get(5)?;
            let source: String = row.get(1)?;

            let datetime = Utc
                .timestamp_opt(timestamp, 0)
                .latest()
                .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap());
            let formatted_time = datetime.format("%H:%M").to_string();
            let formatted_source = format!("[{}]", source);

            items.push(NewsItem {
                title: row.get(0)?,
                source,
                category: row.get(2)?,
                url: row.get(3)?,
                description: row.get(4)?,
                timestamp,
                formatted_time,
                formatted_source,
            });
        }

        Ok(items)
    }

    pub fn cleanup_old_data(&self, policy: &RetentionPolicy) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let cutoff = Utc::now().timestamp() - (policy.as_seconds() as i64);
        let deleted = conn.execute("DELETE FROM news WHERE timestamp < ?1", params![cutoff])?;
        if deleted > 0 {
            info!("Cleaned up {} old records", deleted);
            self.increment_change_counter();
        }
        Ok(deleted)
    }

    pub fn vacuum(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("VACUUM", [])?;
        info!("Database vacuumed");
        Ok(())
    }

    pub fn get_stats(&self) -> Result<(usize, usize)> {
        let conn = self.conn.lock().unwrap();
        let count: usize = conn.query_row("SELECT COUNT(*) FROM news", [], |r| r.get(0))?;
        let sources: usize =
            conn.query_row("SELECT COUNT(DISTINCT source) FROM news", [], |r| r.get(0))?;
        Ok((count, sources))
    }
}
