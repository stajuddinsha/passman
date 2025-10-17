# Passman Terminal Interface Guide

## ✅ **Passman Updated to Launch Terminal Interface!**

The `passman` command now launches the keytui-tui terminal interface instead of the CLI.

### 🎯 **What Changed**

#### ✅ **Global Shortcut Removed**
- **Ctrl+Alt+P shortcut** has been removed
- **No more global shortcut** configuration

#### ✅ **Passman Command Updated**
- **`passman` command** now launches terminal interface
- **No more CLI commands** (add, delete, list, search)
- **Direct terminal interface** for password management

### 🚀 **How to Use**

#### **Open Terminal Interface**
```bash
passman
# Launches the keytui-tui terminal interface
```

#### **Terminal Interface Controls**
- **↑/↓ Arrow Keys** - Navigate through entries
- **Enter** - Copy password and auto-quit
- **Type** - Search entries in real-time
- **a** - Add new entry
- **e** - Edit selected entry
- **d** - Delete selected entry
- **q** - Quit application

### 🎮 **Terminal Interface Features**

#### **Navigation**
- **↑/↓ Arrow Keys** - Navigate through entries
- **Enter** - Copy password and auto-quit
- **Esc** - Clear search / Cancel operation

#### **Search Mode (Default)**
- **Type** - Search entries in real-time
- **a** - Add new entry
- **e** - Edit selected entry
- **d** - Delete selected entry
- **q** - Quit application

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

### 🧪 **Test the Updated Command**

#### **Step 1: Launch Interface**
```bash
passman
# Should open the terminal interface
```

#### **Step 2: Test Navigation**
1. **Use ↑/↓** to navigate entries
2. **Type to search** for specific entries
3. **Press Enter** to copy password (auto-quits)

#### **Step 3: Test Operations**
1. **Press `a`** to add new entry
2. **Press `e`** to edit selected entry
3. **Press `d`** to delete selected entry

### 🎯 **Workflow Examples**

#### **Quick Password Access**
1. **Type `passman`** → Terminal interface opens
2. **Type "gmail"** → Filters to Gmail entry
3. **Press Enter** → Copies password and quits
4. **Paste password** where needed

#### **Add New Password**
1. **Type `passman`** → Terminal interface opens
2. **Press `a`** → Add mode
3. **Type "MyService|mypassword"** → Enter format
4. **Press Enter** → Saves entry
5. **Press `q`** → Quit

#### **Edit Existing Password**
1. **Type `passman`** → Terminal interface opens
2. **Navigate to entry** with ↑/↓
3. **Press `e`** → Edit mode
4. **Type "NewName|newpassword"** → Enter new values
5. **Press Enter** → Updates entry
6. **Press `q`** → Quit

### 🔧 **Configuration Details**

#### **Command Structure**
- **Command**: `passman`
- **Wrapper Script**: `/home/taj/workspace/cortex/projects/passman/passman-wrapper.sh`
- **Target Binary**: `/home/taj/workspace/cortex/projects/passman/target/release/keytui-tui`
- **Symlink**: `/home/taj/.local/bin/passman`

#### **No More CLI Commands**
- ❌ `passman add <name>` - No longer available
- ❌ `passman delete <name>` - No longer available
- ❌ `passman list` - No longer available
- ❌ `passman search <term>` - No longer available
- ❌ `passman help` - No longer available

### 🎉 **Benefits**

#### ✅ **Simplified Interface**
- **Single command** - just `passman`
- **Interactive interface** - no command-line arguments
- **Visual navigation** - see all entries at once

#### ✅ **Full Functionality**
- **Search and filter** entries
- **Add/Edit/Delete** operations
- **Keyboard navigation** only
- **Auto-quit** after copying

#### ✅ **Secure**
- **Hidden password input** for add/edit
- **No password visible** in interface
- **Auto-quit** prevents leaving interface open

### 🎯 **Ready to Use!**

The passman command is now updated and ready:

- ✅ **`passman` command** launches terminal interface
- ✅ **No global shortcut** needed
- ✅ **All navigation** works with keyboard
- ✅ **Auto-quit** after copying passwords
- ✅ **Full CRUD** operations available

**Try it now**: Type `passman` to open the password manager interface! 🎯
