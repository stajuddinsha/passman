#!/bin/bash

# Keytui GUI Global Shortcut Setup Script

set -e

echo "🎯 Setting up global shortcut (Ctrl+Alt+P) for Keytui GUI..."
echo ""

# Check if the binary exists
if [ ! -f "target/release/keytui-gui" ]; then
    echo "❌ Release binary not found. Building first..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "❌ Build failed!"
        exit 1
    fi
fi

# Get the absolute path to the binary
BINARY_PATH=$(realpath target/release/keytui-gui)
echo "📁 Binary path: $BINARY_PATH"

# Check if gsettings is available
if ! command -v gsettings &> /dev/null; then
    echo "❌ gsettings not found. Please install gnome-settings-daemon"
    exit 1
fi

echo ""
echo "🔧 Setting up global shortcut via gsettings..."

# Get the current custom shortcuts
CURRENT_SHORTCUTS=$(gsettings get org.gnome.settings-daemon.plugins.media-keys custom-keybindings)

# Check if Keytui shortcut already exists
if echo "$CURRENT_SHORTCUTS" | grep -q "keytui"; then
    echo "⚠️  Keytui shortcut already exists. Updating..."
    # Remove existing shortcut
    gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "$(echo "$CURRENT_SHORTCUTS" | sed 's/.*keytui.*//')"
fi

# Add the new shortcut
echo "➕ Adding Keytui GUI shortcut..."

# Create the shortcut entry
SHORTCUT_NAME="Keytui GUI"
SHORTCUT_COMMAND="$BINARY_PATH"
SHORTCUT_BINDING="<Ctrl><Alt>p"

# Add to custom shortcuts
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ name "$SHORTCUT_NAME"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ command "$SHORTCUT_COMMAND"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ binding "$SHORTCUT_BINDING"

# Enable the shortcut
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/']"

echo "✅ Global shortcut configured successfully!"
echo ""
echo "🎯 Shortcut Details:"
echo "   • Name: $SHORTCUT_NAME"
echo "   • Command: $SHORTCUT_COMMAND"
echo "   • Binding: $SHORTCUT_BINDING"
echo ""
echo "🧪 Testing the shortcut..."
echo "   Press Ctrl+Alt+P to test the shortcut"
echo "   (The application should open)"
echo ""
echo "📋 Manual Setup (Alternative):"
echo "   1. Open Ubuntu Settings"
echo "   2. Go to Keyboard → Custom Shortcut"
echo "   3. Add new shortcut:"
echo "      • Name: Keytui GUI"
echo "      • Command: $SHORTCUT_COMMAND"
echo "      • Shortcut: Ctrl + Alt + P"
echo ""
echo "🎉 Global shortcut setup complete!"
