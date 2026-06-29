mod app;
mod config;
mod db;
mod fetcher;
mod sources;
mod ui;
mod scraper;

use app::{App, AppEvent};
use config::Config;
use db::Db;
use directories::ProjectDirs;
use std::{error::Error, io, sync::Arc, env};

use crossterm::{
    event::{self, Event as CEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--scrape" {
        let url = if args.len() > 2 { &args[2] } else { "https://techcrunch.com/2024/03/18/nvidia-blackwell-gpu-b200/" };
        println!("Scraping URL: {}", url);
        scraper::run_example_scraper(url)?;
        return Ok(());
    }

    let config = Config::load();

    let db_path = if let Some(proj_dirs) = ProjectDirs::from("com", "LiveNewsTUI", "LiveNews") {
        let db_dir = proj_dirs.data_local_dir();
        let _ = std::fs::create_dir_all(db_dir);
        db_dir.join("news.db")
    } else {
        std::path::PathBuf::from("news.db")
    };

    let db = Arc::new(Db::new(db_path)?);

    let db_cleanup = Arc::clone(&db);
    let policy = config.retention.clone();
    tokio::spawn(async move {
        let _ = db_cleanup.cleanup_old_data(&policy);
    });

    let fetch_db = Arc::clone(&db);
    let fetch_config = config.clone();
    tokio::spawn(async move {
        fetcher::start_fetcher(fetch_db, fetch_config).await;
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let tick_rate = Duration::from_millis(200);

    let tick_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tick_rate).await;
            if tick_tx.send(AppEvent::Tick).is_err() {
                break;
            }
        }
    });

    tokio::spawn(async move {
        loop {
            #[allow(clippy::collapsible_if)]
            if let Ok(true) = crossterm::event::poll(Duration::from_millis(100)) {
                if let Ok(CEvent::Key(key)) = event::read() {
                    if tx.send(AppEvent::Input(key)).is_ok() {
                        // Successfully sent
                    }
                }
            }
        }
    });

    let mut app = App::new(db, config.theme);
    app.on_tick();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if let Some(event) = rx.recv().await {
            match event {
                AppEvent::Input(key) => app.on_key(key),
                AppEvent::Tick => app.on_tick(),
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
