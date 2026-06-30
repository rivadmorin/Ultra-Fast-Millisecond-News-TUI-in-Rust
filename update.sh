#!/usr/bin/env bash

set -e

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}======================================${NC}"
echo -e "${BLUE}    Live News TUI Update              ${NC}"
echo -e "${BLUE}======================================${NC}"

if [ -d .git ]; then
    echo -e "${YELLOW}Checking for updates from repository...${NC}"
    git fetch
    LOCAL=$(git rev-parse @)
    REMOTE=$(git rev-parse @{u})

    if [ "$LOCAL" = "$REMOTE" ]; then
        echo -e "${GREEN}You are already on the latest version.${NC}"
        read -p "Do you want to force a re-install? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 0
        fi
    else
        echo -e "${YELLOW}New version found. Pulling changes...${NC}"
        git pull --rebase
    fi
else
    echo -e "${YELLOW}Not a git repository. Skipping git pull.${NC}"
fi

# Run the installer to rebuild and reinstall
echo -e "${YELLOW}Re-running installer...${NC}"
./install.sh

echo -e "${GREEN}Update complete!${NC}"
