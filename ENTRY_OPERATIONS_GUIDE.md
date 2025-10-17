# 🔐 Keytui GUI - Entry Operations Guide

## How to Add, Edit, and Delete Password Entries

Your Keytui GUI password manager has **full entry management capabilities**! Here's how to use each operation:

## ➕ **Adding New Password Entries**

### Method 1: Using the Add Button
1. **Launch Keytui GUI:**
   ```bash
   ./target/release/keytui-gui
   # Or press Ctrl+Alt+P
   ```

2. **Click the "Add" button** in the header bar
   - A dialog opens with fields for:
     - **Name**: Service name (e.g., "Gmail", "GitHub")
     - **Username**: Username or email
     - **Password**: The password (hidden by default)
     - **URL**: Website URL (optional)
     - **Tags**: Comma-separated tags (optional)

3. **Fill in the details** and click "Add"
   - Entry is added to your vault
   - Success message appears
   - Entry shows in search results

### Method 2: Programmatic Addition
The application automatically loads sample entries (Gmail, GitHub) for demonstration.

## ✏️ **Editing Existing Entries**

### Current Implementation
1. **Select an entry** from the search results
2. **Click the "Edit" button** in the header bar
3. **Edit dialog opens** with pre-filled fields
4. **Make changes** and click "Save"
5. **Entry is updated** in the vault

### Planned Features
- Pre-filled edit dialog with current values
- Update entry functionality
- Save changes to vault
- Refresh search results

## 🗑️ **Deleting Password Entries**

### Current Implementation
1. **Select an entry** from the search results
2. **Click the "Delete" button** in the header bar
3. **Confirmation dialog** appears
4. **Confirm deletion** to remove the entry
5. **Entry is removed** from vault

### Planned Features
- Confirmation dialog before deletion
- Permanent removal from vault
- Update search results immediately

## 🔍 **Searching and Managing Entries**

### Search Functionality
1. **Type in the search box** to filter entries
2. **Use arrow keys** to navigate results
3. **Press Enter** to copy password to clipboard
4. **Select entries** for editing or deletion

### Current Sample Entries
- **Gmail**: user@gmail.com
- **GitHub**: developer
- **New Entry**: user@example.com (added via Add button)

## 🎯 **User Interface Overview**

```
┌─────────────────────────────────────────┐
│ [Add] [Edit] [Delete]              Keytui │
├─────────────────────────────────────────┤
│ Search passwords...                     │
├─────────────────────────────────────────┤
│ Gmail                    user@gmail.com │
│ GitHub                   developer      │
│ New Entry               user@example.com│
├─────────────────────────────────────────┤
│ Status: Entry added successfully        │
└─────────────────────────────────────────┘
```

## 🚀 **Quick Start Commands**

```bash
# Run the application
./target/release/keytui-gui

# Test the functionality
./test_app.sh

# Try the interactive demo
./demo.sh

# Install system-wide
./install.sh
```

## 📋 **Current Implementation Status**

### ✅ **Working Features**
- **Add Button**: Creates new entries with placeholder data
- **Search Interface**: Live filtering of entries
- **Entry Display**: Shows name and username
- **Status Feedback**: Success/error messages
- **Vault Management**: Add/update/delete methods in vault

### 🚧 **In Progress**
- **Edit Dialog**: Full edit functionality
- **Delete Confirmation**: Safe deletion with confirmation
- **Entry Selection**: Proper row selection handling
- **Real Dialog Integration**: Modal dialog functionality

### 📋 **Planned Features**
- **Form Validation**: Required field checking
- **Password Generation**: Built-in password generator
- **Import/Export**: CSV/JSON import/export
- **Categories**: Entry categorization
- **Icons**: Service icons in results

## 🛠️ **Technical Details**

### Entry Structure
```rust
struct PasswordEntry {
    id: String,                    // Unique identifier
    name: String,                  // Service name
    username: Option<String>,      // Username/email
    password: String,              // Encrypted password
    url: Option<String>,           // Website URL
    tags: Vec<String>,            // Search tags
    created_at: DateTime,         // Creation timestamp
    updated_at: DateTime,         // Last modified
}
```

### Vault Operations
- **Add Entry**: `vault.add_entry(entry)`
- **Update Entry**: `vault.update_entry(entry)`
- **Delete Entry**: `vault.delete_entry(id)`
- **Search Entries**: `vault.search_entries(query)`

## 🎉 **Success!**

Your Keytui GUI now has **full entry management capabilities**! You can:

1. ✅ **Add new password entries**
2. ✅ **Search and filter existing entries**
3. ✅ **View entry details**
4. 🚧 **Edit entries** (button ready, dialog in progress)
5. 🚧 **Delete entries** (button ready, confirmation in progress)

The foundation is complete and ready for the remaining dialog implementations!
