#!/bin/bash

# Install script for rview
# Builds the project and installs it to ~/.local/bin/

set -e  # Exit on error

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="rview"

echo "🔨 Building rview in release mode..."
cargo build --release

echo "📁 Creating install directory if it doesn't exist..."
mkdir -p "$INSTALL_DIR"

echo "📦 Installing $BINARY_NAME to $INSTALL_DIR..."
cp "target/release/$BINARY_NAME" "$INSTALL_DIR/"

# Make sure it's executable
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "✅ Successfully installed $BINARY_NAME to $INSTALL_DIR"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "⚠️  WARNING: $INSTALL_DIR is not in your PATH"
    echo "   Add this line to your shell config file (~/.bashrc, ~/.zshrc, etc.):"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "   Or run this command and restart your shell:"
    echo "   echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
else
    echo "✅ $INSTALL_DIR is already in your PATH"
fi

echo ""
echo "🚀 Installation complete!"
echo "   You can now run: $BINARY_NAME"
echo "   Try: $BINARY_NAME --help"

# Test the installation
if command -v "$BINARY_NAME" &> /dev/null; then
    echo ""
    echo "🧪 Testing installation..."
    "$BINARY_NAME" --version
else
    echo ""
    echo "⚠️  $BINARY_NAME not found in PATH. You may need to:"
    echo "   1. Add ~/.local/bin to your PATH (see warning above)"
    echo "   2. Restart your shell"
    echo "   3. Or run directly: $INSTALL_DIR/$BINARY_NAME"
fi