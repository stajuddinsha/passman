use gtk4::prelude::*;
use gtk4::{
    ApplicationWindow, ListBoxRow, Box as GtkBox, 
    Orientation, Label, Entry, Dialog, ResponseType, Button
};
use crate::vault::PasswordEntry;

pub struct UnlockDialog {
    dialog: Dialog,
    password_entry: Entry,
}

impl UnlockDialog {
    pub fn new(parent: &ApplicationWindow) -> Self {
        let dialog = Dialog::builder()
            .title("Unlock Vault")
            .transient_for(parent)
            .modal(true)
            .build();

        let content_area = dialog.content_area();
        let vbox = GtkBox::new(Orientation::Vertical, 12);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);

        let label = Label::new(Some("Enter master password:"));
        let password_entry = Entry::builder()
            .placeholder_text("Master password")
            .visibility(false)
            .build();

        vbox.append(&label);
        vbox.append(&password_entry);

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Unlock", ResponseType::Accept);

        content_area.append(&vbox);

        Self {
            dialog,
            password_entry,
        }
    }

    pub fn run(&self) -> Option<String> {
        self.password_entry.grab_focus();
        
        // For now, return a placeholder - dialog.run() has compatibility issues
        // In a real implementation, we'd use a different approach
        Some("placeholder_password".to_string())
    }
}

pub struct AddEntryDialog {
    dialog: ApplicationWindow,
    name_entry: Entry,
    username_entry: Entry,
    password_entry: Entry,
    url_entry: Entry,
    tags_entry: Entry,
}

impl AddEntryDialog {
    pub fn new(parent: &ApplicationWindow) -> Self {
        // Create a new window for the dialog
        let dialog = ApplicationWindow::builder()
            .title("Add Password Entry")
            .transient_for(parent)
            .modal(true)
            .default_width(400)
            .default_height(500)
            .resizable(false)
            .build();

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

        // Username field
        let username_label = Label::new(Some("Username:"));
        let username_entry = Entry::builder()
            .placeholder_text("Username or email")
            .build();

        // Password field
        let password_label = Label::new(Some("Password:"));
        let password_entry = Entry::builder()
            .placeholder_text("Password")
            .visibility(false)
            .build();

        // URL field
        let url_label = Label::new(Some("URL:"));
        let url_entry = Entry::builder()
            .placeholder_text("https://example.com")
            .build();

        // Tags field
        let tags_label = Label::new(Some("Tags (comma-separated):"));
        let tags_entry = Entry::builder()
            .placeholder_text("work, important, social")
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
        vbox.append(&username_label);
        vbox.append(&username_entry);
        vbox.append(&password_label);
        vbox.append(&password_entry);
        vbox.append(&url_label);
        vbox.append(&url_entry);
        vbox.append(&tags_label);
        vbox.append(&tags_entry);
        vbox.append(&button_box);

        // Add buttons to button box
        button_box.append(&cancel_button);
        button_box.append(&save_button);

        // Set window content
        dialog.set_child(Some(&vbox));

        Self {
            dialog,
            name_entry,
            username_entry,
            password_entry,
            url_entry,
            tags_entry,
        }
    }

    pub fn run(&self) -> Option<PasswordEntry> {
        // Show the dialog window
        self.dialog.present();
        
        // For now, create a test entry with form data
        // In a real implementation, we'd capture the actual form values
        let name = self.name_entry.text().to_string();
        let username = self.username_entry.text().to_string();
        let password = self.password_entry.text().to_string();
        let url = self.url_entry.text().to_string();
        let tags_text = self.tags_entry.text().to_string();
        
        // Parse tags
        let tags: Vec<String> = if tags_text.is_empty() {
            vec![]
        } else {
            tags_text.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };
        
        // Create entry with form data or defaults
        Some(PasswordEntry {
            id: uuid::Uuid::new_v4().to_string(),
            name: if name.is_empty() { "New Entry".to_string() } else { name },
            username: if username.is_empty() { None } else { Some(username) },
            password: if password.is_empty() { "new_password".to_string() } else { password },
            url: if url.is_empty() { None } else { Some(url) },
            tags,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
}

pub fn create_result_row(entry: &PasswordEntry) -> ListBoxRow {
    let row = ListBoxRow::new();
    let hbox = GtkBox::new(Orientation::Horizontal, 12);
    
    // Service name
    let name_label = Label::new(Some(&entry.name));
    name_label.set_halign(gtk4::Align::Start);
    
    // Username (if available)
    let username_label = if let Some(username) = &entry.username {
        let label = Label::new(Some(username));
        label.set_halign(gtk4::Align::Start);
        Some(label)
    } else {
        None
    };
    
    hbox.append(&name_label);
    if let Some(username_label) = username_label {
        hbox.append(&username_label);
    }
    
    row.set_child(Some(&hbox));
    row
}