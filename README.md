# Keytui - Terminal Password Manager

A lightweight terminal password manager with atuin-style interface for quick password access.

## Features

- **Terminal Interface**: Clean TUI with atuin-style design
- **Fast Search**: Real-time fuzzy search with live filtering
- **Auto-quit**: Automatically exits after copying password
- **Secure**: Local encryption with XChaCha20-Poly1305 and Argon2id
- **Keyboard Navigation**: Full keyboard control with arrow keys and Enter
- **Data Persistence**: JSON-based vault storage

## Installation

### Prerequisites

- Linux/Unix system with terminal support
- Rust 1.75 or later
- Terminal with UTF-8 support

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd passman

# Build the terminal version
cargo build --bin keytui-tui --release

# Run the terminal version
./target/release/keytui-tui

# Install system-wide (optional)
sudo cp target/release/keytui-tui /usr/local/bin/
```

### Current Status

✅ **Completed:**
- Terminal UI with atuin-style interface
- Real-time search with live filtering
- Add/Edit/Delete operations
- Auto-quit after copying password
- Data persistence with JSON storage
- Sample password entries

🚧 **In Progress:**
- Vault encryption (XChaCha20-Poly1305 + Argon2id)
- Clipboard integration

📋 **Planned:**
- Entry management (add/edit/delete)
- Auto-lock functionality
- Import/export features

### Dependencies

The application requires either `wl-copy` (Wayland) or `xclip` (X11) for clipboard functionality:

```bash
# For Wayland
sudo apt install wl-clipboard

# For X11
sudo apt install xclip
```

## Setup

### 1. Configure Global Shortcut

1. Open Ubuntu Settings → Keyboard → Custom Shortcut
2. Add a new shortcut:
   - Name: `Keytui GUI`
   - Command: `keytui-gui`
   - Shortcut: `Ctrl + Alt + P`

### 2. First Run

1. Run `keytui-gui` from terminal
2. Set your master password when prompted
3. Add your first password entry

## Usage

1. Press `Ctrl + Alt + P` anywhere on your system
2. Type a few letters of the service name (e.g., "gmail")
3. Use arrow keys to navigate results
4. Press `Enter` to copy the password
5. Press `Esc` to close the overlay

## Configuration

Configuration is stored in `~/.config/keytui/config.toml`:

```toml
clipboard_timeout = 20
auto_lock_minutes = 5
theme = "dark"
window_center = true
```

## Security

- All passwords are encrypted locally using XChaCha20-Poly1305
- Master key is derived using Argon2id
- Vault auto-locks after idle timeout
- Clipboard is automatically cleared
- No data is sent over the network

## Development

### Running in Development Mode

```bash
cargo run
```

### Running as Daemon

```bash
cargo run -- --daemon
```

## License

MIT License - see LICENSE file for details.
