#!/bin/bash

echo "🧪 Direct Clipboard Test"
echo "=========================="

# Clear clipboard first
echo "" | xclip -selection clipboard

# Test copying to clipboard
echo "test123" | xclip -selection clipboard

# Check if it worked
CLIPBOARD_CONTENT=$(xclip -selection clipboard -o)

if [ "$CLIPBOARD_CONTENT" = "test123" ]; then
    echo "✅ Clipboard functionality is working!"
    echo "📋 Content: $CLIPBOARD_CONTENT"
else
    echo "❌ Clipboard functionality is not working"
    echo "📋 Expected: test123"
    echo "📋 Got: $CLIPBOARD_CONTENT"
fi

echo ""
echo "🔧 Now test the TUI:"
echo "1. Run: ./target/release/keytui-tui"
echo "2. Press Enter on any entry"
echo "3. Check clipboard: xclip -selection clipboard -o"
