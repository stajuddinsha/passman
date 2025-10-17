#!/bin/bash

# Test script for Keytui GUI Global Shortcut

echo "🧪 Testing Keytui GUI Global Shortcut Test"
echo "=========================================="
echo ""

# Check if shortcut is configured
echo "🔍 Checking shortcut configuration..."

SHORTCUT_NAME=$(gsettings get org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ name 2>/dev/null | tr -d "'")
SHORTCUT_COMMAND=$(gsettings get org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ command 2>/dev/null | tr -d "'")
SHORTCUT_BINDING=$(gsettings get org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ binding 2>/dev/null | tr -d "'")

if [ "$SHORTCUT_NAME" = "Keytui GUI" ]; then
    echo "✅ Shortcut name: $SHORTCUT_NAME"
else
    echo "❌ Shortcut not configured properly"
    exit 1
fi

if [ "$SHORTCUT_BINDING" = "<Ctrl><Alt>p" ]; then
    echo "✅ Shortcut binding: $SHORTCUT_BINDING"
else
    echo "❌ Shortcut binding incorrect: $SHORTCUT_BINDING"
    exit 1
fi

if [ -f "$SHORTCUT_COMMAND" ]; then
    echo "✅ Command path exists: $SHORTCUT_COMMAND"
else
    echo "❌ Command path not found: $SHORTCUT_COMMAND"
    exit 1
fi

echo ""
echo "🎯 Shortcut Configuration Summary:"
echo "   • Name: $SHORTCUT_NAME"
echo "   • Binding: $SHORTCUT_BINDING"
echo "   • Command: $SHORTCUT_COMMAND"
echo ""
echo "🧪 Manual Test Instructions:"
echo "   1. Press Ctrl+Alt+P anywhere on your desktop"
echo "   2. The Keytui GUI overlay should appear"
echo "   3. Try searching for 'gmail' or 'github'"
echo "   4. Test the Add/Edit/Delete buttons"
echo ""
echo "🎉 Global shortcut is properly configured!"
echo ""
echo "💡 Troubleshooting:"
echo "   • If shortcut doesn't work, try logging out and back in"
echo "   • Check for conflicting shortcuts in Ubuntu Settings"
echo "   • Ensure the binary has execute permissions"
echo ""
echo "🚀 Your Keytui GUI is ready to use with Ctrl+Alt+P!"
