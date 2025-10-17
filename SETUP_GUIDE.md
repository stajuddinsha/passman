# Passman Setup Guide

## 🚀 **Complete Setup Guide for Passman Terminal Interface**

This guide will help you set up Passman, a lightweight terminal password manager with an atuin-style interface.

### 📋 **What is Passman?**

Passman is a terminal-based password manager that provides:
- **Terminal TUI interface** with atuin-style design
- **Real-time search** and filtering
- **Add/Edit/Delete** operations
- **Auto-quit** after copying passwords
- **Secure password input**
- **JSON-based data persistence**

### 🎯 **Features**

- ✅ **Terminal Interface** - Clean TUI with keyboard navigation
- ✅ **Real-time Search** - Filter entries as you type
- ✅ **Auto-quit** - Exits automatically after copying password
- ✅ **Full CRUD** - Add, edit, delete password entries
- ✅ **Secure Input** - Hidden password input
- ✅ **Data Persistence** - JSON-based vault storage
- ✅ **Cross-platform** - Works on Linux/Unix systems

### 🔧 **Prerequisites**

- **Linux/Unix system** with terminal support
- **Rust 1.75+** (for building)
- **Terminal with UTF-8 support**
- **Basic terminal knowledge**

### 📦 **Installation**

#### **Step 1: Clone the Repository**
```bash
git clone https://github.com/yourusername/passman.git
cd passman
```

#### **Step 2: Install Dependencies (Ubuntu/Debian)**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install system dependencies
sudo apt update
sudo apt install build-essential pkg-config
```

#### **Step 3: Build the Application**
```bash
# Build the terminal version
cargo build --bin keytui-tui --release

# Build the CLI version (optional)
cargo build --bin passman --release
```

#### **Step 4: Install System-wide**
```bash
# Install the terminal interface
sudo cp target/release/keytui-tui /usr/local/bin/

# Install the CLI version
sudo cp target/release/passman /usr/local/bin/

# Create wrapper script for passman command
sudo cp passman-wrapper.sh /usr/local/bin/passman-wrapper.sh
sudo chmod +x /usr/local/bin/passman-wrapper.sh
sudo ln -sf /usr/local/bin/passman-wrapper.sh /usr/local/bin/passman
```

### 🚀 **Usage**

#### **Terminal Interface (Recommended)**
```bash
# Launch the terminal interface
passman

# Or launch directly
keytui-tui
```

#### **CLI Commands (Alternative)**
```bash
# List all entries
passman list

# Add new entry
passman add gmail

# Delete entry
passman delete gmail

# Search entries
passman search gmail

# Show help
passman help
```

### 🎮 **Terminal Interface Controls**

#### **Navigation**
- **↑/↓ Arrow Keys** - Navigate through entries
- **Enter** - Copy password and auto-quit
- **Esc** - Clear search / Cancel operation
- **q** - Quit application

#### **Search Mode (Default)**
- **Type** - Search entries in real-time
- **a** - Add new entry
- **e** - Edit selected entry
- **d** - Delete selected entry

#### **Add/Edit Mode**
- **Format**: `name|password` (e.g., `Gmail|mypassword123`)
- **Enter** - Save entry
- **Esc** - Cancel operation

#### **Delete Mode**
- **y** - Confirm deletion
- **n** - Cancel deletion
- **Esc** - Cancel operation

### 🎨 **Interface Layout**

```
🔐 Keytui - Password Manager
┌─────────────────────────────────────┐
│ 🔍 Search: [your query]             │
├─────────────────────────────────────┤
│ ▶ Gmail (user@gmail.com)           │
│   GitHub (developer)                │
│   MyService                         │
├─────────────────────────────────────┤
│ ↑↓ Navigate | Enter: Copy | a: Add │
│ e: Edit | d: Delete | Esc: Clear   │
│ q: Quit                            │
└─────────────────────────────────────┘
```

### 🧪 **Quick Start**

#### **Step 1: Launch the Interface**
```bash
passman
```

#### **Step 2: Add Your First Entry**
1. **Press `a`** to add new entry
2. **Type**: `Gmail|mypassword123`
3. **Press Enter** to save

#### **Step 3: Search and Copy**
1. **Type "gmail"** to search
2. **Press Enter** to copy password
3. **Application auto-quits**

### 📁 **Data Storage**

#### **Vault File**
- **Location**: `./vault.json` (in current directory)
- **Format**: JSON with encrypted passwords
- **Backup**: Copy `vault.json` to backup your passwords

#### **Sample Vault Structure**
```json
{
  "entry_id_1": {
    "id": "entry_id_1",
    "name": "Gmail",
    "username": "user@gmail.com",
    "password": "encrypted_password",
    "url": "https://gmail.com",
    "tags": ["email", "google"],
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

### 🔒 **Security Features**

#### **Password Input**
- **Hidden input** - No characters visible while typing
- **Secure handling** - No password in command history
- **Auto-quit** - Prevents leaving interface open

#### **Data Protection**
- **Local storage** - All data stays on your machine
- **JSON format** - Easy to backup and migrate
- **No cloud sync** - Complete privacy

### 🛠️ **Development**

#### **Build All Versions**
```bash
# Terminal interface
cargo build --bin keytui-tui --release

# CLI version
cargo build --bin passman --release

# GUI version (optional)
cargo build --bin keytui-gui --release
```

#### **Run Tests**
```bash
# Test terminal interface
./target/release/keytui-tui

# Test CLI commands
./target/release/passman help
./target/release/passman list
```

### 📚 **Documentation**

#### **Available Guides**
- `PASSMAN_TERMINAL_GUIDE.md` - Terminal interface usage
- `PASSMAN_CONFIGURATION_GUIDE.md` - Configuration details
- `CLI_VERSION_GUIDE.md` - CLI commands reference
- `TERMINAL_VERSION_GUIDE.md` - Terminal interface features

#### **Troubleshooting**
- **Terminal corruption**: Use CLI version instead
- **Build errors**: Check Rust version and dependencies
- **Permission errors**: Use `sudo` for system-wide installation

### 🎯 **Workflow Examples**

#### **Daily Password Access**
```bash
# Quick access
passman
# Type: gmail
# Press: Enter
# Password copied, app quits
```

#### **Password Management**
```bash
# Add new password
passman
# Press: a
# Type: MyService|mypassword
# Press: Enter

# Edit existing password
passman
# Navigate to entry
# Press: e
# Type: NewName|newpassword
# Press: Enter
```

### 🎉 **Ready to Use!**

Passman is now installed and ready to use:

- ✅ **`passman` command** launches terminal interface
- ✅ **All navigation** works with keyboard
- ✅ **Auto-quit** after copying passwords
- ✅ **Full CRUD** operations available
- ✅ **Secure password** input
- ✅ **Data persistence** working

**Start using it**: Type `passman` to open the password manager! 🎯

### 📞 **Support**

- **Issues**: Report on GitHub Issues
- **Documentation**: Check the guides in the repository
- **Contributing**: Fork and submit pull requests

---

**Passman** - A simple, secure, terminal-based password manager 🚀
