#!/usr/bin/env bash

set -e

echo "======================================"
echo "    Live News TUI Installer "
echo "======================================"

# Determine OS
OS="$(uname -s)"
echo "Detected OS: $OS"

# Check dependencies
dependencies=("curl" "git" "pkg-config")
if [[ "$OS" == "Linux" ]]; then
    dependencies+=("libssl-dev" "python3-pip")
fi

echo "Checking system dependencies..."
for dep in "${dependencies[@]}"; do
    if ! command -v "$dep" &> /dev/null && [[ "$OS" == "Linux" ]]; then
        echo "Missing dependency: $dep. Attempting to install..."
        sudo apt-get update && sudo apt-get install -y "$dep" || echo "Please install $dep manually."
    fi
done

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust/Cargo is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install Python requirements
echo "Installing Python dependencies (scrapling)..."
pip3 install scrapling --break-system-packages || pip3 install scrapling

echo "Compiling Live News TUI in release mode..."
cargo build --release

BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

echo "Installing binary to $BIN_DIR/live_news_tui"
cp target/release/live_news_tui "$BIN_DIR/"

# Add to PATH if not already there
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    SHELL_RC=""
    if [[ "$SHELL" == *"zsh"* ]]; then
        SHELL_RC="$HOME/.zshrc"
    else
        SHELL_RC="$HOME/.bashrc"
    fi

    echo "Adding $BIN_DIR to PATH in $SHELL_RC"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
    echo "Please restart your terminal or run: source $SHELL_RC"
fi

echo "======================================"
echo "Installation complete!"
echo "Run 'live_news_tui' to start."
echo "======================================"
