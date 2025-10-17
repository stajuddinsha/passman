# Add Button Test Guide

## Current Status
The application is currently hanging during startup, which prevents testing the Add button functionality. However, the Add button code is implemented and should work once the startup issue is resolved.

## What the Add Button Does
When you click the "➕ Add" button in the header bar, it will:

1. **Show a status message**: "Add button clicked! Creating test entry..."
2. **Create a test entry** with the following data:
   - Name: "Test Entry"
   - Username: "test@example.com"
   - Password: "test123"
   - URL: "https://test.com"
   - Tags: ["test"]
3. **Add the entry to the vault**
4. **Show success message**: "Test entry added successfully! Add button is working."

## How to Test
1. **Start the application**: `./target/release/keytui-gui`
2. **Look for the Add button**: It should be in the header bar at the top of the window
3. **Click the Add button**: You should see the status message change
4. **Check the status label**: At the bottom of the window, it should show the success message

## Expected Behavior
- The Add button should be visible in the header bar
- Clicking it should immediately show a status message
- A test entry should be added to the vault
- The status should update to show success

## Troubleshooting
If the application hangs during startup:
1. **Kill any running processes**: `pkill keytui-gui`
2. **Check for background processes**: `ps aux | grep keytui-gui`
3. **Try running with timeout**: `timeout 10s ./target/release/keytui-gui`

## Next Steps
Once the startup issue is resolved, we can implement a proper dialog for adding entries with form fields for:
- Name
- Username
- Password
- URL
- Tags

The current implementation creates a test entry to verify the basic functionality works.
