#!/bin/bash

# Keytui GUI Installation Script

set -e

echo "🚀 Installing Keytui GUI..."

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo "❌ Please don't run this script as root. It will use sudo when needed."
    exit 1
fi

# Check if release binary exists
if [ ! -f "target/release/keytui-gui" ]; then
    echo "❌ Release binary not found. Building first..."
    ./build.sh
    if [ $? -ne 0 ]; then
        echo "❌ Build failed!"
        exit 1
    fi
fi

# Install clipboard dependencies
echo "📦 Installing clipboard dependencies..."
sudo apt update
sudo apt install -y wl-clipboard xclip

# Install the binary
echo "📁 Installing binary to /usr/local/bin/..."
sudo cp target/release/keytui-gui /usr/local/bin/
sudo chmod +x /usr/local/bin/keytui-gui

# Create desktop entry
echo "🖥️  Creating desktop entry..."
mkdir -p ~/.local/share/applications
cat > ~/.local/share/applications/keytui-gui.desktop << EOF
[Desktop Entry]
Name=Keytui GUI
Comment=Lightweight desktop password manager
Exec=keytui-gui
Icon=preferences-desktop-user-password
Terminal=false
Type=Application
Categories=Utility;Security;
Keywords=password;manager;security;clipboard;
EOF

# Create autostart entry (optional)
echo "🔄 Creating autostart entry (optional)..."
mkdir -p ~/.config/autostart
cat > ~/.config/autostart/keytui-gui.desktop << EOF
[Desktop Entry]
Name=Keytui GUI Daemon
Comment=Keytui GUI background daemon
Exec=keytui-gui --daemon
Icon=preferences-desktop-user-password
Terminal=false
Type=Application
Hidden=false
X-GNOME-Autostart-enabled=true
EOF

echo "✅ Installation completed successfully!"
echo ""
echo "🎯 Next steps:"
echo "   1. Set up global shortcut:"
echo "      - Open Ubuntu Settings → Keyboard → Custom Shortcut"
echo "      - Add new shortcut:"
echo "        • Name: Keytui GUI"
echo "        • Command: keytui-gui"
echo "        • Shortcut: Ctrl + Alt + P"
echo ""
echo "   2. Test the application:"
echo "      - Run: keytui-gui"
echo "      - Or use the global shortcut once configured"
echo ""
echo "   3. Optional: Enable autostart daemon"
echo "      - The daemon will start automatically on login"
echo "      - Disable by removing ~/.config/autostart/keytui-gui.desktop"
echo ""
echo "🎉 Keytui GUI is ready to use!"
