use crate::config::RetentionPolicy;
use chrono::Utc;
use log::info;
use rusqlite::{Connection, Result, params};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Debug, Clone)]
pub struct NewsItem {
    pub id: Option<i64>,
    pub title: String,
    pub source: String,
    pub category: String,
    pub url: String,
    pub description: Option<String>,
    pub timestamp: i64,
}

pub struct SourceMeta {
    pub url: String,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

impl Db {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;

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
            "CREATE TABLE IF NOT EXISTS sources_meta (
                url TEXT PRIMARY KEY,
                etag TEXT,
                last_modified TEXT
            )",
            [],
        )?;

        // Create indexes for faster queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON news (timestamp)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_category ON news (category)",
            [],
        )?;

        info!("Database initialized");

        Ok(Db {
            conn: Arc::new(Mutex::new(conn)),
        })
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
        Ok(count)
    }

    pub fn get_latest_items(&self, limit: usize, category: Option<&str>) -> Result<Vec<NewsItem>> {
        let conn = self.conn.lock().unwrap();

        let mut query = String::from(
            "SELECT id, title, source, category, url, description, timestamp FROM news",
        );
        if category.is_some() {
            query.push_str(" WHERE category = ?1");
        }
        query.push_str(" ORDER BY timestamp DESC LIMIT ");
        if category.is_some() {
            query.push_str("?2");
        } else {
            query.push_str("?1");
        }

        let mut stmt = conn.prepare(&query)?;

        let mut rows = if let Some(cat) = category {
            stmt.query(params![cat, limit as i64])?
        } else {
            stmt.query(params![limit as i64])?
        };

        let mut items = Vec::new();
        while let Some(row) = rows.next()? {
            items.push(NewsItem {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                source: row.get(2)?,
                category: row.get(3)?,
                url: row.get(4)?,
                description: row.get(5)?,
                timestamp: row.get(6)?,
            });
        }

        Ok(items)
    }

    pub fn get_source_meta(&self, url: &str) -> Result<Option<SourceMeta>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT url, etag, last_modified FROM sources_meta WHERE url = ?1")?;
        let mut rows = stmt.query(params![url])?;

        if let Some(row) = rows.next()? {
            Ok(Some(SourceMeta {
                url: row.get(0)?,
                etag: row.get(1)?,
                last_modified: row.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn set_source_meta(&self, meta: &SourceMeta) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO sources_meta (url, etag, last_modified) VALUES (?1, ?2, ?3)",
            params![meta.url, meta.etag, meta.last_modified],
        )?;
        Ok(())
    }

    pub fn cleanup_old_data(&self, policy: &RetentionPolicy) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let cutoff = Utc::now().timestamp() - (policy.as_seconds() as i64);
        let deleted = conn.execute("DELETE FROM news WHERE timestamp < ?1", params![cutoff])?;
        if deleted > 0 {
            info!("Cleaned up {} old records", deleted);
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
        let sources: usize = conn.query_row("SELECT COUNT(DISTINCT source) FROM news", [], |r| r.get(0))?;
        Ok((count, sources))
    }
}
