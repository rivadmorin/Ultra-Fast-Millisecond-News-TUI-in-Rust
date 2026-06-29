# Live News TUI

A real-time news feed reader in your terminal, built with Rust.

## Features

- **Real-time Updates:** Background fetcher with configurable intervals.
- **Efficient UI:** Built with `ratatui` and `crossterm`, optimized for low resource usage.
- **Database Caching:** Uses SQLite (`rusqlite`) for offline access and efficient data management.
- **Categorization:** Organize news into categories like Tech, AI, World News, and more.
- **Low Power Mode:** Automatically slows down fetch frequency during idle hours.

## Installation

You can install Live News TUI using the provided script:

```bash
# Clone the repository
git clone https://github.com/LiveNewsTUI/LiveNews
cd LiveNews

# Run the installer
./install.sh install
```

Make sure you have Rust and Cargo installed. The installer will place the binary in `~/.local/bin/`. Ensure this directory is in your `PATH`.

## Usage

Start the application by running:
```bash
live_news_tui
```

### Controls

- `q`: Quit the application (or go back from reading view).
- `Enter`: Read the selected news item.
- `Esc`: Go back from reading view to the main feed.
- `↑ / ↓` or `j / k`: Navigate through news items.
- `← / →` or `h / l`: Switch between categories.

## Maintenance & DevOps

The `install.sh` script supports basic lifecycle management:

- **Update:** `./install.sh update` (pulls latest changes and rebuilds).
- **Uninstall:** `./install.sh uninstall` (removes the binary).

## Configuration

The application stores its database and logs in OS-specific data directories (e.g., `~/.local/share/LiveNewsTUI/` on Linux).

Default fetch intervals:
- **Active Hours (6 AM - 10 PM):** Every 60 seconds.
- **Idle Hours:** Every 5 minutes.

## License

MIT
