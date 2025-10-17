#!/bin/bash

echo "🔍 Verifying Passman TUI Data Persistence"
echo "=========================================="

# Check current vault.json
echo "📁 Current vault.json contents:"
if [ -f "vault.json" ]; then
    echo "✅ vault.json exists"
    echo "📊 Entry count: $(cat vault.json | jq 'length' 2>/dev/null || echo 'Could not parse')"
    echo ""
    echo "📋 Current entries:"
    cat vault.json | jq -r '.[] | "• \(.name) (\(.username // "no username"))"' 2>/dev/null || echo "Could not parse entries"
else
    echo "❌ vault.json does not exist"
fi

echo ""
echo "🧪 Manual Test Steps:"
echo "====================="
echo "1. Run: passman"
echo "2. You should see existing entries (Gmail, GitHub)"
echo "3. Press 'a' to add new entry"
echo "4. Type: MyTest|mypassword123"
echo "5. Press Enter to save"
echo "6. Press 'q' to quit"
echo "7. Run: passman again"
echo "8. You should see: Gmail, GitHub, MyTest"
echo ""
echo "✅ If MyTest appears after restart, persistence is working!"
echo ""
echo "🔧 Troubleshooting:"
echo "- If entries don't appear, check vault.json file"
echo "- If save fails, check file permissions"
echo "- If load fails, check JSON format"
