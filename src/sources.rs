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
        FeedSource {
            url: "https://engadget.com/rss.xml",
            category: "Tech",
            source_name: "Engadget",
        },
        // AI
        FeedSource {
            url: "https://bair.berkeley.edu/blog/feed.xml",
            category: "AI",
            source_name: "BAIR",
        },
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
        FeedSource {
            url: "https://export.arxiv.org/rss/cs.AI",
            category: "AI",
            source_name: "arXiv (CS.AI)",
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
        FeedSource {
            url: "http://rss.cnn.com/rss/edition.rss",
            category: "World News",
            source_name: "CNN",
        },
        // Business
        FeedSource {
            url: "https://feeds.a.dj.com/rss/WSJcomUSBusiness.xml",
            category: "Business",
            source_name: "WSJ",
        },
        FeedSource {
            url: "https://www.ft.com/?format=rss",
            category: "Business",
            source_name: "Financial Times",
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
        FeedSource {
            url: "https://www.polygon.com/rss/index.xml",
            category: "Entertainment",
            source_name: "Polygon",
        },
    ]
}

pub fn get_categories() -> Vec<&'static str> {
    vec![
        "All",
        "Tech",
        "AI",
        "World News",
        "Business",
        "Entertainment",
    ]
}
