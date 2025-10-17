#!/bin/bash

echo "🧪 Testing Keytui GUI UI Layout..."
echo "=================================="
echo ""

# Check if binary exists
if [ ! -f "target/release/keytui-gui" ]; then
    echo "❌ Release binary not found. Building first..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "❌ Build failed!"
        exit 1
    fi
fi

echo "✅ Release binary found"
echo ""
echo "🎯 Expected UI Layout:"
echo "┌─────────────────────────────────────────┐"
echo "│ [Add] [Edit] [Delete]              Keytui │  ← Header Bar"
echo "├─────────────────────────────────────────┤"
echo "│ Search passwords...                     │  ← Search Bar"
echo "├─────────────────────────────────────────┤"
echo "│ Gmail                    user@gmail.com │  ← Results"
echo "│ GitHub                   developer      │"
echo "├─────────────────────────────────────────┤"
echo "│ Status: Ready                           │  ← Status"
echo "└─────────────────────────────────────────┘"
echo ""
echo "🔍 What to Look For:"
echo "1. Header bar with Add/Edit/Delete buttons at the top"
echo "2. Search bar below the header"
echo "3. Results list with sample entries"
echo "4. Status bar at the bottom"
echo ""
echo "🚀 Starting application..."
echo "   Look for the header bar with buttons at the top!"
echo ""

# Run the application
./target/release/keytui-gui
