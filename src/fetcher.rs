use crate::config::Config;
use crate::db::{Db, NewsItem};
use crate::scraper::Scraper;
use crate::sources::get_sources;
use chrono::{Datelike, TimeZone, Timelike, Utc};
use log::{error, info};
use std::io::Cursor;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};

pub async fn start_fetcher(db: Arc<Db>, config: Config) {
    let sources = get_sources();
    let semaphore = Arc::new(Semaphore::new(config.worker_threads));
    let mut last_maintenance_day = Utc::now().day();

    loop {
        let now = Utc::now();
        let hour = now.hour();

        if hour == 3 && now.day() != last_maintenance_day {
            info!("Running daily maintenance...");
            let _ = db.cleanup_old_data(&config.retention);
            let _ = db.vacuum();
            last_maintenance_day = now.day();
        }

        let is_active = (config.active_hours_start..config.active_hours_end).contains(&hour);

        let interval_secs = if is_active {
            config.fetch_interval_active_seconds
        } else {
            config.fetch_interval_idle_seconds
        };

        db.next_fetch_timestamp
            .store(now.timestamp() + interval_secs as i64, Ordering::Relaxed);

        info!(
            "Starting stealthy fetch cycle for {} sources (Mode: {})",
            sources.len(),
            if is_active { "Active" } else { "Idle" }
        );

        let mut tasks = vec![];

        for source in &sources {
            let db = Arc::clone(&db);
            let sem = Arc::clone(&semaphore);
            let url = source.url.to_string();
            let source_name = source.source_name.to_string();
            let category = source.category.to_string();

            let task = tokio::spawn(async move {
                let _permit = sem.acquire().await.ok();

                info!("Stealthy fetching: {}", source_name);

                let fetch_result =
                    tokio::task::spawn_blocking(move || Scraper::fetch_raw(&url)).await;

                if let Ok(Ok(bytes)) = fetch_result {
                    let cursor = Cursor::new(bytes);
                    if let Ok(feed) = feed_rs::parser::parse(cursor) {
                        let mut items = Vec::new();
                        for entry in feed.entries {
                            let title = entry
                                .title
                                .map(|t| t.content)
                                .unwrap_or_else(|| "No Title".to_string());
                            let item_url = entry
                                .links
                                .first()
                                .map(|l| l.href.clone())
                                .unwrap_or_default();

                            let description =
                                entry.summary.map(|s| html2md::parse_html(&s.content));

                            let timestamp = entry
                                .published
                                .map(|d| d.timestamp())
                                .unwrap_or_else(|| Utc::now().timestamp());

                            if !item_url.is_empty() {
                                let datetime = Utc
                                    .timestamp_opt(timestamp, 0)
                                    .latest()
                                    .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap());
                                let formatted_time = datetime.format("%H:%M").to_string();
                                let formatted_source = format!("[{}]", source_name);

                                items.push(NewsItem {
                                    title,
                                    source: source_name.clone(),
                                    category: category.clone(),
                                    url: item_url,
                                    description,
                                    timestamp,
                                    formatted_time,
                                    formatted_source,
                                });
                            }
                        }

                        #[allow(clippy::collapsible_if)]
                        if !items.is_empty() {
                            if let Err(e) = db.insert_items(&items) {
                                error!("Failed to insert items from {}: {}", source_name, e);
                            }
                        }
                    }
                }
            });

            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await;
        }

        sleep(Duration::from_secs(interval_secs)).await;
    }
}
