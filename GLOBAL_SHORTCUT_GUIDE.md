# ⌨️ Keytui GUI - Global Shortcut Setup Guide

## Setting up Ctrl+Alt+P Global Shortcut

Your Keytui GUI password manager can be launched instantly from anywhere using the **Ctrl+Alt+P** global shortcut. Here are multiple ways to set it up:

## 🚀 **Method 1: Automated Setup (Recommended)**

### Quick Setup
```bash
# Run the automated setup script
./setup_shortcut.sh
```

This script will:
- ✅ Build the application if needed
- ✅ Configure the global shortcut automatically
- ✅ Test the shortcut functionality
- ✅ Provide manual setup instructions as backup

## 🖱️ **Method 2: Manual Setup via Ubuntu Settings**

### Step-by-Step Instructions

1. **Open Ubuntu Settings**
   - Click the gear icon in the top-right corner
   - Or press `Super + I` (Windows key + I)

2. **Navigate to Keyboard Settings**
   - Go to **Keyboard** in the left sidebar
   - Click on **Custom Shortcut** (or **Keyboard Shortcuts**)

3. **Add New Shortcut**
   - Click the **"+"** button to add a new shortcut
   - Fill in the details:
     ```
     Name: Keytui GUI
     Command: /home/taj/workspace/cortex/projects/passman/target/release/keytui-gui
     Shortcut: Ctrl + Alt + P
     ```

4. **Test the Shortcut**
   - Press **Ctrl + Alt + P** anywhere on your system
   - The Keytui GUI overlay should appear

## 🔧 **Method 3: Command Line Setup**

### Using gsettings (GNOME)
```bash
# Set the shortcut name
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ name "Keytui GUI"

# Set the command
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ command "/home/taj/workspace/cortex/projects/passman/target/release/keytui-gui"

# Set the key binding
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ binding "<Ctrl><Alt>p"

# Enable the shortcut
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/']"
```

### Using xbindkeys (Alternative)
```bash
# Install xbindkeys
sudo apt install xbindkeys

# Create xbindkeys config
cat > ~/.xbindkeysrc << EOF
"target/release/keytui-gui"
  ctrl+alt + p
EOF

# Start xbindkeys
xbindkeys
```

## 🧪 **Testing the Shortcut**

### Test Steps
1. **Press Ctrl+Alt+P** from anywhere on your desktop
2. **Verify the application opens** with the overlay window
3. **Test the search functionality** by typing in the search box
4. **Test the entry management** by clicking Add/Edit/Delete buttons

### Troubleshooting
If the shortcut doesn't work:

1. **Check if the binary exists:**
   ```bash
   ls -la target/release/keytui-gui
   ```

2. **Verify the shortcut is set:**
   ```bash
   gsettings get org.gnome.settings-daemon.plugins.media-keys custom-keybindings
   ```

3. **Check for conflicting shortcuts:**
   - Open Ubuntu Settings → Keyboard → Custom Shortcut
   - Look for other shortcuts using Ctrl+Alt+P

4. **Restart the desktop environment:**
   ```bash
   # Log out and log back in
   # Or restart GNOME
   ```

## 🎯 **Shortcut Configuration Details**

### Current Setup
- **Name**: Keytui GUI
- **Command**: `target/release/keytui-gui`
- **Shortcut**: `Ctrl + Alt + P`
- **Type**: Global shortcut (works from anywhere)

### Alternative Shortcuts
If Ctrl+Alt+P conflicts with other applications, try:
- `Ctrl + Alt + K` (for Keytui)
- `Super + P` (Windows key + P)
- `Ctrl + Shift + P`
- `Alt + F12`

## 🚀 **Advanced Configuration**

### System-wide Installation
```bash
# Install the application system-wide
sudo cp target/release/keytui-gui /usr/local/bin/

# Update the shortcut command
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ command "keytui-gui"
```

### Autostart Daemon
```bash
# Create autostart entry for background daemon
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
```

## 🎉 **Success!**

Once configured, you can:

1. **Press Ctrl+Alt+P anywhere** to instantly open your password manager
2. **Search for passwords** with live filtering
3. **Add, edit, and delete entries** using the interface
4. **Copy passwords to clipboard** with Enter key

Your Keytui GUI is now fully integrated into your Ubuntu desktop! 🚀
