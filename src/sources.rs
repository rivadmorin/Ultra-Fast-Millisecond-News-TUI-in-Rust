pub struct FeedSource {
    pub url: &'static str,
    pub category: &'static str,
    pub source_name: &'static str,
}

pub fn get_sources() -> Vec<FeedSource> {
    vec![
        // Tech
        FeedSource {
            url: "https://news.ycombinator.com/rss",
            category: "Tech",
            source_name: "Hacker News",
        },
        FeedSource {
            url: "https://techcrunch.com/feed/",
            category: "Tech",
            source_name: "TechCrunch",
        },
        FeedSource {
            url: "https://www.theverge.com/rss/index.xml",
            category: "Tech",
            source_name: "The Verge",
        },
        FeedSource {
            url: "https://wired.com/feed/rss",
            category: "Tech",
            source_name: "Wired",
        },
        FeedSource {
            url: "https://feeds.arstechnica.com/arstechnica/index",
            category: "Tech",
            source_name: "Ars Technica",
        },
        // AI
        FeedSource {
            url: "https://openai.com/blog/rss.xml",
            category: "AI",
            source_name: "OpenAI",
        },
        FeedSource {
            url: "https://deepmind.google/blog/rss.xml",
            category: "AI",
            source_name: "DeepMind",
        },
        FeedSource {
            url: "https://towardsdatascience.com/feed",
            category: "AI",
            source_name: "Towards Data Science",
        },
        // World News
        FeedSource {
            url: "http://feeds.bbci.co.uk/news/rss.xml",
            category: "World News",
            source_name: "BBC News",
        },
        FeedSource {
            url: "https://rss.nytimes.com/services/xml/rss/nyt/World.xml",
            category: "World News",
            source_name: "NYT",
        },
        FeedSource {
            url: "https://www.theguardian.com/world/rss",
            category: "World News",
            source_name: "The Guardian",
        },
        FeedSource {
            url: "https://www.aljazeera.com/xml/rss/all.xml",
            category: "World News",
            source_name: "Al Jazeera",
        },
        // Indonesia (Added for completeness based on user language)
        FeedSource {
            url: "https://www.antaranews.com/rss/terpopuler.xml",
            category: "Indonesia",
            source_name: "Antara News",
        },
        FeedSource {
            url: "https://rss.detik.com/index.php/detikcom",
            category: "Indonesia",
            source_name: "Detikcom",
        },
        FeedSource {
            url: "https://www.kompas.com/tag/berita-terbaru/rss",
            category: "Indonesia",
            source_name: "Kompas",
        },
        // Business
        FeedSource {
            url: "https://feeds.a.dj.com/rss/WSJcomUSBusiness.xml",
            category: "Business",
            source_name: "WSJ",
        },
        FeedSource {
            url: "http://feeds.bloomberg.com/markets/news.rss",
            category: "Business",
            source_name: "Bloomberg",
        },
        // Entertainment
        FeedSource {
            url: "https://www.ign.com/rss/articles/feed",
            category: "Entertainment",
            source_name: "IGN",
        },
    ]
}

pub fn get_categories() -> Vec<&'static str> {
    vec![
        "All",
        "Indonesia",
        "Tech",
        "AI",
        "World News",
        "Business",
        "Entertainment",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_get_categories_basic() {
        let categories = get_categories();
        assert_eq!(categories.len(), 7);
        assert!(categories.contains(&"All"));
        assert!(categories.contains(&"Tech"));
        assert!(categories.contains(&"AI"));
        assert!(categories.contains(&"World News"));
        assert!(categories.contains(&"Business"));
        assert!(categories.contains(&"Indonesia"));
        assert!(categories.contains(&"Entertainment"));
    }

    #[test]
    fn test_get_categories_contains_all_sources() {
        let categories = get_categories();
        let sources = get_sources();

        for source in sources {
            assert!(
                categories.contains(&source.category),
                "Category '{}' from source '{}' not found in get_categories()",
                source.category,
                source.source_name
            );
        }
    }

    #[test]
    fn test_get_categories_uniqueness() {
        let categories = get_categories();
        let mut set = HashSet::new();
        for category in &categories {
            assert!(
                set.insert(category),
                "Duplicate category found: {}",
                category
            );
        }
    }

    #[test]
    fn test_get_categories_starts_with_all() {
        let categories = get_categories();
        assert!(!categories.is_empty());
        assert_eq!(categories[0], "All");
    }
}
