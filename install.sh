#!/usr/bin/env bash

set -e

# Colors for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}======================================${NC}"
echo -e "${BLUE}    Live News TUI: Production Setup   ${NC}"
echo -e "${BLUE}======================================${NC}"

# Detect OS
OS="$(uname -s)"
echo -e "Detected OS: ${GREEN}$OS${NC}"

# Dependency Check Function
check_dep() {
    if ! command -v "$1" &> /dev/null; then
        return 1
    else
        return 0
    fi
}

# Install system dependencies
if [[ "$OS" == "Linux" ]]; then
    if command -v apt-get &> /dev/null; then
        echo -e "${YELLOW}Checking system dependencies (APT)...${NC}"
        deps=("curl" "git" "pkg-config" "libssl-dev" "python3-pip" "python3-venv" "build-essential")
        to_install=()
        for dep in "${deps[@]}"; do
            if ! dpkg -s "$dep" &> /dev/null && ! check_dep "$dep"; then
                to_install+=("$dep")
            fi
        done

        if [ ${#to_install[@]} -ne 0 ]; then
            echo -e "Installing missing dependencies: ${to_install[*]}"
            sudo apt-get update && sudo apt-get install -y "${to_install[@]}"
        fi
    elif command -v dnf &> /dev/null; then
        echo -e "${YELLOW}Checking system dependencies (DNF)...${NC}"
        sudo dnf install -y curl git pkgconf-pkg-config openssl-devel python3-pip gcc gcc-c++
    fi
fi

# Ensure Rust is installed
if ! check_dep cargo; then
    echo -e "${YELLOW}Rust/Cargo not found. Installing...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo -e "${GREEN}Rust is already installed.${NC}"
fi

# Python environment setup
echo -e "${YELLOW}Setting up Python environment...${NC}"
# We'll use a shared directory for the venv to avoid cluttering the source tree
VENV_DIR="$HOME/.local/share/LiveNews/venv"
mkdir -p "$(dirname "$VENV_DIR")"

if [ ! -d "$VENV_DIR" ]; then
    python3 -m venv "$VENV_DIR"
fi

source "$VENV_DIR/bin/activate"
pip install --upgrade pip
pip install scrapling duckduckgo-search

# Compile the application
echo -e "${YELLOW}Compiling Live News TUI in release mode...${NC}"
cargo build --release

# Installation directory
BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

echo -e "${YELLOW}Installing binary to $BIN_DIR/live_news_tui${NC}"
cp target/release/live_news_tui "$BIN_DIR/"

# Handle PATH
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo -e "${YELLOW}Adding $BIN_DIR to PATH...${NC}"
    SHELL_TYPE=$(basename "$SHELL")
    RC_FILE=""

    case "$SHELL_TYPE" in
        zsh) RC_FILE="$HOME/.zshrc" ;;
        bash) RC_FILE="$HOME/.bashrc" ;;
        *) RC_FILE="$HOME/.profile" ;;
    esac

    if [ -f "$RC_FILE" ]; then
        if ! grep -q "$BIN_DIR" "$RC_FILE"; then
            echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$RC_FILE"
            echo -e "${GREEN}Path added to $RC_FILE. Please restart your terminal or run 'source $RC_FILE'${NC}"
        fi
    fi
fi

echo -e "${BLUE}======================================${NC}"
echo -e "${GREEN}    Installation Successful!          ${NC}"
echo -e "    Run 'live_news_tui' to start.     "
echo -e "${BLUE}======================================${NC}"
