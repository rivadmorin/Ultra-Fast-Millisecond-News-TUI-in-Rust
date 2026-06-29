#!/usr/bin/env bash

set -e

APP_NAME="live_news_tui"
BIN_DIR="$HOME/.local/bin"
DATA_DIR="$HOME/.local/share/LiveNewsTUI"

show_help() {
    echo "Live News TUI Management Script"
    echo "Usage: ./manage.sh [command]"
    echo ""
    echo "Commands:"
    echo "  install   Install dependencies and build the application"
    echo "  update    Pull latest changes (if in git) and rebuild"
    echo "  uninstall Remove binary and application data"
    echo "  help      Show this help message"
}

check_dependencies() {
    echo "Checking system dependencies..."
    OS="$(uname -s)"
    deps=("curl" "git" "pkg-config")

    if [[ "$OS" == "Linux" ]]; then
        deps+=("libssl-dev" "build-essential")
        if command -v apt-get &> /dev/null; then
            sudo apt-get update && sudo apt-get install -y "${deps[@]}"
        fi
    fi

    if ! command -v cargo &> /dev/null; then
        echo "Rust not found. Installing via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
}

do_install() {
    check_dependencies
    echo "Building $APP_NAME in release mode..."
    cargo build --release
    mkdir -p "$BIN_DIR"
    cp target/release/$APP_NAME "$BIN_DIR/"

    if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
        echo "Warning: $BIN_DIR is not in your PATH."
        echo "Add 'export PATH=\"$BIN_DIR:\$PATH\"' to your .bashrc or .zshrc"
    fi
    echo "Installation successful! Run '$APP_NAME' to start."
}

do_update() {
    echo "Updating $APP_NAME..."
    if [[ -d ".git" ]]; then
        # Use a subshell to avoid blocking issues in some environments
        (git pull origin main || echo "Git pull failed, using local source.")
    fi
    cargo build --release
    cp target/release/$APP_NAME "$BIN_DIR/"
    echo "Update successful!"
}

do_uninstall() {
    echo "Uninstalling $APP_NAME..."
    rm -f "$BIN_DIR/$APP_NAME"
    echo "Removing application data..."
    rm -rf "$DATA_DIR"
    echo "Uninstallation complete."
}

case "$1" in
    install)
        do_install
        ;;
    update)
        do_update
        ;;
    uninstall)
        do_uninstall
        ;;
    help|*)
        show_help
        ;;
esac
