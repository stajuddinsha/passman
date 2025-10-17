#!/bin/bash

# Passman Demo Script
# Demonstrates the key features of the password manager

set -e

echo "🎬 Passman Demo"
echo "==============="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_demo() {
    echo -e "${CYAN}[DEMO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

# Check if passman is installed
if ! command -v passman &> /dev/null; then
    echo -e "${RED}Error: Passman not found. Please install it first.${NC}"
    echo "Run: ./install.sh"
    exit 1
fi

print_demo "Welcome to the Passman demo!"
echo ""

# Demo 1: Show current vault
print_demo "1. Current vault contents:"
if [ -f ~/.passman/vault.json ]; then
    echo "   📁 Vault location: ~/.passman/vault.json"
    echo "   📊 Current entries:"
    if [ -s ~/.passman/vault.json ]; then
        cat ~/.passman/vault.json | jq -r '.[] | "   • \(.name): \(.password)"' 2>/dev/null || \
        cat ~/.passman/vault.json | grep -o '"name":"[^"]*"' | sed 's/"name":"/   • /' | sed 's/"//'
    else
        echo "   (empty vault)"
    fi
else
    echo "   📁 Vault location: ~/.passman/vault.json (not created yet)"
    echo "   📊 Current entries: (empty vault)"
fi
echo ""

# Demo 2: CLI functionality
print_demo "2. CLI Commands:"
echo "   💻 Add password:    passman add gmail"
echo "   📋 List passwords:  passman list"  
echo "   🗑️  Delete password: passman delete gmail"
echo ""

# Demo 3: TUI functionality
print_demo "3. TUI Interface:"
echo "   🖥️  Launch TUI:     passman"
echo "   ⌨️  Navigation:      ↑/↓ arrows"
echo "   🔍 Search:          Type to filter"
echo "   📋 Copy password:   Enter key"
echo "   ➕ Add entry:       'a' key"
echo "   ✏️  Edit entry:     'e' key"
echo "   🗑️  Delete entry:   'd' key"
echo "   🚪 Quit:           'q' key"
echo ""

# Demo 4: Clipboard test
print_demo "4. Clipboard functionality:"
if command -v xclip &> /dev/null; then
    echo "   📋 Testing xclip..."
    echo "test123" | xclip -selection clipboard
    if xclip -selection clipboard -o | grep -q "test123"; then
        print_success "   ✅ Clipboard working (xclip)"
    else
        echo "   ❌ Clipboard test failed"
    fi
elif command -v wl-copy &> /dev/null; then
    echo "   📋 Testing wl-copy..."
    echo "test123" | wl-copy
    if wl-paste | grep -q "test123"; then
        print_success "   ✅ Clipboard working (wl-copy)"
    else
        echo "   ❌ Clipboard test failed"
    fi
else
    echo "   ⚠️  No clipboard tool found (install xclip or wl-clipboard)"
fi
echo ""

# Demo 5: Interactive demo
print_demo "5. Interactive Demo:"
echo "   🎮 Would you like to see the TUI in action?"
read -p "   Launch passman TUI? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "   Launching passman TUI..."
    echo "   (Press 'q' to quit the TUI when done exploring)"
    sleep 2
    passman
else
    print_info "   Skipping interactive demo"
fi

echo ""
print_demo "6. Key Features Summary:"
echo "   🔐 Secure local storage in ~/.passman/"
echo "   ⚡ Lightning-fast search and navigation"
echo "   📋 One-click password copying"
echo "   ⌨️  Full keyboard control"
echo "   🎨 Beautiful atuin-style interface"
echo "   🚀 System-wide 'passman' command"
echo ""

print_success "🎉 Demo completed!"
echo ""
print_info "📚 For more information:"
echo "   • README.md      - Project overview"
echo "   • SETUP_GUIDE.md - Detailed setup instructions"
echo "   • ./install.sh   - Easy installation"
echo "   • ./uninstall.sh - Easy removal"
echo ""
print_success "Happy password managing! 🔐"