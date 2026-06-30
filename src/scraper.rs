use anyhow::{Result, anyhow};
use chrono::Utc;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsArticle {
    pub id: String,
    pub title: String,
    pub author: String,
    pub publish_date: String,
    pub category: String,
    pub content: String,
    pub source_url: String,
    pub scraped_at: String,
}

pub struct Scraper;

impl Scraper {
    pub fn fetch_raw(url: &str) -> Result<Vec<u8>> {
        let result: PyResult<Vec<u8>> = Python::with_gil(|py| {
            let scrapling = py.import("scrapling")?;
            let fetchers = scrapling.getattr("fetchers")?;
            let stealthy_session = fetchers.getattr("StealthySession")?;

            let kwargs = PyDict::new(py);
            kwargs.set_item("solve_cloudflare", true)?;
            let session = stealthy_session.call((), Some(&kwargs))?;

            let page = session.call_method1("get", (url,))?;
            let content: Vec<u8> = page.getattr("content")?.extract()?;
            Ok(content)
        });

        result.map_err(|e| anyhow!("Python error during fetch_raw: {}", e))
    }

    pub fn ddg_search(query: &str) -> Result<Vec<NewsArticle>> {
        let result: PyResult<Vec<NewsArticle>> = Python::with_gil(|py| {
            let ddgs = py.import("duckduckgo_search")?.getattr("DDGS")?;
            let ddgs_instance = ddgs.call0()?;

            let kwargs = PyDict::new(py);
            kwargs.set_item("keywords", query)?;
            kwargs.set_item("max_results", 20)?;

            let results = ddgs_instance.call_method("text", (), Some(&kwargs))?;
            let results_list = results.downcast::<PyList>()?;

            let mut articles = Vec::new();
            for item in results_list.iter() {
                let dict = item.downcast::<PyDict>()?;
                let title: String = dict
                    .get_item("title")?
                    .map(|i| i.extract())
                    .unwrap_or(Ok("No Title".to_string()))?;
                let href: String = dict
                    .get_item("href")?
                    .map(|i| i.extract())
                    .unwrap_or(Ok("".to_string()))?;
                let body: String = dict
                    .get_item("body")?
                    .map(|i| i.extract())
                    .unwrap_or(Ok("".to_string()))?;

                let id = format!("{:x}", md5::compute(&href));

                articles.push(NewsArticle {
                    id,
                    title,
                    author: "DuckDuckGo".to_string(),
                    publish_date: "Recent".to_string(),
                    category: "Search".to_string(),
                    content: body,
                    source_url: href,
                    scraped_at: Utc::now().to_rfc3339(),
                });
            }
            Ok(articles)
        });

        result.map_err(|e| anyhow!("DuckDuckGo Search error: {}", e))
    }

    pub fn scrape_article(url: &str) -> Result<NewsArticle> {
        let result: PyResult<NewsArticle> = Python::with_gil(|py| {
            let scrapling = py.import("scrapling")?;
            let fetchers = scrapling.getattr("fetchers")?;
            let stealthy_session = fetchers.getattr("StealthySession")?;

            let kwargs = PyDict::new(py);
            kwargs.set_item("solve_cloudflare", true)?;
            let session = stealthy_session.call((), Some(&kwargs))?;

            let page = session.call_method1("get", (url,))?;

            let title: String = page
                .call_method1("css", ("h1::text, title::text",))?
                .call_method0("get")?
                .extract()
                .unwrap_or_else(|_| "Unknown Title".to_string());

            let content: String = page
                .call_method1("css", ("article p::text, .content p::text, main p::text",))?
                .call_method0("get_all")?
                .extract::<Vec<String>>()?
                .join("\n");

            let id = format!("{:x}", md5::compute(url));
            let scraped_at = Utc::now().to_rfc3339();

            Ok(NewsArticle {
                id,
                title,
                author: "Unknown".to_string(),
                publish_date: "N/A".to_string(),
                category: "General".to_string(),
                content,
                source_url: url.to_string(),
                scraped_at,
            })
        });

        result.map_err(|e| anyhow!("Python error during scrape_article: {}", e))
    }
}

pub fn run_example_scraper(url: &str) -> Result<()> {
    let article = Scraper::scrape_article(url)?;
    println!("{}", serde_json::to_string_pretty(&article)?);
    Ok(())
}
