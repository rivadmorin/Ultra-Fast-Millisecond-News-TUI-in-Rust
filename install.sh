#!/usr/bin/env bash

echo "======================================"
echo "    Live News TUI Installer "
echo "======================================"

# Determine OS and Arch
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "Detected OS: $OS"
echo "Detected Arch: $ARCH"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust/Cargo is not installed."
    echo "Please install Rust first: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    return 1 2>/dev/null || true
fi

echo "Compiling the application for maximum performance (this may take a few minutes)..."
cargo build --release

BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

echo "Installing binary to $BIN_DIR/live_news_tui"
cp target/release/live_news_tui "$BIN_DIR/"

# Make sure it's in path
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo ""
    echo "WARNING: $BIN_DIR is not in your PATH."
    echo "Please add 'export PATH=\"\$HOME/.local/bin:\$PATH\"' to your ~/.bashrc or ~/.zshrc"
    echo ""
fi

echo "Installation complete!"
echo "Run 'live_news_tui' to start the application."
