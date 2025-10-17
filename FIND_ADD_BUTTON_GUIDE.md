# 🔍 How to Find and Use the Add Button

## ✅ **Fixed: Add Button is Now Visible!**

The issue was that the window was set to `decorated(false)`, which hides the title bar and header bar. I've fixed this by setting `decorated(true)`.

## 🎯 **Where to Find the Add Button**

### **Location**: Top of the window in the header bar
- **Window Title**: "Keytui" 
- **Header Bar**: Contains three buttons: **Add**, **Edit**, **Delete**
- **Add Button**: Left side of the header bar with "Add" label

## 🖼️ **Visual Layout**

```
┌─────────────────────────────────────────┐
│ [Add] [Edit] [Delete]              Keytui │  ← Header Bar (now visible!)
├─────────────────────────────────────────┤
│ Search passwords...                     │
├─────────────────────────────────────────┤
│ Gmail                    user@gmail.com │
│ GitHub                   developer      │
│ New Entry               user@example.com│
├─────────────────────────────────────────┤
│ Status: Ready                           │
└─────────────────────────────────────────┘
```

## 🚀 **How to Use the Add Button**

### **Step 1: Launch the Application**
```bash
# Run the application
./target/release/keytui-gui

# Or use the global shortcut
# Press Ctrl+Alt+P
```

### **Step 2: Look for the Header Bar**
- The window now has a **title bar** with "Keytui" as the title
- Below the title bar, you'll see the **header bar** with three buttons
- The **Add button** is on the left side

### **Step 3: Click the Add Button**
1. **Click "Add"** in the header bar
2. **Dialog opens** with form fields:
   - Name: Service name (e.g., "Gmail")
   - Username: Username or email
   - Password: Your password
   - URL: Website URL (optional)
   - Tags: Comma-separated tags (optional)
3. **Fill in the details** and click "Add"
4. **Entry appears** in the search results

## 🧪 **Test the Add Functionality**

### **Quick Test**
1. **Run the app**: `./target/release/keytui-gui`
2. **Look for the header bar** at the top
3. **Click "Add"** button
4. **Fill in a test entry**:
   ```
   Name: Test Service
   Username: test@example.com
   Password: test123
   URL: https://test.com
   Tags: test, demo
   ```
5. **Click "Add"** to save
6. **Verify** the entry appears in the search results

## 🔧 **If You Still Don't See the Add Button**

### **Check These Things**
1. **Window is decorated**: The window should have a title bar
2. **Header bar is visible**: You should see buttons below the title
3. **Window size**: Should be 640x420 pixels
4. **No overlapping**: Make sure no other windows are covering it

### **Troubleshooting**
```bash
# Rebuild the application
cargo build --release

# Test the application
./test_app.sh

# Run the application
./target/release/keytui-gui
```

## 🎉 **Success!**

The **Add button** is now visible in the header bar! You can:

1. ✅ **See the Add button** in the header bar
2. ✅ **Click Add** to open the dialog
3. ✅ **Fill in entry details** in the form
4. ✅ **Save new entries** to your password vault
5. ✅ **See entries** in the search results

**Your Keytui GUI password manager is now fully functional with visible Add/Edit/Delete buttons!** 🔐✨
