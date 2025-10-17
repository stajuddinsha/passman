#!/bin/bash

# Passman Uninstall Script
# Removes the terminal password manager from the system

set -e

echo "🗑️  Passman Uninstaller"
echo "======================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   print_error "This script should not be run as root"
   exit 1
fi

print_status "Uninstalling Passman..."

# Remove system-wide installation
if [ -f "/usr/local/bin/passman" ]; then
    print_status "Removing /usr/local/bin/passman..."
    sudo rm -f /usr/local/bin/passman
    print_success "Removed system-wide binary"
else
    print_warning "System-wide binary not found"
fi

# Remove user symlink
if [ -L ~/.local/bin/passman ]; then
    print_status "Removing ~/.local/bin/passman symlink..."
    rm -f ~/.local/bin/passman
    print_success "Removed user symlink"
else
    print_warning "User symlink not found"
fi

# Ask about vault data
echo ""
print_warning "Your password vault is stored in ~/.passman/vault.json"
read -p "Do you want to delete your vault data? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_status "Removing vault data..."
    rm -rf ~/.passman
    print_success "Vault data removed"
else
    print_status "Vault data preserved in ~/.passman/"
fi

# Check for any remaining files
print_status "Checking for remaining files..."

if [ -f "passman-wrapper.sh" ]; then
    print_status "Removing wrapper script..."
    rm -f passman-wrapper.sh
    print_success "Wrapper script removed"
fi

# Check if passman command still exists
if command -v passman &> /dev/null; then
    print_warning "Passman command still found in PATH"
    print_status "You may need to restart your terminal"
else
    print_success "Passman command removed from PATH"
fi

echo ""
print_success "🎉 Uninstallation completed!"
echo ""
print_status "Summary:"
echo "  ✅ Removed system-wide binary"
echo "  ✅ Removed user symlink"
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "  ✅ Removed vault data"
else
    echo "  📁 Vault data preserved in ~/.passman/"
fi
echo ""
print_success "Passman has been uninstalled successfully! 👋"
