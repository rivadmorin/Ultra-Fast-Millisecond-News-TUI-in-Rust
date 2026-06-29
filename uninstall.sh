#!/usr/bin/env bash
set -e
echo "Uninstalling Live News TUI..."
BIN_DIR="$HOME/.local/bin"
rm -f "$BIN_DIR/live_news_tui"
echo "Binary removed."
echo "Note: Configuration and database files in ~/.local/share/LiveNews (or OS equivalent) were NOT removed."
