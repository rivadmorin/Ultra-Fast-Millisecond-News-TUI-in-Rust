use crate::config::Config;
use crate::db::{Db, NewsItem, SourceMeta};
use crate::sources::get_sources;
use chrono::{Datelike, Timelike, Utc};
use log::{error, info, warn};
use reqwest::Client;
use reqwest::header::{ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};

pub async fn start_fetcher(db: Arc<Db>, config: Config) {
    let sources = get_sources();
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("LiveNewsTUI/0.1.0 (+https://github.com/LiveNewsTUI/LiveNews)")
        .build()
        .unwrap_or_default();

    let semaphore = Arc::new(Semaphore::new(config.worker_threads));
    let mut last_maintenance_day = Utc::now().day();

    loop {
        let now = Utc::now();
        let hour = now.hour();

        // Check for daily maintenance (run at 3 AM)
        if hour == 3 && now.day() != last_maintenance_day {
            info!("Running daily maintenance...");
            let _ = db.cleanup_old_data(&config.retention);
            let _ = db.vacuum();
            last_maintenance_day = now.day();
        }

        // Determine interval based on active hours
        let is_active = if config.active_hours_start <= config.active_hours_end {
            hour >= config.active_hours_start && hour < config.active_hours_end
        } else {
            // Overlap midnight (e.g., 22:00 to 06:00)
            hour >= config.active_hours_start || hour < config.active_hours_end
        };

        let interval_secs = if is_active {
            config.fetch_interval_active_seconds
        } else {
            config.fetch_interval_idle_seconds
        };

        info!(
            "Starting fetch cycle for {} sources (Mode: {})",
            sources.len(),
            if is_active { "Active" } else { "Idle" }
        );

        let mut tasks = vec![];

        for source in &sources {
            let client = client.clone();
            let db = Arc::clone(&db);
            let sem = Arc::clone(&semaphore);
            let url = source.url.to_string();
            let source_name = source.source_name.to_string();
            let category = source.category.to_string();

            let task = tokio::spawn(async move {
                // Acquire permit from semaphore to limit concurrency
                let _permit = sem.acquire().await.ok();

                // Get metadata for conditional GET
                let meta = db.get_source_meta(&url).unwrap_or(None);

                let mut req = client.get(&url);
                if let Some(ref m) = meta {
                    if let Some(ref etag) = m.etag {
                        req = req.header(IF_NONE_MATCH, etag);
                    }
                    if let Some(ref lm) = m.last_modified {
                        req = req.header(IF_MODIFIED_SINCE, lm);
                    }
                }

                match req.send().await {
                    Ok(res) => {
                        if res.status() == reqwest::StatusCode::NOT_MODIFIED {
                            // No changes since last fetch
                            return;
                        }

                        if !res.status().is_success() {
                            warn!("Failed to fetch {}: {}", source_name, res.status());
                            return;
                        }

                        // Capture new headers for next time
                        let new_etag = res
                            .headers()
                            .get(ETAG)
                            .and_then(|v| v.to_str().ok().map(|s| s.to_string()));
                        let new_lm = res
                            .headers()
                            .get(LAST_MODIFIED)
                            .and_then(|v| v.to_str().ok().map(|s| s.to_string()));

                        if let Ok(bytes) = res.bytes().await {
                            if let Ok(feed) = feed_rs::parser::parse(bytes.as_ref()) {
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
                                        items.push(NewsItem {
                                            id: None,
                                            title,
                                            source: source_name.clone(),
                                            category: category.clone(),
                                            url: item_url,
                                            description,
                                            timestamp,
                                        });
                                    }
                                }

                                if !items.is_empty() {
                                    if let Err(e) = db.insert_items(&items) {
                                        error!(
                                            "Failed to insert items from {}: {}",
                                            source_name, e
                                        );
                                    }
                                }

                                // Update metadata even if no new items were inserted (but feed was fetched successfully)
                                let _ = db.set_source_meta(&SourceMeta {
                                    url: url.clone(),
                                    etag: new_etag,
                                    last_modified: new_lm,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error fetching {}: {}", source_name, e);
                    }
                }
            });

            tasks.push(task);
        }

        // Wait for all tasks in this cycle to complete
        for task in tasks {
            let _ = task.await;
        }

        sleep(Duration::from_secs(interval_secs)).await;
    }
}
