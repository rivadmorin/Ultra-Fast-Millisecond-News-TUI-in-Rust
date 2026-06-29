mod app;
mod config;
mod db;
mod fetcher;
mod sources;
mod ui;

use app::{App, AppEvent};
use config::Config;
use db::Db;
use directories::ProjectDirs;
use std::{error::Error, io, sync::Arc};

use crossterm::{
    event::{self, Event as CEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Basic setup
    let config = Config::default();

    // Setup DB Path
    let db_path = if let Some(proj_dirs) = ProjectDirs::from("com", "LiveNewsTUI", "LiveNews") {
        let db_dir = proj_dirs.data_local_dir();
        std::fs::create_dir_all(db_dir)?;
        db_dir.join("news.db")
    } else {
        std::path::PathBuf::from("news.db")
    };

    let db = Arc::new(Db::new(db_path)?);

    // Clean up old data in background
    let db_cleanup = Arc::clone(&db);
    let policy = config.retention.clone();
    tokio::spawn(async move {
        let _ = db_cleanup.cleanup_old_data(&policy);
    });

    // Start background fetcher with the full config
    let fetch_db = Arc::clone(&db);
    let fetch_config = config.clone();
    tokio::spawn(async move {
        fetcher::start_fetcher(fetch_db, fetch_config).await;
    });

    // TUI setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Channel for events
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let tick_rate = Duration::from_millis(200); // 200ms UI tick

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
            if crossterm::event::poll(Duration::from_millis(100)).unwrap_or(false)
                && let Ok(CEvent::Key(key)) = event::read()
                && tx.send(AppEvent::Input(key)).is_err()
            {
                break;
            }
        }
    });

    let mut app = App::new(db);
    // Initial fetch
    app.on_tick();

    // Main loop
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

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
