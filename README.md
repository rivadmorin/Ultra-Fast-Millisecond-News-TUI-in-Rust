# Live News TUI

A high-performance, real-time terminal news aggregator built with Rust.

## Features

- **Real-Time Data**: Fast, efficient background fetching from multiple RSS/Atom sources.
- **Dynamic Scheduling**:
  - **Active Mode**: High-frequency updates during peak hours (6 AM - 10 PM).
  - **Idle Mode**: Low-power updates during off-peak hours to save resources.
- **Resource Efficient**:
  - **Worker Pool**: Limits concurrent network requests to prevent system saturation.
  - **Conditional Fetching**: Uses ETags and Last-Modified headers to avoid redundant downloads.
- **Stability**: Automated daily maintenance (3 AM) including database cleanup and SQLite VACUUM.
- **Production Ready**: Robust logging and error handling.
- **Fully Free**: No API keys required, uses public feeds.

## System Architecture

- **Frontend**: Interactive TUI built with `ratatui` and `crossterm`.
- **Backend**: Async background fetcher using `tokio` and `reqwest`.
- **Storage**: Local persistent storage using `rusqlite` for fast querying and caching.
- **Management**: Unified DevOps script for lifecycle management.

## Installation & Usage

### One-Command Installation
```bash
./manage.sh install
```
*This will install system dependencies, Rust (if missing), and build the binary.*

### One-Command Update
```bash
./manage.sh update
```

### One-Command Uninstallation
```bash
./manage.sh uninstall
```

### Running the App
Once installed and in your PATH:
```bash
live_news_tui
```

## Controls

- `q`: Quit / Back
- `j` / `k` or `Up` / `Down`: Navigate news items
- `h` / `l` or `Left` / `Right`: Switch news categories
- `Enter`: Read article summary/details
- `Esc`: Close reading view

## Logs & Data
- **Logs**: `~/.local/share/LiveNewsTUI/live_news.log`
- **Database**: `~/.local/share/LiveNewsTUI/news.db`

---
*Developed with focus on speed, efficiency, and stability.*
