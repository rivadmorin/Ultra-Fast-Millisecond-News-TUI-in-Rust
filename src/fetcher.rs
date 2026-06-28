use crate::db::{Db, NewsItem};
use crate::sources::get_sources;
use chrono::Utc;
use log::{error, info};
use reqwest::Client;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

pub async fn start_fetcher(db: Arc<Db>, interval_seconds: u64) {
    let sources = get_sources();
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap_or_default();

    let interval = Duration::from_secs(interval_seconds);

    loop {
        info!("Starting fetch cycle for {} sources", sources.len());

        // We use tokio::spawn to fetch each feed concurrently
        let mut tasks = vec![];

        for source in &sources {
            let client = client.clone();
            let db = Arc::clone(&db);
            let url = source.url.to_string();
            let source_name = source.source_name.to_string();
            let category = source.category.to_string();

            let task = tokio::spawn(async move {
                if let Ok(res) = client.get(&url).send().await
                    && let Ok(bytes) = res.bytes().await
                        && let Ok(feed) = feed_rs::parser::parse(bytes.as_ref()) {
                            let mut items = Vec::new();
                            for entry in feed.entries {
                                let title = entry
                                    .title
                                    .map(|t| t.content)
                                    .unwrap_or_else(|| "No Title".to_string());
                                let url = entry
                                    .links
                                    .first()
                                    .map(|l| l.href.clone())
                                    .unwrap_or_else(|| "".to_string());
                                let description =
                                    entry.summary.map(|s| html2md::parse_html(&s.content));
                                let timestamp = entry
                                    .published
                                    .map(|d| d.timestamp())
                                    .unwrap_or_else(|| Utc::now().timestamp());

                                // Only process valid items with URL
                                if !url.is_empty() {
                                    items.push(NewsItem {
                                        id: None,
                                        title,
                                        source: source_name.clone(),
                                        category: category.clone(),
                                        url,
                                        description,
                                        timestamp,
                                    });
                                }
                            }

                            if !items.is_empty()
                                && let Err(e) = db.insert_items(&items) {
                                    error!("Failed to insert items from {}: {}", source_name, e);
                                }
                        }
            });

            tasks.push(task);

            // Add a very slight delay to stagger the network requests and simulate a continuous stream
            sleep(Duration::from_millis(50)).await;
        }

        // Wait for all tasks in this cycle to complete
        for task in tasks {
            let _ = task.await;
        }

        sleep(interval).await;
    }
}
