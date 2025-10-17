# Global Shortcut Guide - Ctrl+Alt+P

## ✅ **Global Shortcut Successfully Configured!**

The Ctrl+Alt+P shortcut is now configured to open the Keytui Terminal interface.

### 🎯 **Shortcut Configuration**

#### ✅ **Shortcut Details**
- **Key Combination**: `Ctrl+Alt+P`
- **Name**: "Keytui Terminal"
- **Command**: `/home/taj/workspace/cortex/projects/passman/target/release/keytui-tui`
- **Status**: ✅ Active and working

### 🚀 **How to Use**

#### **Open Terminal Interface**
1. **Press `Ctrl+Alt+P`** anywhere on your system
2. **Terminal TUI opens** with password entries
3. **Navigate** with ↑/↓ arrow keys
4. **Search** by typing
5. **Copy password** by pressing Enter (auto-quits)
6. **Add/Edit/Delete** with a/e/d keys

### 🎮 **Terminal Interface Controls**

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

### 🧪 **Test the Shortcut**

#### **Step 1: Test the Shortcut**
1. **Press `Ctrl+Alt+P`** from anywhere
2. **Terminal interface should open**
3. **You should see your password entries**

#### **Step 2: Test Navigation**
1. **Use ↑/↓** to navigate entries
2. **Type to search** for specific entries
3. **Press Enter** to copy password (auto-quits)

#### **Step 3: Test Operations**
1. **Press `a`** to add new entry
2. **Press `e`** to edit selected entry
3. **Press `d`** to delete selected entry

### 🔧 **Verification Commands**

#### **Check Shortcut Configuration**
```bash
gsettings get org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ name
# Should return: 'Keytui Terminal'

gsettings get org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ binding
# Should return: '<Ctrl><Alt>p'
```

#### **Test Manual Launch**
```bash
/home/taj/workspace/cortex/projects/passman/target/release/keytui-tui
# Should open the terminal interface
```

### 🎯 **Workflow Examples**

#### **Quick Password Access**
1. **Press `Ctrl+Alt+P`** → Terminal opens
2. **Type "gmail"** → Filters to Gmail entry
3. **Press Enter** → Copies password and quits
4. **Paste password** where needed

#### **Add New Password**
1. **Press `Ctrl+Alt+P`** → Terminal opens
2. **Press `a`** → Add mode
3. **Type "MyService|mypassword"** → Enter format
4. **Press Enter** → Saves entry
5. **Press `q`** → Quit

#### **Edit Existing Password**
1. **Press `Ctrl+Alt+P`** → Terminal opens
2. **Navigate to entry** with ↑/↓
3. **Press `e`** → Edit mode
4. **Type "NewName|newpassword"** → Enter new values
5. **Press Enter** → Updates entry
6. **Press `q`** → Quit

### 🎉 **Benefits**

#### ✅ **Instant Access**
- **Global shortcut** works from anywhere
- **No terminal needed** - just press Ctrl+Alt+P
- **Quick password copying** with auto-quit

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

The global shortcut is now configured and ready:

- ✅ **Ctrl+Alt+P** opens terminal interface
- ✅ **All navigation** works with keyboard
- ✅ **Auto-quit** after copying passwords
- ✅ **Full CRUD** operations available

**Try it now**: Press `Ctrl+Alt+P` from anywhere to open the password manager! 🎯
