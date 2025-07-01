#!/bin/bash

# Uninstall script for rview
# Removes rview from ~/.local/bin/

set -e  # Exit on error

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="rview"
BINARY_PATH="$INSTALL_DIR/$BINARY_NAME"

echo "🗑️  Uninstalling rview..."

if [ -f "$BINARY_PATH" ]; then
    rm "$BINARY_PATH"
    echo "✅ Successfully removed $BINARY_PATH"
else
    echo "⚠️  $BINARY_PATH not found - may already be uninstalled"
fi

# Verify removal
if command -v "$BINARY_NAME" &> /dev/null; then
    echo "⚠️  Warning: $BINARY_NAME is still found in PATH"
    echo "   It may be installed elsewhere or cached by your shell"
    echo "   Try: hash -r  (to clear shell cache)"
else
    echo "✅ $BINARY_NAME successfully removed from PATH"
fi

echo "🧹 Uninstall complete!"