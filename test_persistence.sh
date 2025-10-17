#!/bin/bash

echo "Testing Passman TUI Data Persistence"
echo "===================================="

# Check if vault.json exists
if [ -f "vault.json" ]; then
    echo "✅ vault.json exists"
    echo "Current entries:"
    cat vault.json | jq 'length' 2>/dev/null || echo "Could not parse JSON"
else
    echo "❌ vault.json does not exist"
fi

echo ""
echo "Instructions for testing:"
echo "1. Run: passman"
echo "2. Press 'a' to add new entry"
echo "3. Type: TestEntry|testpassword123"
echo "4. Press Enter to save"
echo "5. Press 'q' to quit"
echo "6. Run: passman again"
echo "7. Check if TestEntry appears in the list"
echo ""
echo "If TestEntry appears after restart, persistence is working! ✅"
