#!/usr/bin/env bash
set -e
echo "Updating Live News TUI..."
if [ -d .git ]; then
    echo "Updating via git..."
    # git pull --rebase
else
    echo "Error: Not a git repository."
fi
./install.sh
echo "Update complete!"
