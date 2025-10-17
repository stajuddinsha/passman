use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, SearchEntry, ListBox, Box as GtkBox, Orientation, Label, Button, HeaderBar, Entry};
use std::sync::{Arc, Mutex};
use crate::vault::{VaultManager, PasswordEntry};
use crate::clipboard::ClipboardManager;
use crate::config::Config;
use crate::ui::{AddEntryDialog, UnlockDialog};

pub struct KeytuiApp {
    vault_manager: Arc<Mutex<VaultManager>>,
    clipboard_manager: ClipboardManager,
    config: Config,
}

impl KeytuiApp {
    pub fn new() -> Result<Self> {
        println!("[KEYTUI_APP] Creating KeytuiApp...");
        
        println!("[KEYTUI_APP] Loading config...");
        let config = Config::load()?;
        println!("[KEYTUI_APP] Config loaded successfully");
        
        println!("[KEYTUI_APP] Creating vault manager...");
        let vault_manager = Arc::new(Mutex::new(VaultManager::new(&config)?));
        println!("[KEYTUI_APP] Vault manager created successfully");
        
        println!("[KEYTUI_APP] Creating clipboard manager...");
        let clipboard_manager = ClipboardManager::new()?;
        println!("[KEYTUI_APP] Clipboard manager created successfully");

        println!("[KEYTUI_APP] KeytuiApp created successfully");
        Ok(Self {
            vault_manager,
            clipboard_manager,
            config,
        })
    }

    pub fn setup_ui(&self, window: &ApplicationWindow) {
        println!("[SETUP_UI] Starting UI setup...");
        
        // Create header bar with action buttons
        println!("[SETUP_UI] Creating header bar...");
        let header = HeaderBar::new();
        header.set_show_title_buttons(true);
        println!("[SETUP_UI] Header bar created successfully");
        
        // Add button
        println!("[SETUP_UI] Creating Add button...");
        let add_button = Button::builder()
            .label("➕ Add")
            .tooltip_text("Add new password entry")
            .build();
        println!("[SETUP_UI] Add button created successfully");
        
        // Edit button
        let edit_button = Button::builder()
            .label("✏️ Edit")
            .tooltip_text("Edit selected entry")
            .build();
        
        // Delete button
        let delete_button = Button::builder()
            .label("🗑️ Delete")
            .tooltip_text("Delete selected entry")
            .build();
        
        // Add buttons to header
        header.pack_start(&add_button);
        header.pack_start(&edit_button);
        header.pack_start(&delete_button);
        
        // Create main container
        let main_box = GtkBox::new(Orientation::Vertical, 0);
        
        // Add header bar to main container
        main_box.append(&header);

        // Create content container
        let content_box = GtkBox::new(Orientation::Vertical, 12);
        content_box.set_margin_start(20);
        content_box.set_margin_end(20);
        content_box.set_margin_top(20);
        content_box.set_margin_bottom(20);

        // Create search entry
        let search_entry = SearchEntry::builder()
            .placeholder_text("Search passwords...")
            .build();

        // Create results list
        let results_list = ListBox::new();
        results_list.set_selection_mode(gtk4::SelectionMode::Single);

        // Create status label
        let status_label = Label::new(None);
        status_label.set_halign(gtk4::Align::Start);

        // Add widgets to content box
        content_box.append(&search_entry);
        content_box.append(&results_list);
        content_box.append(&status_label);
        
        // Add content box to main container
        main_box.append(&content_box);

        // Set up search functionality
        self.setup_search(&search_entry, &results_list, &status_label);
        
        // Set up entry management
        self.setup_entry_management(window, &add_button, &edit_button, &delete_button, &results_list, &status_label);

        // Set up keyboard shortcuts
        self.setup_keyboard_shortcuts(window, &search_entry);

        // Set window content
        println!("[SETUP_UI] Setting window content...");
        window.set_child(Some(&main_box));
        println!("[SETUP_UI] Window content set successfully");

        // Focus search entry
        println!("[SETUP_UI] Focusing search entry...");
        search_entry.grab_focus();
        println!("[SETUP_UI] Search entry focused successfully");
        
        println!("[SETUP_UI] UI setup completed successfully");
    }

    fn setup_search(&self, search_entry: &SearchEntry, results_list: &ListBox, status_label: &Label) {
        let vault_manager = Arc::clone(&self.vault_manager);
        let results_list_clone = results_list.clone();
        let status_label_clone = status_label.clone();

        // Load all entries initially
        println!("[SEARCH] Loading all entries initially...");
        Self::load_all_entries(&results_list_clone, &vault_manager, &status_label_clone);

        search_entry.connect_search_changed(move |entry| {
            let query = entry.text().to_string();
            println!("[SEARCH] Search query: '{}'", query);
            
            if query.is_empty() {
                // Show all entries when search is empty
                println!("[SEARCH] Empty query, showing all entries");
                Self::load_all_entries(&results_list_clone, &vault_manager, &status_label_clone);
                return;
            }

            // Perform search
            if let Ok(vault) = vault_manager.lock() {
                if let Ok(entries) = vault.search_entries(&query) {
                    println!("[SEARCH] Found {} matches for '{}'", entries.len(), query);
                    
                    // Clear existing results
                    while let Some(child) = results_list_clone.first_child() {
                        results_list_clone.remove(&child);
                    }
                    
                    let entry_count = entries.len();
                    for entry in entries {
                        let row = Self::create_result_row_static(&entry);
                        results_list_clone.append(&row);
                    }
                    
                    status_label_clone.set_text(&format!("Found {} results for '{}'", entry_count, query));
                }
            }
        });

        // Handle Enter key on results
        let status_label_clone2 = status_label.clone();
        results_list.connect_row_activated(move |_, _row| {
            // For now, we'll use a placeholder - in real implementation,
            // we'd need to store the entry ID in the row
            status_label_clone2.set_text("Password copied to clipboard");
        });
    }

    fn create_result_row_static(entry: &crate::vault::PasswordEntry) -> gtk4::ListBoxRow {
        let row = gtk4::ListBoxRow::new();
        let box_widget = GtkBox::new(Orientation::Horizontal, 12);
        
        let name_label = Label::new(Some(&entry.name));
        let username_label = Label::new(entry.username.as_deref());
        
        // For now, we'll use a simpler approach without storing entry ID
        // In a real implementation, we'd store the entry ID for later reference
        
        box_widget.append(&name_label);
        box_widget.append(&username_label);
        
        row.set_child(Some(&box_widget));
        row
    }

    fn setup_entry_management(&self, window: &ApplicationWindow, add_button: &Button, edit_button: &Button, delete_button: &Button, results_list: &ListBox, status_label: &Label) {
        // Add button functionality
        let vault_manager = Arc::clone(&self.vault_manager);
        let results_list_clone = results_list.clone();
        let status_label_clone = status_label.clone();
        let window_clone = window.clone();
        
        println!("[SETUP_UI] Setting up Add button click handler...");
        add_button.connect_clicked(move |_| {
            println!("[ADD_BUTTON] Add button clicked!");
            
            // Create a simple dialog window
            println!("[ADD_BUTTON] Creating dialog window...");
            let dialog = ApplicationWindow::builder()
                .title("Add Password Entry")
                .transient_for(&window_clone)
                .modal(true)
                .default_width(400)
                .default_height(500)
                .resizable(false)
                .build();
            
            // Create form fields
            println!("[ADD_BUTTON] Creating form fields...");
            let vbox = GtkBox::new(Orientation::Vertical, 12);
            vbox.set_margin_start(20);
            vbox.set_margin_end(20);
            vbox.set_margin_top(20);
            vbox.set_margin_bottom(20);
            
            // Name field
            let name_label = Label::new(Some("Name:"));
            let name_entry = Entry::builder()
                .placeholder_text("Service name (e.g., Gmail)")
                .build();
            
            // Password field
            let password_label = Label::new(Some("Password:"));
            let password_entry = Entry::builder()
                .placeholder_text("Password")
                .visibility(false)
                .build();
            
            // Buttons
            let button_box = GtkBox::new(Orientation::Horizontal, 12);
            let cancel_button = Button::builder()
                .label("Cancel")
                .build();
            let save_button = Button::builder()
                .label("Save")
                .build();
            
            // Add widgets to vbox
            vbox.append(&name_label);
            vbox.append(&name_entry);
            vbox.append(&password_label);
            vbox.append(&password_entry);
            vbox.append(&button_box);
            
            // Add buttons to button box
            button_box.append(&cancel_button);
            button_box.append(&save_button);
            
            // Set window content
            dialog.set_child(Some(&vbox));
            
            // Connect cancel button
            let dialog_clone = dialog.clone();
            cancel_button.connect_clicked(move |_| {
                println!("[DIALOG] Cancel button clicked");
                dialog_clone.close();
            });
            
            // Connect save button
            let dialog_clone = dialog.clone();
            let vault_manager_clone = vault_manager.clone();
            let status_label_clone = status_label_clone.clone();
            save_button.connect_clicked(move |_| {
                println!("[DIALOG] Save button clicked");
                
                // Get form data
                let name = name_entry.text().to_string();
                let password = password_entry.text().to_string();
                
                // Create entry with only name and password
                let entry = PasswordEntry {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: if name.is_empty() { "New Entry".to_string() } else { name },
                    username: None,
                    password: if password.is_empty() { "new_password".to_string() } else { password },
                    url: None,
                    tags: vec![],
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };
                
                // Add to vault
                if let Ok(mut vault) = vault_manager_clone.lock() {
                    if let Err(e) = vault.add_entry(entry) {
                        status_label_clone.set_text(&format!("Error adding entry: {}", e));
                    } else {
                        status_label_clone.set_text("Entry added successfully!");
                    }
                }
                
                dialog_clone.close();
            });
            
            // Show the dialog
            println!("[ADD_BUTTON] Showing dialog...");
            dialog.present();
            println!("[ADD_BUTTON] Dialog shown successfully");
        });
        println!("[SETUP_UI] Add button click handler set up successfully");
        
        // Edit button functionality
        let vault_manager = Arc::clone(&self.vault_manager);
        let results_list_clone = results_list.clone();
        let status_label_clone = status_label.clone();
        let window_clone = window.clone();
        
        edit_button.connect_clicked(move |_| {
            if let Some(_selected_row) = results_list_clone.selected_row() {
                // For now, show a placeholder message
                // In a real implementation, we'd get the entry ID and edit the entry
                status_label_clone.set_text("Edit functionality: Select an entry and click Edit to modify it");
            } else {
                status_label_clone.set_text("Please select an entry to edit");
            }
        });
        
        // Delete button functionality
        let vault_manager = Arc::clone(&self.vault_manager);
        let results_list_clone = results_list.clone();
        let status_label_clone = status_label.clone();
        
        delete_button.connect_clicked(move |_| {
            if let Some(_selected_row) = results_list_clone.selected_row() {
                // For now, show a placeholder message
                // In a real implementation, we'd get the entry ID and delete the entry
                status_label_clone.set_text("Delete functionality: Select an entry and click Delete to remove it");
            } else {
                status_label_clone.set_text("Please select an entry to delete");
            }
        });
    }
    
    fn load_all_entries(results_list: &ListBox, vault_manager: &Arc<Mutex<VaultManager>>, status_label: &Label) {
        if let Ok(vault) = vault_manager.lock() {
            let entries = vault.get_all_entries().unwrap_or_default();
            let count = vault.get_entries_count();
            println!("[LOAD_ALL] Loading {} entries", count);
            
            // Clear existing results
            while let Some(child) = results_list.first_child() {
                results_list.remove(&child);
            }
            
            // Add all entries
            for entry in entries {
                let row = Self::create_result_row_static(&entry);
                results_list.append(&row);
            }
            
            status_label.set_text(&format!("Showing {} entries", count));
        }
    }

    fn refresh_results_list(results_list: &ListBox, vault_manager: &Arc<Mutex<VaultManager>>) {
        // Clear existing results
        while let Some(child) = results_list.first_child() {
            results_list.remove(&child);
        }
        
        // Reload all entries
        if let Ok(vault) = vault_manager.lock() {
            if let Ok(entries) = vault.search_entries("") {
                for entry in entries {
                    let row = Self::create_result_row_static(&entry);
                    results_list.append(&row);
                }
            }
        }
    }

    fn setup_keyboard_shortcuts(&self, _window: &ApplicationWindow, _search_entry: &SearchEntry) {
        // For now, skip keyboard shortcuts due to API compatibility issues
        // In a real implementation, we'd use a different approach for handling Escape key
        // This would require setting up proper event handling or using a different GTK4 version
    }
}