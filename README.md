# 🔐 Passman - Terminal Password Manager

A lightweight, secure terminal-based password manager with an atuin-style interface for Ubuntu/Linux systems.

![Passman Demo](https://img.shields.io/badge/Status-Ready%20for%20Use-brightgreen)
![Rust](https://img.shields.io/badge/Made%20with-Rust-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

## ✨ Features

- 🎯 **Instant Search** - Type to filter passwords instantly
- 📋 **Smart Clipboard** - Auto-copy passwords with one keystroke
- ⌨️ **Full Keyboard Navigation** - No mouse required
- 🚀 **Lightning Fast** - Launch and search in milliseconds
- 🔒 **Local Storage** - Your data stays on your machine
- 🎨 **Atuin-Style UI** - Beautiful terminal interface
- 🔧 **CLI & TUI** - Both command-line and interactive modes

## 🚀 Quick Start

### Installation

```bash
# Clone and build
git clone https://github.com/stajuddinsha/passman.git
cd passman
cargo build --bin keytui-tui --release

# Install system-wide
sudo cp passman-wrapper.sh /usr/local/bin/passman
sudo chmod +x /usr/local/bin/passman
mkdir -p ~/.local/bin
ln -sf /usr/local/bin/passman ~/.local/bin/passman
```

### Usage

```bash
# Launch the beautiful TUI
passman

# Or use CLI commands
passman add gmail
passman list
passman delete gmail
```

## 🎮 TUI Controls

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate entries |
| `Enter` | Copy password to clipboard |
| `a` | Add new entry |
| `e` | Edit selected entry |
| `d` | Delete selected entry |
| `q` | Quit |
| `Esc` | Clear search |

## 📖 Detailed Setup

For complete installation instructions, dependencies, and troubleshooting, see the [**Setup Guide**](SETUP_GUIDE.md).

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Terminal UI   │    │   CLI Interface │    │   Vault Engine  │
│   (ratatui)     │◄──►│   (clap)        │◄──►│   (JSON)        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Clipboard Ops  │    │  Search Engine  │    │  Data Storage   │
│  (xclip/wl-copy)│    │  (fuzzy)        │    │  (~/.passman/)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🔧 Development

### Project Structure

```
passman/
├── src/
│   ├── tui_main.rs      # 🖥️  Terminal UI interface
│   ├── cli_main.rs      # 💻  Command-line interface  
│   ├── vault.rs         # 🗄️  Data structures
│   ├── clipboard.rs     # 📋  Clipboard operations
│   └── search.rs        # 🔍 Search functionality
├── Cargo.toml           # 📦 Dependencies
├── SETUP_GUIDE.md       # 📖 Detailed setup instructions
└── README.md           # 📚 This file
```

### Building

```bash
# Development
cargo build

# Release
cargo build --release

# Run specific binary
cargo run --bin keytui-tui
cargo run --bin passman
```

## 🔒 Security

**Current Version**: Passwords stored in plain text JSON format.

**Future Roadmap**:
- 🔐 XChaCha20-Poly1305 encryption
- 🔑 Argon2id key derivation  
- ⏰ Auto-lock functionality
- 🧹 Secure clipboard clearing
- 🔐 Master password protection

## 📊 Data Format

Your passwords are stored in `~/.passman/vault.json`:

```json
[
  {
    "name": "gmail",
    "password": "mypassword123"
  },
  {
    "name": "github", 
    "password": "anotherpassword"
  }
]
```

## 🐛 Troubleshooting

### Clipboard Issues
```bash
# Test clipboard
echo "test" | xclip -selection clipboard
xclip -selection clipboard -o
```

### Terminal Corruption
```bash
# Reset terminal
reset
```

### Build Problems
```bash
# Clean rebuild
cargo clean
cargo build --bin keytui-tui --release
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) - Beautiful terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [atuin](https://github.com/atuinsh/atuin) - Inspiration for the UI style

## 📞 Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/stajuddinsha/passman/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/stajuddinsha/passman/discussions)
- 📖 **Documentation**: [Setup Guide](SETUP_GUIDE.md)

---

<div align="center">

**Made with ❤️ and Rust**

[⭐ Star this repo](https://github.com/stajuddinsha/passman) • [🐛 Report Bug](https://github.com/stajuddinsha/passman/issues) • [💡 Request Feature](https://github.com/stajuddinsha/passman/discussions)

</div>