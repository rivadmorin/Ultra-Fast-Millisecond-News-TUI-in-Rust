pub struct FeedSource {
    pub url: &'static str,
    pub category: &'static str,
    pub source_name: &'static str,
}

pub fn get_sources() -> Vec<FeedSource> {
    vec![
        // ==========================================
        // INDONESIA (Premium & Regional)
        // ==========================================
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
            url: "https://www.antaranews.com/rss/terpopuler.xml",
            category: "Indonesia",
            source_name: "Antara News",
        },
        FeedSource {
            url: "https://www.cnnindonesia.com/ekonomi/rss",
            category: "Indonesia",
            source_name: "CNN Ekonomi",
        },
        FeedSource {
            url: "https://www.liputan6.com/rss",
            category: "Indonesia",
            source_name: "Liputan6",
        },
        FeedSource {
            url: "https://www.merdeka.com/rss",
            category: "Indonesia",
            source_name: "Merdeka",
        },
        FeedSource {
            url: "https://www.tempo.co/rss/nasional",
            category: "Indonesia",
            source_name: "Tempo Nasional",
        },
        FeedSource {
            url: "https://www.viva.co.id/rss/all",
            category: "Indonesia",
            source_name: "Viva News",
        },
        FeedSource {
            url: "https://www.republika.co.id/rss",
            category: "Indonesia",
            source_name: "Republika",
        },
        FeedSource {
            url: "https://www.suara.com/rss/news",
            category: "Indonesia",
            source_name: "Suara.com",
        },
        FeedSource {
            url: "https://www.tribunnews.com/rss",
            category: "Indonesia",
            source_name: "Tribunnews",
        },
        FeedSource {
            url: "https://www.okezone.com/rss/news.xml",
            category: "Indonesia",
            source_name: "Okezone",
        },
        FeedSource {
            url: "https://www.jawapos.com/rss",
            category: "Indonesia",
            source_name: "Jawa Pos",
        },
        FeedSource {
            url: "https://www.idntimes.com/feed",
            category: "Indonesia",
            source_name: "IDN Times",
        },
        FeedSource {
            url: "https://www.beritasatu.com/rss",
            category: "Indonesia",
            source_name: "BeritaSatu",
        },
        FeedSource {
            url: "https://www.inews.id/rss",
            category: "Indonesia",
            source_name: "iNews",
        },
        FeedSource {
            url: "https://www.kumparan.com/rss",
            category: "Indonesia",
            source_name: "Kumparan",
        },
        FeedSource {
            url: "https://www.bisnis.com/rss",
            category: "Indonesia",
            source_name: "Bisnis.com",
        },
        FeedSource {
            url: "https://www.cnbcindonesia.com/news/rss",
            category: "Indonesia",
            source_name: "CNBC ID",
        },
        FeedSource {
            url: "https://www.kontan.co.id/rss",
            category: "Indonesia",
            source_name: "Kontan",
        },
        FeedSource {
            url: "https://www.inilah.com/feed",
            category: "Indonesia",
            source_name: "Inilah",
        },
        FeedSource {
            url: "https://www.poskota.co.id/rss",
            category: "Indonesia",
            source_name: "Poskota",
        },
        FeedSource {
            url: "https://www.solopos.com/feed",
            category: "Indonesia",
            source_name: "Solopos",
        },
        FeedSource {
            url: "https://www.hallo.id/rss",
            category: "Indonesia",
            source_name: "Hallo.id",
        },
        FeedSource {
            url: "https://www.kapanlagi.com/feed/",
            category: "Indonesia",
            source_name: "KapanLagi",
        },
        FeedSource {
            url: "https://www.pikiran-rakyat.com/rss",
            category: "Indonesia",
            source_name: "Pikiran Rakyat",
        },
        FeedSource {
            url: "https://www.grid.id/rss",
            category: "Indonesia",
            source_name: "Grid",
        },
        FeedSource {
            url: "https://www.suaramerdeka.com/rss",
            category: "Indonesia",
            source_name: "Suara Merdeka",
        },
        FeedSource {
            url: "https://www.merdeka.com/peristiwa/rss",
            category: "Indonesia",
            source_name: "Merdeka Peristiwa",
        },
        // ==========================================
        // WORLD & GEOPOLITICS
        // ==========================================
        FeedSource {
            url: "http://feeds.bbci.co.uk/news/rss.xml",
            category: "World",
            source_name: "BBC News",
        },
        FeedSource {
            url: "https://rss.nytimes.com/services/xml/rss/nyt/World.xml",
            category: "World",
            source_name: "NYT World",
        },
        FeedSource {
            url: "https://www.aljazeera.com/xml/rss/all.xml",
            category: "World",
            source_name: "Al Jazeera",
        },
        FeedSource {
            url: "https://www.theguardian.com/world/rss",
            category: "World",
            source_name: "The Guardian",
        },
        FeedSource {
            url: "https://www.reutersagency.com/feed/?best-types=political-news&post_type=best",
            category: "World",
            source_name: "Reuters",
        },
        FeedSource {
            url: "https://www.scmp.com/rss/91/feed",
            category: "World",
            source_name: "SCMP",
        },
        FeedSource {
            url: "https://www.france24.com/en/rss",
            category: "World",
            source_name: "France 24",
        },
        FeedSource {
            url: "https://www.dw.com/en/top-stories/s-9097/rss",
            category: "World",
            source_name: "DW",
        },
        FeedSource {
            url: "https://www.washingtonpost.com/arc/outboundfeeds/rss/world/",
            category: "World",
            source_name: "Washington Post",
        },
        FeedSource {
            url: "https://www.asahi.com/english/rss/",
            category: "World",
            source_name: "Asahi Shimbun",
        },
        FeedSource {
            url: "https://www.japantimes.co.jp/feed/",
            category: "World",
            source_name: "Japan Times",
        },
        FeedSource {
            url: "https://www.rt.com/rss/news/",
            category: "World",
            source_name: "RT News",
        },
        FeedSource {
            url: "https://www.euronews.com/rss?format=mrss&level=vertical&name=news",
            category: "World",
            source_name: "EuroNews",
        },
        FeedSource {
            url: "https://www.foreignaffairs.com/rss.xml",
            category: "World",
            source_name: "Foreign Affairs",
        },
        FeedSource {
            url: "https://www.cfr.org/rss/news-releases.xml",
            category: "World",
            source_name: "CFR",
        },
        FeedSource {
            url: "https://www.un.org/press/en/feed",
            category: "World",
            source_name: "UN News",
        },
        FeedSource {
            url: "https://www.voanews.com/api/z-",
            category: "World",
            source_name: "VOA News",
        },
        FeedSource {
            url: "https://www.thestar.com.my/rss/news/world/",
            category: "World",
            source_name: "The Star",
        },
        FeedSource {
            url: "https://www.taipeitimes.com/rss/news",
            category: "World",
            source_name: "Taipei Times",
        },
        FeedSource {
            url: "https://www.moscowtimes.ru/rss/news",
            category: "World",
            source_name: "Moscow Times",
        },
        FeedSource {
            url: "https://www.timesofisrael.com/feed/",
            category: "World",
            source_name: "Times of Israel",
        },
        FeedSource {
            url: "https://www.independent.co.uk/news/world/rss",
            category: "World",
            source_name: "Independent",
        },
        FeedSource {
            url: "https://www.politico.com/rss/politicopicks.xml",
            category: "World",
            source_name: "Politico",
        },
        FeedSource {
            url: "https://www.defense.gov/DesktopModules/ArticleCS/RSS.ashx?ContentType=1&Site=945&max=10",
            category: "World",
            source_name: "US Defense",
        },
        FeedSource {
            url: "https://www.thejournal.ie/feed/",
            category: "World",
            source_name: "The Journal",
        },
        FeedSource {
            url: "https://english.kyodonews.net/rss/news.xml",
            category: "World",
            source_name: "Kyodo News",
        },
        FeedSource {
            url: "https://www.theonion.com/rss",
            category: "World",
            source_name: "The Onion",
        },
        // ==========================================
        // FINANCE & ECONOMY
        // ==========================================
        FeedSource {
            url: "http://feeds.bloomberg.com/markets/news.rss",
            category: "Finance",
            source_name: "Bloomberg",
        },
        FeedSource {
            url: "https://feeds.a.dj.com/rss/WSJcomUSBusiness.xml",
            category: "Finance",
            source_name: "WSJ Business",
        },
        FeedSource {
            url: "https://www.ft.com/?format=rss",
            category: "Finance",
            source_name: "Financial Times",
        },
        FeedSource {
            url: "https://www.cnbc.com/id/10000664/device/rss/rss.html",
            category: "Finance",
            source_name: "CNBC",
        },
        FeedSource {
            url: "https://www.economist.com/finance-and-economics/rss.xml",
            category: "Finance",
            source_name: "The Economist",
        },
        FeedSource {
            url: "https://www.investing.com/rss/news.rss",
            category: "Finance",
            source_name: "Investing.com",
        },
        FeedSource {
            url: "https://www.forbes.com/business/feed/",
            category: "Finance",
            source_name: "Forbes",
        },
        FeedSource {
            url: "https://www.businessinsider.com/rss",
            category: "Finance",
            source_name: "Business Insider",
        },
        FeedSource {
            url: "https://feeds.feedburner.com/zerohedge/feed",
            category: "Finance",
            source_name: "ZeroHedge",
        },
        FeedSource {
            url: "https://www.marketwatch.com/rss/topstories",
            category: "Finance",
            source_name: "MarketWatch",
        },
        FeedSource {
            url: "https://finance.yahoo.com/news/rssindex",
            category: "Finance",
            source_name: "Yahoo Finance",
        },
        FeedSource {
            url: "https://www.kiplinger.com/index.xml",
            category: "Finance",
            source_name: "Kiplinger",
        },
        FeedSource {
            url: "https://www.worldbank.org/en/news/rss.xml",
            category: "Finance",
            source_name: "World Bank",
        },
        FeedSource {
            url: "https://www.imf.org/en/news/rss",
            category: "Finance",
            source_name: "IMF News",
        },
        FeedSource {
            url: "https://www.ibtimes.com/rss",
            category: "Finance",
            source_name: "IB Times",
        },
        FeedSource {
            url: "https://www.benzinga.com/feed",
            category: "Finance",
            source_name: "Benzinga",
        },
        FeedSource {
            url: "https://www.moneycontrol.com/rss/latestnews.xml",
            category: "Finance",
            source_name: "MoneyControl",
        },
        FeedSource {
            url: "https://www.fxstreet.com/rss/news",
            category: "Finance",
            source_name: "FXStreet",
        },
        FeedSource {
            url: "https://www.wallstreetmojo.com/feed/",
            category: "Finance",
            source_name: "WallStreetMojo",
        },
        // ==========================================
        // TECH & INNOVATION
        // ==========================================
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
            url: "https://arstechnica.com/feed/",
            category: "Tech",
            source_name: "Ars Technica",
        },
        FeedSource {
            url: "https://www.zdnet.com/news/rss.xml",
            category: "Tech",
            source_name: "ZDNet",
        },
        FeedSource {
            url: "https://www.engadget.com/rss.xml",
            category: "Tech",
            source_name: "Engadget",
        },
        FeedSource {
            url: "https://towardsdatascience.com/feed",
            category: "AI",
            source_name: "Towards Data Science",
        },
        FeedSource {
            url: "https://www.technologyreview.com/feed/",
            category: "Tech",
            source_name: "MIT Tech Review",
        },
        FeedSource {
            url: "https://www.nextbigfuture.com/feed",
            category: "Tech",
            source_name: "Next Big Future",
        },
        FeedSource {
            url: "https://www.techradar.com/rss",
            category: "Tech",
            source_name: "TechRadar",
        },
        FeedSource {
            url: "https://www.slashgear.com/feed/",
            category: "Tech",
            source_name: "SlashGear",
        },
        FeedSource {
            url: "https://www.digitaltrends.com/feed/",
            category: "Tech",
            source_name: "Digital Trends",
        },
        FeedSource {
            url: "https://9to5mac.com/feed/",
            category: "Tech",
            source_name: "9to5Mac",
        },
        FeedSource {
            url: "https://www.androidcentral.com/feed",
            category: "Tech",
            source_name: "Android Central",
        },
        FeedSource {
            url: "https://www.venturebeat.com/feed/",
            category: "Tech",
            source_name: "VentureBeat",
        },
        FeedSource {
            url: "https://www.theblock.co/rss",
            category: "Tech",
            source_name: "The Block",
        },
        FeedSource {
            url: "https://www.artificialintelligence-news.com/feed/",
            category: "AI",
            source_name: "AI News",
        },
        FeedSource {
            url: "https://www.ubergizmo.com/feed/",
            category: "Tech",
            source_name: "Ubergizmo",
        },
        FeedSource {
            url: "https://www.tomsguide.com/rss",
            category: "Tech",
            source_name: "Toms Guide",
        },
        FeedSource {
            url: "https://www.howtogeek.com/feed/",
            category: "Tech",
            source_name: "HowToGeek",
        },
        // ==========================================
        // CRYPTO
        // ==========================================
        FeedSource {
            url: "https://www.coindesk.com/arc/outboundfeeds/rss/",
            category: "Crypto",
            source_name: "CoinDesk",
        },
        FeedSource {
            url: "https://cointelegraph.com/rss",
            category: "Crypto",
            source_name: "CoinTelegraph",
        },
        FeedSource {
            url: "https://bitcoinmagazine.com/.rss/full/",
            category: "Crypto",
            source_name: "Bitcoin Magazine",
        },
        FeedSource {
            url: "https://cryptoslate.com/feed/",
            category: "Crypto",
            source_name: "CryptoSlate",
        },
        FeedSource {
            url: "https://decrypt.co/feed",
            category: "Crypto",
            source_name: "Decrypt",
        },
        FeedSource {
            url: "https://news.bitcoin.com/feed/",
            category: "Crypto",
            source_name: "Bitcoin.com",
        },
        FeedSource {
            url: "https://blockworks.co/feed",
            category: "Crypto",
            source_name: "Blockworks",
        },
        FeedSource {
            url: "https://ambcrypto.com/feed/",
            category: "Crypto",
            source_name: "AMBCrypto",
        },
        FeedSource {
            url: "https://beincrypto.com/feed/",
            category: "Crypto",
            source_name: "BeInCrypto",
        },
        FeedSource {
            url: "https://www.newsbtc.com/feed/",
            category: "Crypto",
            source_name: "NewsBTC",
        },
        FeedSource {
            url: "https://www.trustnodes.com/feed",
            category: "Crypto",
            source_name: "TrustNodes",
        },
        FeedSource {
            url: "https://dailyhodl.com/feed/",
            category: "Crypto",
            source_name: "DailyHodl",
        },
        // ==========================================
        // SCIENCE & HEALTH
        // ==========================================
        FeedSource {
            url: "https://www.nasa.gov/feed/",
            category: "Science",
            source_name: "NASA",
        },
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
            url: "https://www.scientificamerican.com/rss/all",
            category: "Science",
            source_name: "Scientific American",
        },
        FeedSource {
            url: "https://www.healthline.com/feed",
            category: "Health",
            source_name: "Healthline",
        },
        FeedSource {
            url: "https://www.medicalnewstoday.com/feed",
            category: "Health",
            source_name: "Medical News Today",
        },
        FeedSource {
            url: "https://www.webmd.com/rss/public/news.xml",
            category: "Health",
            source_name: "WebMD",
        },
        FeedSource {
            url: "https://www.nih.gov/news-events/news-releases?format=rss",
            category: "Health",
            source_name: "NIH",
        },
        FeedSource {
            url: "https://www.who.int/rss-feeds/news-english.xml",
            category: "Health",
            source_name: "WHO",
        },
        FeedSource {
            url: "https://www.livescience.com/feed",
            category: "Science",
            source_name: "LiveScience",
        },
        FeedSource {
            url: "https://www.smithsonianmag.com/rss/all/",
            category: "Science",
            source_name: "Smithsonian",
        },
        FeedSource {
            url: "https://feeds.feedburner.com/EnvironmentalNewsNetwork",
            category: "Science",
            source_name: "ENN",
        },
        FeedSource {
            url: "https://www.popsci.com/feed/",
            category: "Science",
            source_name: "PopSci",
        },
        FeedSource {
            url: "https://www.quantamagazine.org/feed/",
            category: "Science",
            source_name: "Quanta",
        },
        FeedSource {
            url: "https://www.phys.org/rss-feed/",
            category: "Science",
            source_name: "Phys.org",
        },
        FeedSource {
            url: "https://www.psychologytoday.com/us/front/feed",
            category: "Health",
            source_name: "Psychology Today",
        },
        // ==========================================
        // LIFESTYLE & CULTURE
        // ==========================================
        FeedSource {
            url: "https://www.vogue.com/feed/rss",
            category: "Lifestyle",
            source_name: "Vogue",
        },
        FeedSource {
            url: "https://www.gq.com/feed/rss",
            category: "Lifestyle",
            source_name: "GQ",
        },
        FeedSource {
            url: "https://www.nationalgeographic.com/rss/index.xml",
            category: "Lifestyle",
            source_name: "NatGeo",
        },
        FeedSource {
            url: "https://www.rollingstone.com/feed/",
            category: "Lifestyle",
            source_name: "Rolling Stone",
        },
        FeedSource {
            url: "https://www.vanityfair.com/feed/rss",
            category: "Lifestyle",
            source_name: "Vanity Fair",
        },
        FeedSource {
            url: "https://www.newyorker.com/feed/everything",
            category: "Lifestyle",
            source_name: "The New Yorker",
        },
        FeedSource {
            url: "https://www.travelandleisure.com/rss",
            category: "Lifestyle",
            source_name: "Travel + Leisure",
        },
        FeedSource {
            url: "https://www.foodandwine.com/rss",
            category: "Lifestyle",
            source_name: "Food & Wine",
        },
        FeedSource {
            url: "https://www.architecturaldigest.com/feed/rss",
            category: "Lifestyle",
            source_name: "Arch Digest",
        },
        FeedSource {
            url: "https://www.cntraveller.com/rss/all",
            category: "Lifestyle",
            source_name: "CN Traveller",
        },
        FeedSource {
            url: "https://www.harpersbazaar.com/rss/all.xml",
            category: "Lifestyle",
            source_name: "Harpers Bazaar",
        },
        FeedSource {
            url: "https://www.bonappetit.com/feed/rss",
            category: "Lifestyle",
            source_name: "Bon Appetit",
        },
        FeedSource {
            url: "https://www.elle.com/rss/all.xml",
            category: "Lifestyle",
            source_name: "Elle",
        },
        FeedSource {
            url: "https://www.esquire.com/rss/all.xml",
            category: "Lifestyle",
            source_name: "Esquire",
        },
        FeedSource {
            url: "https://www.highsnobiety.com/feed/",
            category: "Lifestyle",
            source_name: "Highsnobiety",
        },
        // ==========================================
        // SPORTS
        // ==========================================
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
            url: "https://www.skysports.com/rss/12040",
            category: "Sports",
            source_name: "Sky Sports",
        },
        FeedSource {
            url: "https://www.theathletic.com/rss",
            category: "Sports",
            source_name: "The Athletic",
        },
        FeedSource {
            url: "https://www.cbssports.com/rss/headlines/",
            category: "Sports",
            source_name: "CBS Sports",
        },
        FeedSource {
            url: "https://www.nbcsports.com/feed",
            category: "Sports",
            source_name: "NBC Sports",
        },
        FeedSource {
            url: "https://www.fifa.com/rss/news.xml",
            category: "Sports",
            source_name: "FIFA",
        },
        FeedSource {
            url: "https://www.nba.com/rss/nba_rss.xml",
            category: "Sports",
            source_name: "NBA",
        },
        FeedSource {
            url: "https://www.motorsport.com/rss/all/news/",
            category: "Sports",
            source_name: "Motorsport",
        },
        FeedSource {
            url: "https://www.nfl.com/rss/rsslanding?searchString=home",
            category: "Sports",
            source_name: "NFL",
        },
        FeedSource {
            url: "https://www.mlb.com/feeds/news/all.xml",
            category: "Sports",
            source_name: "MLB",
        },
        FeedSource {
            url: "https://www.tennis.com/rss/news/",
            category: "Sports",
            source_name: "Tennis.com",
        },
        FeedSource {
            url: "https://www.golfchannel.com/rss/news",
            category: "Sports",
            source_name: "Golf Channel",
        },
        FeedSource {
            url: "https://www.cyclingnews.com/rss/all/",
            category: "Sports",
            source_name: "CyclingNews",
        },
        // ==========================================
        // ENTERTAINMENT & GAMING
        // ==========================================
        FeedSource {
            url: "https://www.ign.com/rss/articles/feed",
            category: "Entertainment",
            source_name: "IGN",
        },
        FeedSource {
            url: "https://www.gamespot.com/feeds/news/",
            category: "Entertainment",
            source_name: "GameSpot",
        },
        FeedSource {
            url: "https://variety.com/feed/",
            category: "Entertainment",
            source_name: "Variety",
        },
        FeedSource {
            url: "https://www.hollywoodreporter.com/feed/",
            category: "Entertainment",
            source_name: "Hollywood Reporter",
        },
        FeedSource {
            url: "https://www.tmz.com/rss.xml",
            category: "Entertainment",
            source_name: "TMZ",
        },
        FeedSource {
            url: "https://www.rollingstone.com/music/feed/",
            category: "Entertainment",
            source_name: "RS Music",
        },
        FeedSource {
            url: "https://www.kotaku.com/rss",
            category: "Gaming",
            source_name: "Kotaku",
        },
        FeedSource {
            url: "https://www.polygon.com/rss/index.xml",
            category: "Gaming",
            source_name: "Polygon",
        },
        FeedSource {
            url: "https://www.pcgamer.com/rss",
            category: "Gaming",
            source_name: "PC Gamer",
        },
        FeedSource {
            url: "https://www.destructoid.com/feed/",
            category: "Gaming",
            source_name: "Destructoid",
        },
        FeedSource {
            url: "https://www.nintendolife.com/feeds/latest",
            category: "Gaming",
            source_name: "Nintendo Life",
        },
        FeedSource {
            url: "https://www.rockpapershotgun.com/feed",
            category: "Gaming",
            source_name: "RPS",
        },
        FeedSource {
            url: "https://www.gematsu.com/feed",
            category: "Gaming",
            source_name: "Gematsu",
        },
        FeedSource {
            url: "https://www.eurogamer.net/feed",
            category: "Gaming",
            source_name: "Eurogamer",
        },
        FeedSource {
            url: "https://www.engadget.com/gaming/rss.xml",
            category: "Gaming",
            source_name: "Engadget Gaming",
        },
        // ==========================================
        // AUTO & LEGAL
        // ==========================================
        FeedSource {
            url: "https://www.autoblog.com/rss.xml",
            category: "Auto",
            source_name: "Autoblog",
        },
        FeedSource {
            url: "https://www.caranddriver.com/rss/all.xml",
            category: "Auto",
            source_name: "Car and Driver",
        },
        FeedSource {
            url: "https://www.motortrend.com/rss/",
            category: "Auto",
            source_name: "MotorTrend",
        },
        FeedSource {
            url: "https://www.autocar.co.uk/rss/news",
            category: "Auto",
            source_name: "AutoCar",
        },
        FeedSource {
            url: "https://www.jalopnik.com/rss",
            category: "Auto",
            source_name: "Jalopnik",
        },
        FeedSource {
            url: "https://www.scotusblog.com/feed/",
            category: "Legal",
            source_name: "SCOTUSblog",
        },
        FeedSource {
            url: "https://www.law.com/rss/news/",
            category: "Legal",
            source_name: "Law.com",
        },
        FeedSource {
            url: "https://www.abajournal.com/magazine/rss/",
            category: "Legal",
            source_name: "ABA Journal",
        },
        FeedSource {
            url: "https://www.justice.gov/news-releases.xml",
            category: "Legal",
            source_name: "DOJ News",
        },
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
        "Entertainment",
        "Gaming",
        "Auto",
        "Legal",
    ]
}
