#!/usr/bin/env bash

# Live News TUI Manager script
# Supports: install, uninstall, update

set -e

APP_NAME="live_news_tui"
INSTALL_DIR="$HOME/.local/bin"

show_help() {
    echo "Usage: ./install.sh [install|uninstall|update]"
}

install_app() {
    echo "Checking dependencies..."
    OS="$(uname -s)"

    # Minimal dependency check
    if ! command -v cargo &> /dev/null; then
        echo "Rust is required. Please install it from https://rustup.rs"
        return 1
    fi

    echo "Building $APP_NAME..."
    cargo build --release

    echo "Installing to $INSTALL_DIR..."
    mkdir -p "$INSTALL_DIR"
    cp target/release/$APP_NAME "$INSTALL_DIR/"

    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo "Warning: $INSTALL_DIR is not in your PATH."
    fi

    echo "Done."
}

uninstall_app() {
    echo "Removing $APP_NAME..."
    rm -f "$INSTALL_DIR/$APP_NAME"
    echo "Done."
}

update_app() {
    echo "Updating..."
    if [ -d ".git" ]; then
        # Use a safe way to pull or just tell the user
        echo "Please run 'git pull' first, then './install.sh install'"
    else
        echo "Not a git repository."
    fi
}

case "$1" in
    install) install_app ;;
    uninstall) uninstall_app ;;
    update) update_app ;;
    help|--help|-h) show_help ;;
    *) install_app ;;
esac
