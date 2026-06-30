#!/usr/bin/env bash

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${RED}======================================${NC}"
echo -e "${RED}    Live News TUI Uninstaller         ${NC}"
echo -e "${RED}======================================${NC}"

BIN_DIR="$HOME/.local/bin"
DATA_DIR="$HOME/.local/share/LiveNews"

# Remove Binary
if [ -f "$BIN_DIR/live_news_tui" ]; then
    echo -e "${YELLOW}Removing binary from $BIN_DIR...${NC}"
    rm "$BIN_DIR/live_news_tui"
    echo -e "${GREEN}Binary removed.${NC}"
else
    echo -e "Binary not found in $BIN_DIR."
fi

# Optional Data Cleanup
if [ -d "$DATA_DIR" ]; then
    read -p "Do you want to remove configuration and database files in $DATA_DIR? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Removing data directory $DATA_DIR...${NC}"
        rm -rf "$DATA_DIR"
        echo -e "${GREEN}Data removed.${NC}"
    else
        echo -e "Data directory kept."
    fi
fi

echo -e "${BLUE}======================================${NC}"
echo -e "${GREEN}    Uninstallation Complete!          ${NC}"
echo -e "${BLUE}======================================${NC}"
