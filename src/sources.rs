pub struct FeedSource {
    pub url: &'static str,
    pub category: &'static str,
    pub source_name: &'static str,
}

pub fn get_sources() -> Vec<FeedSource> {
    vec![
        // Tech & AI
        FeedSource { url: "https://news.ycombinator.com/rss", category: "Tech", source_name: "Hacker News" },
        FeedSource { url: "https://techcrunch.com/feed/", category: "Tech", source_name: "TechCrunch" },
        FeedSource { url: "https://www.theverge.com/rss/index.xml", category: "Tech", source_name: "The Verge" },
        FeedSource { url: "https://wired.com/feed/rss", category: "Tech", source_name: "Wired" },
        FeedSource { url: "https://openai.com/blog/rss.xml", category: "AI", source_name: "OpenAI" },
        FeedSource { url: "https://deepmind.google/blog/rss.xml", category: "AI", source_name: "DeepMind" },

        // Finance & Business
        FeedSource { url: "https://www.ft.com/?format=rss", category: "Finance", source_name: "Financial Times" },
        FeedSource { url: "https://feeds.a.dj.com/rss/WSJcomUSBusiness.xml", category: "Finance", source_name: "WSJ Business" },
        FeedSource { url: "https://www.cnbc.com/id/10000664/device/rss/rss.html", category: "Finance", source_name: "CNBC Finance" },
        FeedSource { url: "http://feeds.bloomberg.com/markets/news.rss", category: "Finance", source_name: "Bloomberg" },
        FeedSource { url: "https://www.economist.com/finance-and-economics/rss.xml", category: "Finance", source_name: "The Economist" },
        FeedSource { url: "https://www.investing.com/rss/news.rss", category: "Finance", source_name: "Investing.com" },

        // Geopolitics & World
        FeedSource { url: "http://feeds.bbci.co.uk/news/rss.xml", category: "World", source_name: "BBC News" },
        FeedSource { url: "https://rss.nytimes.com/services/xml/rss/nyt/World.xml", category: "World", source_name: "NYT World" },
        FeedSource { url: "https://www.aljazeera.com/xml/rss/all.xml", category: "World", source_name: "Al Jazeera" },
        FeedSource { url: "https://www.theguardian.com/world/rss", category: "World", source_name: "The Guardian" },
        FeedSource { url: "https://www.reutersagency.com/feed/?best-types=political-news&post_type=best", category: "World", source_name: "Reuters Politics" },
        FeedSource { url: "https://www.scmp.com/rss/91/feed", category: "World", source_name: "SCMP" },

        // Crypto
        FeedSource { url: "https://www.coindesk.com/arc/outboundfeeds/rss/", category: "Crypto", source_name: "CoinDesk" },
        FeedSource { url: "https://cointelegraph.com/rss", category: "Crypto", source_name: "CoinTelegraph" },
        FeedSource { url: "https://bitcoinmagazine.com/.rss/full/", category: "Crypto", source_name: "Bitcoin Magazine" },

        // Science & Health
        FeedSource { url: "https://www.nasa.gov/feed/", category: "Science", source_name: "NASA" },
        FeedSource { url: "https://www.sciencedaily.com/rss/all.xml", category: "Science", source_name: "Science Daily" },
        FeedSource { url: "https://www.nature.com/nature.rss", category: "Science", source_name: "Nature" },
        FeedSource { url: "https://www.healthline.com/feed", category: "Health", source_name: "Healthline" },

        // Lifestyle & Culture
        FeedSource { url: "https://www.vogue.com/feed/rss", category: "Lifestyle", source_name: "Vogue" },
        FeedSource { url: "https://www.gq.com/feed/rss", category: "Lifestyle", source_name: "GQ" },
        FeedSource { url: "https://www.nationalgeographic.com/rss/index.xml", category: "Lifestyle", source_name: "NatGeo" },
        FeedSource { url: "https://www.rollingstone.com/feed/", category: "Lifestyle", source_name: "Rolling Stone" },

        // Sports
        FeedSource { url: "https://www.espn.com/espn/rss/news", category: "Sports", source_name: "ESPN" },
        FeedSource { url: "https://feeds.bbci.co.uk/sport/rss.xml", category: "Sports", source_name: "BBC Sport" },

        // Indonesia
        FeedSource { url: "https://www.antaranews.com/rss/terpopuler.xml", category: "Indonesia", source_name: "Antara News" },
        FeedSource { url: "https://rss.detik.com/index.php/detikcom", category: "Indonesia", source_name: "Detikcom" },
        FeedSource { url: "https://www.kompas.com/tag/berita-terbaru/rss", category: "Indonesia", source_name: "Kompas" },
        FeedSource { url: "https://www.cnnindonesia.com/ekonomi/rss", category: "Indonesia", source_name: "CNN ID Ekonomi" },
        FeedSource { url: "https://www.liputan6.com/rss", category: "Indonesia", source_name: "Liputan6" },
        FeedSource { url: "https://www.merdeka.com/rss", category: "Indonesia", source_name: "Merdeka.com" },
    ]
}

pub fn get_categories() -> Vec<&'static str> {
    vec![
        "All",
        "Indonesia",
        "World",
        "Finance",
        "Tech",
        "AI",
        "Crypto",
        "Science",
        "Health",
        "Lifestyle",
        "Sports",
    ]
}
