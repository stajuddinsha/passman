#!/bin/bash

# Keytui GUI Demo Script

echo "🎬 Keytui GUI Demo"
echo "=================="
echo ""

# Set environment variables
export PATH="/usr/bin:$PATH"
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig"

# Check if release binary exists
if [ ! -f "target/release/keytui-gui" ]; then
    echo "❌ Release binary not found. Building first..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "❌ Build failed!"
        exit 1
    fi
fi

echo "🚀 Starting Keytui GUI Demo..."
echo ""
echo "📋 Demo Features:"
echo "   • Overlay window (640×420px)"
echo "   • Search interface with live filtering"
echo "   • Sample password entries (Gmail, GitHub)"
echo "   • Keyboard navigation"
echo ""
echo "🎯 Try searching for:"
echo "   • 'gmail' - to find Gmail entry"
echo "   • 'github' - to find GitHub entry"
echo "   • 'dev' - to find development-related entries"
echo ""
echo "⌨️  Controls:"
echo "   • Type to search"
echo "   • Arrow keys to navigate"
echo "   • Enter to select (copies to clipboard)"
echo "   • Close window to exit"
echo ""
echo "🔄 Starting application in 3 seconds..."
sleep 3

# Start the application
./target/release/keytui-gui
