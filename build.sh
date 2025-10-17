#!/bin/bash

# Keytui GUI Build Script

set -e

echo "🔨 Building Keytui GUI..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

# Check if GTK development libraries are installed
if ! pkg-config --exists gtk4; then
    echo "❌ GTK4 development libraries not found."
    echo "   Install with: sudo apt install libgtk-4-dev"
    exit 1
fi

# Build the project
echo "📦 Building project..."
cargo build --release

echo "✅ Build completed successfully!"
echo ""
echo "🚀 To run the application:"
echo "   cargo run"
echo ""
echo "📋 To install system-wide:"
echo "   sudo cp target/release/keytui-gui /usr/local/bin/"
echo ""
echo "⚙️  Don't forget to set up the global shortcut:"
echo "   Ubuntu Settings → Keyboard → Custom Shortcut"
echo "   Command: keytui-gui"
echo "   Shortcut: Ctrl + Alt + P"
