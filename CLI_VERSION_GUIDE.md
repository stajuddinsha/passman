# CLI Version Guide

## ✅ **CLI Version Complete - No Terminal Corruption!**

I've created a CLI version that solves both issues:
1. **No terminal corruption** - Uses standard CLI instead of TUI
2. **Command-line interface** - Simple commands like `passman add <name>`

### 🎯 **CLI Commands**

#### Available Commands
```bash
passman add <name>      # Add a new password entry
passman delete <name>   # Delete a password entry  
passman list           # List all entries
passman search <term>  # Search for entries
passman help           # Show help
```

#### Examples
```bash
# Add a new entry (will prompt for password)
passman add gmail

# Delete an entry
passman delete gmail

# List all entries
passman list

# Search for entries
passman search gmail

# Show help
passman help
```

### 🚀 **How to Use**

#### Step 1: Build the CLI Version
```bash
cargo build --bin passman --release
```

#### Step 2: Test the Commands
```bash
# Show help
./target/release/passman help

# List current entries
./target/release/passman list

# Add a new entry
./target/release/passman add mygmail
# (Will prompt: "Enter password for 'mygmail': ")

# List entries to see the new one
./target/release/passman list

# Delete the entry
./target/release/passman delete mygmail
```

### 🎮 **Command Details**

#### Add Entry
```bash
passman add gmail
# Prompts: "Enter password for 'gmail': "
# Saves: gmail|yourpassword
# Shows: "✅ Entry 'gmail' added successfully!"
```

#### Delete Entry
```bash
passman delete gmail
# Shows: "✅ Entry 'gmail' deleted successfully!"
# Or: "Error: Entry 'gmail' not found"
```

#### List Entries
```bash
passman list
# Shows:
# Password Entries:
# =================
# • Gmail (user@gmail.com) - https://gmail.com [email, google]
# • GitHub (developer) - https://github.com [development, git]
```

#### Search Entries
```bash
passman search gmail
# Shows:
# Search results for 'gmail':
# ==========================
# • Gmail (user@gmail.com) - https://gmail.com [email, google]
```

### 🔧 **Features**

#### ✅ **No Terminal Corruption**
- Uses standard CLI input/output
- No TUI that corrupts terminal state
- Safe to use in any terminal

#### ✅ **Secure Password Input**
- Uses `rpassword` for hidden password input
- No password visible on screen
- Standard secure input handling

#### ✅ **Simple Commands**
- `passman add <name>` - Add entry
- `passman delete <name>` - Delete entry
- `passman list` - List all entries
- `passman search <term>` - Search entries

#### ✅ **Data Persistence**
- Same `vault.json` file as other versions
- Automatic saving after add/delete
- Shared data between CLI and other versions

### 🎯 **Installation**

#### System-wide Installation
```bash
# Build the CLI version
cargo build --bin passman --release

# Install system-wide
sudo cp target/release/passman /usr/local/bin/

# Now you can use from anywhere:
passman add gmail
passman list
```

#### Local Usage
```bash
# Build and use locally
cargo build --bin passman --release
./target/release/passman add gmail
```

### 🎉 **Benefits**

#### ✅ **No Terminal Issues**
- No "command not found" errors
- No terminal corruption
- Safe to use in any shell

#### ✅ **Simple Interface**
- No complex TUI navigation
- Direct commands
- Easy to script and automate

#### ✅ **Secure**
- Hidden password input
- No password visible in history
- Standard CLI security practices

### 📋 **Comparison**

| Feature | TUI Version | CLI Version |
|---------|-------------|-------------|
| Terminal Safety | ❌ Can corrupt | ✅ Safe |
| Commands | Interactive | Direct commands |
| Password Input | Visible | Hidden |
| Scripting | Difficult | Easy |
| Automation | Limited | Full support |

### 🎯 **Recommendation**

**Use the CLI version** for:
- **Safe terminal usage** - No corruption issues
- **Simple commands** - Direct and clear
- **Scripting** - Easy to automate
- **Security** - Hidden password input

The CLI version provides all the functionality without the terminal corruption issues! 🎯
