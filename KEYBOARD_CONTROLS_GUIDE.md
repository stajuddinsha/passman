# Keyboard Controls Guide

## 🎮 **How to Use the Controls**

The terminal version uses **single letter keys** - no Ctrl combinations needed!

### 📝 **Basic Navigation**
- **↑/↓ Arrow Keys** - Navigate through entries
- **Enter** - Copy password and auto-quit
- **Esc** - Clear search / Cancel operation
- **q** - Quit application

### 🔧 **Entry Operations**
- **Press `a`** - Add new entry
- **Press `e`** - Edit selected entry
- **Press `d`** - Delete selected entry

### 🎯 **Step-by-Step Examples**

#### Adding a New Entry
1. Start app: `./target/release/keytui-tui`
2. **Press `a`** (just the letter 'a')
3. Enter format: `MyService|mypassword123`
4. Press **Enter** to save
5. Press **q** to quit

#### Editing an Entry
1. Start app: `./target/release/keytui-tui`
2. Use **↑/↓** to select an entry
3. **Press `e`** (just the letter 'e')
4. Enter new format: `NewName|newpassword`
5. Press **Enter** to save
6. Press **q** to quit

#### Deleting an Entry
1. Start app: `./target/release/keytui-tui`
2. Use **↑/↓** to select an entry
3. **Press `d`** (just the letter 'd')
4. Press **y** to confirm or **n** to cancel
5. Press **q** to quit

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

### 🚀 **Quick Reference**

| Key | Action |
|-----|--------|
| `a` | Add new entry |
| `e` | Edit selected entry |
| `d` | Delete selected entry |
| `↑/↓` | Navigate entries |
| `Enter` | Copy password & quit |
| `Esc` | Clear search |
| `q` | Quit application |

### 💡 **Tips**

1. **No Ctrl needed** - Just press the letter keys directly
2. **Case sensitive** - Use lowercase letters
3. **Format for add/edit**: `name|password`
4. **Auto-quit** - App exits automatically after copying password
5. **Manual quit** - Press `q` when done with management tasks

### 🎯 **Example Workflow**

```bash
# Start the app
./target/release/keytui-tui

# Add a new entry
# Press: a
# Type: Gmail|mypassword123
# Press: Enter

# Edit an entry
# Press: ↑ (navigate to entry)
# Press: e
# Type: NewGmail|newpassword
# Press: Enter

# Copy a password
# Press: ↑/↓ (navigate to entry)
# Press: Enter (copies and auto-quits)
```

The controls are simple and intuitive - just single letter keys! 🎯
