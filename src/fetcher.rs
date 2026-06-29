#![allow(clippy::collapsible_if)]
use crate::cleanser;
use crate::config::Config;
use crate::db::{Db, NewsItem, SourceMeta};
use crate::sources::get_sources;
use chrono::{Datelike, TimeZone, Timelike, Utc};
use log::{error, info, warn};
use rand::seq::SliceRandom;
use reqwest::Client;
use reqwest::header::{ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED, USER_AGENT};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};

const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (ApplePC; Apple iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
];

pub async fn start_fetcher(db: Arc<Db>, config: Config) {
    let sources = get_sources();
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .unwrap_or_default();

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

        let is_active = if config.active_hours_start <= config.active_hours_end {
            (config.active_hours_start..config.active_hours_end).contains(&hour)
        } else {
            hour >= config.active_hours_start || hour < config.active_hours_end
        };

        let interval_secs = if is_active {
            config.fetch_interval_active_seconds
        } else {
            config.fetch_interval_idle_seconds
        };

        info!(
            "Starting fetch cycle ({} sources, {} mode)",
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
                let _permit = sem.acquire().await.ok();

                let mut retry_count = 0;
                let max_retries = 3;
                let base_delay = Duration::from_secs(2);

                while retry_count < max_retries {
                    let meta = db.get_source_meta(&url).unwrap_or(None);
                    let ua = USER_AGENTS
                        .choose(&mut rand::thread_rng())
                        .unwrap_or(&USER_AGENTS[0]);

                    let mut req = client.get(&url).header(USER_AGENT, *ua);
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
                                return;
                            }

                            if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS
                                || res.status().is_server_error()
                            {
                                retry_count += 1;
                                let delay = base_delay * 2u32.pow(retry_count - 1);
                                warn!(
                                    "Rate limited or server error for {}, retrying in {:?} (Attempt {})",
                                    source_name, delay, retry_count
                                );
                                sleep(delay).await;
                                continue;
                            }

                            if !res.status().is_success() {
                                warn!("Failed to fetch {}: {}", source_name, res.status());
                                return;
                            }

                            let new_etag = res
                                .headers()
                                .get(ETAG)
                                .and_then(|v| v.to_str().ok().map(|s| s.to_string()));
                            let new_lm = res
                                .headers()
                                .get(LAST_MODIFIED)
                                .and_then(|v| v.to_str().ok().map(|s| s.to_string()));

                            let body_bytes = match res.bytes().await {
                                Ok(b) => b,
                                Err(_) => return,
                            };

                            if let Ok(feed) = feed_rs::parser::parse(body_bytes.as_ref()) {
                                let mut items = Vec::new();
                                for entry in feed.entries {
                                    let title = cleanser::cleanse_text(
                                        &entry
                                            .title
                                            .map(|t| t.content)
                                            .unwrap_or_else(|| "No Title".to_string()),
                                    );
                                    let item_url = entry
                                        .links
                                        .first()
                                        .map(|l| l.href.clone())
                                        .unwrap_or_default();
                                    let content_summary = entry
                                        .summary
                                        .map(|s| cleanser::cleanse_html(&s.content))
                                        .or_else(|| {
                                            entry
                                                .content
                                                .and_then(|c| c.body)
                                                .map(|b| cleanser::cleanse_html(&b))
                                        });

                                    let published_at = entry
                                        .published
                                        .or(entry.updated)
                                        .map(|d| d.timestamp())
                                        .unwrap_or_else(|| Utc::now().timestamp());

                                    if !item_url.is_empty() && !title.is_empty() {
                                        let datetime = Utc
                                            .timestamp_opt(published_at, 0)
                                            .latest()
                                            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap());
                                        let formatted_time = datetime.format("%H:%M").to_string();
                                        let formatted_source = format!("[{}]", source_name);

                                        items.push(NewsItem {
                                            title,
                                            source: source_name.clone(),
                                            category: category.clone(),
                                            url: item_url,
                                            content_summary,
                                            published_at,
                                            formatted_time,
                                            formatted_source,
                                        });
                                    }
                                }

                                if !items.is_empty() {
                                    let _ = db.insert_items(&items);
                                }

                                let _ = db.set_source_meta(&SourceMeta {
                                    url: url.clone(),
                                    etag: new_etag,
                                    last_modified: new_lm,
                                });
                            }
                            return;
                        }
                        Err(e) => {
                            retry_count += 1;
                            let delay = base_delay * 2u32.pow(retry_count - 1);
                            error!(
                                "Error fetching {}: {}, retrying in {:?} (Attempt {})",
                                source_name, e, delay, retry_count
                            );
                            sleep(delay).await;
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
