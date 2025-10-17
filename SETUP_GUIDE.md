# Passman - Terminal Password Manager Setup Guide

A lightweight, secure terminal-based password manager with an atuin-style interface for Ubuntu/Linux systems.

## Features

- 🔐 **Secure Storage**: Passwords stored in `~/.passman/vault.json`
- 🎯 **Instant Search**: Type to filter passwords instantly
- 📋 **Clipboard Integration**: Auto-copy passwords to clipboard
- ⌨️ **Keyboard Shortcuts**: Full keyboard navigation
- 🚀 **Fast Access**: Launch with `passman` command
- 🔒 **Privacy**: No cloud dependencies, local storage only

## Prerequisites

- Ubuntu 20.04+ or compatible Linux distribution
- Rust compiler (cargo)
- Git

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/passman.git
cd passman
```

### 2. Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install system dependencies
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev

# Install clipboard tools (choose one)
sudo apt install -y xclip        # For X11 systems
# OR
sudo apt install -y wl-clipboard # For Wayland systems
```

### 3. Build the Application

```bash
# Build the TUI version
cargo build --bin keytui-tui --release

# Build the CLI version (optional)
cargo build --bin passman --release
```

### 4. Install System-wide

```bash
# Create wrapper script
cat > passman-wrapper.sh << 'EOF'
#!/bin/bash
# Passman wrapper script - launches the TUI interface
exec "/home/$(whoami)/workspace/cortex/projects/passman/target/release/keytui-tui"
EOF

chmod +x passman-wrapper.sh

# Install system-wide
sudo cp passman-wrapper.sh /usr/local/bin/passman
sudo chmod +x /usr/local/bin/passman

# Create symlink in user bin (for PATH priority)
mkdir -p ~/.local/bin
ln -sf /usr/local/bin/passman ~/.local/bin/passman
```

### 5. Verify Installation

```bash
# Test the installation
passman --help  # Should show help or launch TUI
which passman   # Should show /home/username/.local/bin/passman
```

## Usage

### Launch the TUI Interface

```bash
passman
```

### TUI Controls

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate entries |
| `Enter` | Copy password to clipboard |
| `a` | Add new entry |
| `e` | Edit selected entry |
| `d` | Delete selected entry |
| `q` | Quit |
| `Esc` | Clear search |

### Adding Passwords

1. Launch `passman`
2. Press `a` to add mode
3. Type: `name|password` (e.g., `gmail|mypassword123`)
4. Press `Enter` to save

### Searching Passwords

1. Launch `passman`
2. Start typing to filter entries
3. Use `↑/↓` to navigate results
4. Press `Enter` to copy password

### CLI Commands (Alternative)

```bash
# Add password
passman add gmail
# (will prompt for password)

# List all passwords
passman list

# Delete password
passman delete gmail
```

## Data Storage

- **Location**: `~/.passman/vault.json`
- **Format**: JSON array of password entries
- **Security**: Plain text (encryption planned for future versions)

### Example vault.json:
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

## Troubleshooting

### Clipboard Not Working

```bash
# Test clipboard functionality
echo "test" | xclip -selection clipboard
xclip -selection clipboard -o  # Should show "test"

# For Wayland systems
echo "test" | wl-copy
wl-paste  # Should show "test"
```

### Terminal Corruption After Exit

If your terminal gets corrupted after using passman:

```bash
# Reset terminal
reset
# OR
stty sane
```

### Build Issues

```bash
# Clean and rebuild
cargo clean
cargo build --bin keytui-tui --release

# If Rust issues, update toolchain
rustup update
```

### Permission Issues

```bash
# Fix permissions
chmod +x target/release/keytui-tui
chmod +x passman-wrapper.sh
```

## Development

### Project Structure

```
passman/
├── src/
│   ├── tui_main.rs      # Terminal UI interface
│   ├── cli_main.rs      # Command-line interface
│   ├── vault.rs         # Data structures
│   ├── clipboard.rs     # Clipboard operations
│   └── search.rs        # Search functionality
├── Cargo.toml           # Dependencies
└── README.md           # Project documentation
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run specific binary
cargo run --bin keytui-tui
cargo run --bin passman
```

### Testing

```bash
# Test clipboard
./test_clipboard_direct.sh

# Test TUI
./test_clipboard.sh
```

## Security Notes

⚠️ **Current Version**: Passwords are stored in plain text JSON format.

🔒 **Future Plans**: 
- XChaCha20-Poly1305 encryption
- Argon2id key derivation
- Auto-lock functionality
- Secure clipboard clearing

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

- **Issues**: Report bugs on GitHub Issues
- **Features**: Request features on GitHub Discussions
- **Documentation**: Check the README.md for more details

---

**Quick Start Summary:**
1. `git clone` → `cargo build --release` → `sudo cp passman-wrapper.sh /usr/local/bin/passman`
2. Run `passman` to launch the TUI
3. Press `a` to add passwords, type to search, `Enter` to copy

Enjoy your secure, fast password management! 🚀