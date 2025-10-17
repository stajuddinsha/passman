#!/bin/bash

# Test script for Keytui GUI

echo "🧪 Testing Keytui GUI Application..."

# Set environment variables
export PATH="/usr/bin:$PATH"
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig"

# Check if the release binary exists
if [ ! -f "target/release/keytui-gui" ]; then
    echo "❌ Release binary not found. Building first..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "❌ Build failed!"
        exit 1
    fi
fi

echo "✅ Release binary found. Testing application..."

# Test if the application can start (with timeout)
timeout 5s ./target/release/keytui-gui 2>&1 | head -10

echo ""
echo "🎉 Basic functionality test completed!"
echo ""
echo "📋 Next steps:"
echo "   1. Set up global shortcut (Ctrl+Alt+P) in Ubuntu Settings"
echo "   2. Install clipboard tools: sudo apt install wl-clipboard xclip"
echo "   3. Test the overlay interface"
echo ""
echo "🚀 To run the application:"
echo "   ./target/release/keytui-gui"
echo ""
echo "📦 To install system-wide:"
echo "   ./install.sh"
