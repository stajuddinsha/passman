# Keytui GUI - Instant Password Search for Ubuntu

A lightweight desktop password manager built for Ubuntu with a Spotlight-style overlay interface.

## Features

- **Global Shortcut**: Press `Ctrl + Alt + P` anywhere to open the search overlay
- **Fast Search**: Fuzzy search with live filtering
- **Secure**: Local encryption with XChaCha20-Poly1305 and Argon2id
- **Auto-clear**: Clipboard automatically clears after 20 seconds
- **Keyboard Navigation**: Full keyboard control with arrow keys and Enter
- **Auto-lock**: Vault locks after idle timeout or system suspend

## Installation

### Prerequisites

- Ubuntu 20.04 or later
- GTK 4.6 or later (tested with 4.6.9)
- Rust 1.75 or later

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd passman

# Install dependencies
sudo apt install libgtk-4-dev pkg-config wl-clipboard xclip

# Build the project
cargo build --release

# Install system-wide (optional)
sudo cp target/release/keytui-gui /usr/local/bin/
```

### Current Status

✅ **Completed:**
- Basic GTK4 GUI framework
- Search interface with live filtering
- Sample password entries
- Project structure and configuration

🚧 **In Progress:**
- Vault encryption (XChaCha20-Poly1305 + Argon2id)
- Clipboard integration
- Global shortcut setup

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
