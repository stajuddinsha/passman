# 🔐 Passman - Terminal Password Manager

A lightweight, secure, terminal-based password manager with an atuin-style interface for quick password access.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Unix-green.svg)]()

## ✨ Features

- 🎯 **Terminal Interface** - Clean TUI with atuin-style design
- 🔍 **Real-time Search** - Filter entries as you type
- ⚡ **Auto-quit** - Exits automatically after copying password
- 🔧 **Full CRUD** - Add, edit, delete password entries
- 🔒 **Secure Input** - Hidden password input
- 💾 **Data Persistence** - JSON-based vault storage
- 🚀 **Fast** - Instant startup and navigation
- 🎮 **Keyboard-driven** - Perfect for power users

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/passman.git
cd passman

# Build the application
cargo build --bin keytui-tui --release

# Install system-wide
sudo cp target/release/keytui-tui /usr/local/bin/
sudo cp passman-wrapper.sh /usr/local/bin/
sudo chmod +x /usr/local/bin/passman-wrapper.sh
sudo ln -sf /usr/local/bin/passman-wrapper.sh /usr/local/bin/passman
```

### Usage

```bash
# Launch the terminal interface
passman

# Quick workflow:
# 1. Type to search
# 2. Press Enter to copy password
# 3. App auto-quits
```

## 🎮 Interface

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

## 🎯 Controls

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate entries |
| `Enter` | Copy password & quit |
| `a` | Add new entry |
| `e` | Edit selected entry |
| `d` | Delete selected entry |
| `q` | Quit application |
| `Esc` | Clear search |

## 📋 Commands

### Terminal Interface (Recommended)
```bash
passman                    # Launch terminal interface
keytui-tui                 # Launch directly
```

### CLI Commands (Alternative)
```bash
passman list              # List all entries
passman add <name>        # Add new entry
passman delete <name>     # Delete entry
passman search <term>     # Search entries
passman help              # Show help
```

## 🛠️ Development

### Prerequisites
- Rust 1.75+
- Linux/Unix system
- Terminal with UTF-8 support

### Build
```bash
# Terminal interface
cargo build --bin keytui-tui --release

# CLI version
cargo build --bin passman --release

# GUI version (optional)
cargo build --bin keytui-gui --release
```

### Run
```bash
# Test terminal interface
./target/release/keytui-tui

# Test CLI commands
./target/release/passman help
```

## 📁 Data Storage

- **Vault File**: `./vault.json` (JSON format)
- **Backup**: Copy `vault.json` to backup passwords
- **Migration**: Easy to import/export data

## 🔒 Security

- **Local Storage** - All data stays on your machine
- **Hidden Input** - Passwords not visible while typing
- **No Cloud Sync** - Complete privacy
- **Secure Handling** - No passwords in command history

## 📚 Documentation

- [Setup Guide](SETUP_GUIDE.md) - Complete installation guide
- [Terminal Guide](PASSMAN_TERMINAL_GUIDE.md) - Terminal interface usage
- [CLI Guide](CLI_VERSION_GUIDE.md) - CLI commands reference
- [Configuration Guide](PASSMAN_CONFIGURATION_GUIDE.md) - Configuration details

## 🎯 Workflow Examples

### Quick Password Access
```bash
passman
# Type: gmail
# Press: Enter
# Password copied, app quits
```

### Add New Password
```bash
passman
# Press: a
# Type: MyService|mypassword
# Press: Enter
```

### Edit Password
```bash
passman
# Navigate to entry
# Press: e
# Type: NewName|newpassword
# Press: Enter
```

## 🚀 Why Passman?

- **Fast** - Instant startup and auto-quit
- **Simple** - Just type `passman` and go
- **Secure** - Local storage, hidden input
- **Efficient** - Keyboard-driven interface
- **Lightweight** - Minimal dependencies
- **Cross-platform** - Works on Linux/Unix

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [atuin](https://github.com/ellie/atuin) terminal interface design
- Built with [Rust](https://www.rust-lang.org/) and [ratatui](https://github.com/ratatui-org/ratatui)
- Terminal UI framework by [ratatui-org](https://github.com/ratatui-org)

---

**Passman** - A simple, secure, terminal-based password manager 🚀

*Made with ❤️ for terminal users*