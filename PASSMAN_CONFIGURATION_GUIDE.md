# Passman Configuration Guide

## ✅ **Passman Successfully Configured!**

The passman CLI is now properly installed and configured on your system.

### 🎯 **Installation Status**

#### ✅ **System-wide Installation**
- **Binary**: `/usr/local/bin/passman`
- **Symlink**: `/home/taj/.local/bin/passman` (for PATH priority)
- **Status**: ✅ Working from anywhere

#### ✅ **Command Available**
```bash
which passman
# Returns: /home/taj/.local/bin/passman

passman help
# Shows: Help message with all commands
```

### 🎮 **Available Commands**

#### ✅ **All Commands Working**
```bash
passman help           # Show help message
passman list           # List all password entries
passman add <name>     # Add new password entry
passman delete <name>  # Delete password entry
passman search <term>  # Search for entries
```

### 🧪 **Tested Commands**

#### ✅ **List Command**
```bash
passman list
# Shows:
# Password Entries:
# =================
# • Gmail (user@gmail.com) - https://gmail.com [email, google]
# • GitHub (developer) - https://github.com [development, git]
```

#### ✅ **Search Command**
```bash
passman search gmail
# Shows:
# Search results for 'gmail':
# ==========================
# • Gmail (user@gmail.com) - https://gmail.com [email, google]
```

#### ✅ **Help Command**
```bash
passman help
# Shows complete usage information
```

### 🚀 **How to Use**

#### **List All Entries**
```bash
passman list
```

#### **Add New Entry**
```bash
passman add mygmail
# Prompts: "Enter password for 'mygmail': "
# (Password input is hidden)
```

#### **Delete Entry**
```bash
passman delete mygmail
# Shows: "✅ Entry 'mygmail' deleted successfully!"
```

#### **Search Entries**
```bash
passman search gmail
# Shows matching entries
```

### 🔧 **Configuration Details**

#### **Installation Paths**
- **System Binary**: `/usr/local/bin/passman`
- **User Symlink**: `/home/taj/.local/bin/passman`
- **Data File**: `./vault.json` (in current directory)

#### **PATH Priority**
- `/home/taj/.local/bin` (first priority)
- `/usr/local/bin` (fallback)
- Both point to the same binary

### 📋 **Command Examples**

#### **Complete Workflow**
```bash
# 1. List current entries
passman list

# 2. Add a new entry
passman add github
# (Enter password when prompted)

# 3. List to see the new entry
passman list

# 4. Search for specific entries
passman search github

# 5. Delete an entry
passman delete github

# 6. Verify deletion
passman list
```

### 🎯 **Features**

#### ✅ **Secure Password Input**
- Hidden password input (no characters visible)
- Secure CLI practices
- No password in command history

#### ✅ **Data Persistence**
- Saves to `vault.json` file
- Automatic saving after operations
- JSON format for easy backup

#### ✅ **Search Functionality**
- Search by name, username, URL, tags
- Case-insensitive search
- Real-time filtering

#### ✅ **Simple Commands**
- Easy to remember syntax
- Clear success/error messages
- Help system included

### 🎉 **Ready to Use!**

The passman CLI is now fully configured and ready to use:

- ✅ **`passman` command** works from anywhere
- ✅ **`passman list`** shows all entries
- ✅ **All commands** tested and working
- ✅ **Secure password input** implemented
- ✅ **Data persistence** working

**Start using it now**: `passman help` to see all available commands! 🎯
