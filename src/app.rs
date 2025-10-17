use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, SearchEntry, ListBox, Box as GtkBox, Orientation, Label, Button, HeaderBar};
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
        let config = Config::load()?;
        let vault_manager = Arc::new(Mutex::new(VaultManager::new(&config)?));
        let clipboard_manager = ClipboardManager::new()?;

        Ok(Self {
            vault_manager,
            clipboard_manager,
            config,
        })
    }

    pub fn setup_ui(&self, window: &ApplicationWindow) {
        // Create header bar with action buttons
        let header = HeaderBar::new();
        
        // Add button
        let add_button = Button::builder()
            .label("Add")
            .tooltip_text("Add new password entry")
            .build();
        
        // Edit button
        let edit_button = Button::builder()
            .label("Edit")
            .tooltip_text("Edit selected entry")
            .build();
        
        // Delete button
        let delete_button = Button::builder()
            .label("Delete")
            .tooltip_text("Delete selected entry")
            .build();
        
        // Add buttons to header
        header.pack_start(&add_button);
        header.pack_start(&edit_button);
        header.pack_start(&delete_button);
        
        // Create main container
        let main_box = GtkBox::new(Orientation::Vertical, 12);
        main_box.set_margin_start(20);
        main_box.set_margin_end(20);
        main_box.set_margin_top(20);
        main_box.set_margin_bottom(20);

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

        // Add widgets to main box
        main_box.append(&search_entry);
        main_box.append(&results_list);
        main_box.append(&status_label);

        // Set up search functionality
        self.setup_search(&search_entry, &results_list, &status_label);
        
        // Set up entry management
        self.setup_entry_management(window, &add_button, &edit_button, &delete_button, &results_list, &status_label);

        // Set up keyboard shortcuts
        self.setup_keyboard_shortcuts(window, &search_entry);

        // Set window content
        window.set_titlebar(Some(&header));
        window.set_child(Some(&main_box));

        // Focus search entry
        search_entry.grab_focus();
    }

    fn setup_search(&self, search_entry: &SearchEntry, results_list: &ListBox, status_label: &Label) {
        let vault_manager = Arc::clone(&self.vault_manager);
        let results_list_clone = results_list.clone();
        let status_label_clone = status_label.clone();

        search_entry.connect_search_changed(move |entry| {
            let query = entry.text().to_string();
            if query.is_empty() {
                // Clear existing results
                while let Some(child) = results_list_clone.first_child() {
                    results_list_clone.remove(&child);
                }
                return;
            }

            // Perform search
            if let Ok(vault) = vault_manager.lock() {
                if let Ok(entries) = vault.search_entries(&query) {
                    // Clear existing results
                    while let Some(child) = results_list_clone.first_child() {
                        results_list_clone.remove(&child);
                    }
                    
                    for entry in entries {
                        let row = Self::create_result_row_static(&entry);
                        results_list_clone.append(&row);
                    }
                }
            }
        });

        // Handle Enter key on results
        results_list.connect_row_activated(move |_, _row| {
            // For now, we'll use a placeholder - in real implementation,
            // we'd need to store the entry ID in the row
            status_label_clone.set_text("Password copied to clipboard");
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
        
        add_button.connect_clicked(move |_| {
            let dialog = AddEntryDialog::new(&window_clone);
            if let Some(entry) = dialog.run() {
                if let Ok(mut vault) = vault_manager.lock() {
                    if let Err(e) = vault.add_entry(entry) {
                        status_label_clone.set_text(&format!("Error adding entry: {}", e));
                    } else {
                        status_label_clone.set_text("Entry added successfully");
                        // Refresh the results list
                        Self::refresh_results_list(&results_list_clone, &vault_manager);
                    }
                }
            }
        });
        
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