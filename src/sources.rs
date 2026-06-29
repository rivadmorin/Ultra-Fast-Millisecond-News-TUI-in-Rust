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
            url: "https://www.engadget.com/rss.xml",
            category: "Tech",
            source_name: "Engadget",
        },
        FeedSource {
            url: "https://9to5mac.com/feed/",
            category: "Tech",
            source_name: "9to5Mac",
        },
        FeedSource {
            url: "https://android-developers.googleblog.com/feeds/posts/default",
            category: "Tech",
            source_name: "Android Developers",
        },
        FeedSource {
            url: "https://developer.apple.com/news/rss/news.rss",
            category: "Tech",
            source_name: "Apple Developer",
        },
        // AI & Data Science
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
            url: "https://machinelearningmastery.com/feed/",
            category: "AI",
            source_name: "ML Mastery",
        },
        FeedSource {
            url: "https://ai.googleblog.com/feeds/posts/default",
            category: "AI",
            source_name: "Google AI",
        },
        FeedSource {
            url: "https://pytorch.org/feed.xml",
            category: "AI",
            source_name: "PyTorch",
        },
        // Indonesia
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
        FeedSource {
            url: "https://lapi.tempo.co/feeds/rss/nasional",
            category: "Indonesia",
            source_name: "Tempo",
        },
        FeedSource {
            url: "https://www.cnnindonesia.com/nasional/rss",
            category: "Indonesia",
            source_name: "CNN Indonesia",
        },
        FeedSource {
            url: "https://nasional.republika.co.id/rss",
            category: "Indonesia",
            source_name: "Republika",
        },
        FeedSource {
            url: "https://www.suara.com/rss/news",
            category: "Indonesia",
            source_name: "Suara.com",
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
            url: "https://www.reutersagency.com/feed/?best-topics=world-news&post_type=best",
            category: "World News",
            source_name: "Reuters",
        },
        FeedSource {
            url: "https://feeds.npr.org/1001/rss.xml",
            category: "World News",
            source_name: "NPR",
        },
        FeedSource {
            url: "https://www.dw.com/xml/rss-en-all",
            category: "World News",
            source_name: "DW",
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
        FeedSource {
            url: "https://www.forbes.com/business/feed/",
            category: "Business",
            source_name: "Forbes",
        },
        FeedSource {
            url: "https://feeds.feedburner.com/economist/full_print_edition",
            category: "Business",
            source_name: "The Economist",
        },
        FeedSource {
            url: "https://cnbc.com/id/10001147/device/rss/rss.html",
            category: "Business",
            source_name: "CNBC",
        },
        // Science
        FeedSource {
            url: "https://www.sciencedaily.com/rss/all.xml",
            category: "Science",
            source_name: "Science Daily",
        },
        FeedSource {
            url: "https://www.nature.com/nature.rss",
            category: "Science",
            source_name: "Nature",
        },
        FeedSource {
            url: "https://www.scientificamerican.com/rss/scientific-american/",
            category: "Science",
            source_name: "Scientific American",
        },
        FeedSource {
            url: "https://phys.org/rss-feed/",
            category: "Science",
            source_name: "Phys.org",
        },
        FeedSource {
            url: "https://www.nasa.gov/rss/dyn/breaking_news.rss",
            category: "Science",
            source_name: "NASA",
        },
        // Sports
        FeedSource {
            url: "https://www.espn.com/espn/rss/news",
            category: "Sports",
            source_name: "ESPN",
        },
        FeedSource {
            url: "https://feeds.bbci.co.uk/sport/rss.xml",
            category: "Sports",
            source_name: "BBC Sport",
        },
        FeedSource {
            url: "https://nfltraderumors.co/feed/",
            category: "Sports",
            source_name: "NFL Rumors",
        },
        FeedSource {
            url: "https://www.fifa.com/rss/index.xml",
            category: "Sports",
            source_name: "FIFA",
        },
        // Entertainment
        FeedSource {
            url: "https://www.ign.com/rss/articles/feed",
            category: "Entertainment",
            source_name: "IGN",
        },
        FeedSource {
            url: "https://variety.com/feed/",
            category: "Entertainment",
            source_name: "Variety",
        },
        FeedSource {
            url: "https://www.hollywoodreporter.com/feed/",
            category: "Entertainment",
            source_name: "THR",
        },
        FeedSource {
            url: "https://deadline.com/feed/",
            category: "Entertainment",
            source_name: "Deadline",
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
        "Science",
        "Sports",
        "Entertainment",
    ]
}
