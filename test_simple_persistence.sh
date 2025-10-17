#!/bin/bash

echo "🧪 Testing Simple Persistence"
echo "============================"

# Check current vault.json
echo "📁 Current vault.json:"
if [ -f "vault.json" ]; then
    echo "✅ vault.json exists"
    cat vault.json
else
    echo "❌ vault.json does not exist"
fi

echo ""
echo "🔧 Manual Test Steps:"
echo "===================="
echo "1. Run: ./target/release/keytui-tui"
echo "2. Press 'a' to add entry"
echo "3. Type: test|test123"
echo "4. Press Enter"
echo "5. Press 'q' to quit"
echo "6. Check vault.json file"
echo "7. Run TUI again to verify entry appears"
echo ""
echo "Expected vault.json format:"
echo "["
echo "  {"
echo "    \"name\": \"test\","
echo "    \"password\": \"test123\""
echo "  }"
echo "]"
