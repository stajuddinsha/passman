use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
    sync::atomic::{AtomicBool, Ordering},
};

mod config;
mod vault;
mod clipboard;
mod search;

use vault::PasswordEntry;

#[derive(Clone)]
pub struct App {
    pub should_quit: bool,
    pub search_query: String,
    pub entries: Vec<PasswordEntry>,
    pub filtered_entries: Vec<PasswordEntry>,
    pub selected_index: usize,
    pub list_state: ListState,
    pub mode: AppMode,
    pub status_message: String,
    pub status_timer: Option<Instant>,
}

#[derive(Clone, PartialEq)]
pub enum AppMode {
    Search,
    Add,
    Edit,
    Delete,
}

impl Default for App {
    fn default() -> App {
        App {
            should_quit: false,
            search_query: String::new(),
            entries: Vec::new(),
            filtered_entries: Vec::new(),
            selected_index: 0,
            list_state: ListState::default(),
            mode: AppMode::Search,
            status_message: String::new(),
            status_timer: None,
        }
    }
}

impl App {
    pub fn new() -> Result<Self> {
        let mut app = Self::default();
        app.load_entries()?;
        app.filter_entries();
        Ok(app)
    }

    fn load_entries(&mut self) -> Result<()> {
        // Load entries from vault file in user's home directory
        let vault_path = self.get_vault_path()?;
        
        if vault_path.exists() {
            let vault_content = std::fs::read_to_string(&vault_path)?;
            
            // Parse as array format (new simplified format)
            if let Ok(entries) = serde_json::from_str::<Vec<PasswordEntry>>(&vault_content) {
                self.entries = entries;
                println!("[TUI] Loaded {} entries from {:?}", self.entries.len(), vault_path);
            } else {
                println!("[TUI] Could not parse vault file, starting with empty vault");
                self.entries = Vec::new();
            }
        } else {
            // Create empty vault if it doesn't exist
            self.entries = Vec::new();
            println!("[TUI] No vault file found at {:?}, starting with empty vault", vault_path);
        }
        Ok(())
    }

    fn save_entries(&self) {
        // Save entries to vault file in user's home directory
        if let Ok(vault_path) = self.get_vault_path() {
            if let Ok(entries_json) = serde_json::to_string_pretty(&self.entries) {
                if let Err(e) = std::fs::write(&vault_path, entries_json) {
                    eprintln!("Error saving entries to {:?}: {}", vault_path, e);
                } else {
                    println!("[TUI] Saved {} entries to {:?}", self.entries.len(), vault_path);
                }
            }
        } else {
            eprintln!("Error getting vault path for saving");
        }
    }

    fn get_vault_path(&self) -> Result<std::path::PathBuf> {
        // Get user's home directory
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        
        // Create .passman directory if it doesn't exist
        let passman_dir = home_dir.join(".passman");
        if !passman_dir.exists() {
            std::fs::create_dir_all(&passman_dir)?;
        }
        
        // Return path to vault.json in .passman directory
        Ok(passman_dir.join("vault.json"))
    }

    fn filter_entries(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_entries = self.entries.clone();
        } else {
            self.filtered_entries = self.entries
                .iter()
                .filter(|entry| {
                    entry.name.to_lowercase().contains(&self.search_query.to_lowercase())
                })
                .cloned()
                .collect();
        }
        
        // Reset selection if out of bounds
        if self.selected_index >= self.filtered_entries.len() {
            self.selected_index = 0;
        }
        
        // Update list state
        if !self.filtered_entries.is_empty() {
            self.list_state.select(Some(self.selected_index));
        } else {
            self.list_state.select(None);
        }
    }

    fn next_entry(&mut self) {
        if !self.filtered_entries.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.filtered_entries.len();
            self.list_state.select(Some(self.selected_index));
        }
    }

    fn previous_entry(&mut self) {
        if !self.filtered_entries.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.filtered_entries.len() - 1
            } else {
                self.selected_index - 1
            };
            self.list_state.select(Some(self.selected_index));
        }
    }

    fn get_selected_entry(&self) -> Option<&PasswordEntry> {
        self.filtered_entries.get(self.selected_index)
    }

    fn copy_password(&mut self) {
        if let Some(entry) = self.get_selected_entry() {
            // Copy password to clipboard
            if let Err(e) = self.copy_to_clipboard(&entry.password) {
                self.status_message = format!("Error copying to clipboard: {}", e);
            } else {
                self.status_message = format!("Password for '{}' copied to clipboard", entry.name);
            }
            self.status_timer = Some(Instant::now() + Duration::from_secs(1));
            // Auto-quit after copying password
            self.should_quit = true;
        }
    }

    fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        // Try to copy to clipboard using system commands
        if self.try_wayland_copy(text) || self.try_x11_copy(text) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to copy to clipboard"))
        }
    }

    fn try_wayland_copy(&self, text: &str) -> bool {
        std::process::Command::new("wl-copy")
            .arg(text)
            .output()
            .is_ok()
    }

    fn try_x11_copy(&self, text: &str) -> bool {
        std::process::Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
                child.wait()?;
                Ok(())
            })
            .is_ok()
    }

    fn add_entry(&mut self) {
        self.mode = AppMode::Add;
        self.status_message = "Add mode: Enter name and password (format: name|password)".to_string();
    }

    fn edit_entry(&mut self) {
        if self.get_selected_entry().is_some() {
            self.mode = AppMode::Edit;
            self.status_message = "Edit mode: Enter new name and password (format: name|password)".to_string();
        }
    }

    fn delete_entry(&mut self) {
        if let Some(entry_name) = self.get_selected_entry().map(|e| e.name.clone()) {
            self.mode = AppMode::Delete;
            self.status_message = "Delete mode: Press 'y' to confirm, 'n' to cancel".to_string();
        }
    }

    fn process_input(&mut self, input: &str) {
        match self.mode {
            AppMode::Add => {
                if let Some((name, password)) = input.split_once('|') {
                    let entry = PasswordEntry {
                        name: name.trim().to_string(),
                        password: password.trim().to_string(),
                    };
                    self.entries.push(entry);
                    self.save_entries();
                    self.filter_entries();
                    self.mode = AppMode::Search;
                    self.status_message = "Entry added successfully!".to_string();
                } else {
                    self.status_message = "Invalid format. Use: name|password".to_string();
                }
            }
            AppMode::Edit => {
                if let Some((name, password)) = input.split_once('|') {
                    if let Some(entry) = self.filtered_entries.get_mut(self.selected_index) {
                        let old_name = entry.name.clone();
                        entry.name = name.trim().to_string();
                        entry.password = password.trim().to_string();
                        
                        // Update in main entries list
                        if let Some(main_entry) = self.entries.iter_mut().find(|e| e.name == old_name) {
                            main_entry.name = name.trim().to_string();
                            main_entry.password = password.trim().to_string();
                        }
                        
                        self.save_entries();
                        self.mode = AppMode::Search;
                        self.status_message = "Entry updated successfully!".to_string();
                    }
                } else {
                    self.status_message = "Invalid format. Use: name|password".to_string();
                }
            }
            AppMode::Delete => {
                match input.to_lowercase().as_str() {
                    "y" | "yes" => {
                        if let Some(entry_name) = self.get_selected_entry().map(|e| e.name.clone()) {
                            self.entries.retain(|e| e.name != entry_name);
                            self.save_entries();
                            self.filter_entries();
                            self.mode = AppMode::Search;
                            self.status_message = "Entry deleted successfully!".to_string();
                        }
                    }
                    "n" | "no" => {
                        self.mode = AppMode::Search;
                        self.status_message = "Delete cancelled".to_string();
                    }
                    _ => {
                        self.status_message = "Press 'y' to confirm or 'n' to cancel".to_string();
                    }
                }
            }
            AppMode::Search => {
                // This shouldn't happen in search mode
            }
        }
    }
}

fn main() -> Result<()> {
    // Setup panic handler to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Force restore terminal on panic
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        original_hook(panic_info);
    }));

    // Setup signal handler for Ctrl+C
    let running = std::sync::Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        // Force restore terminal on interrupt
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        std::process::exit(0);
    })?;

    // Setup terminal with cleanup guard
    let _guard = TerminalGuard;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;
    let res = run_app(&mut terminal, &mut app);

    // Always restore terminal, even on panic
    restore_terminal(&mut terminal);

    if let Err(err) = res {
        eprintln!("{err:?}");
    }

    Ok(())
}

fn restore_terminal<B: Backend>(terminal: &mut Terminal<B>) {
    // Force restore terminal state
    let _ = disable_raw_mode();
    let _ = execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    let _ = terminal.show_cursor();
    
    // Additional cleanup
    let _ = terminal.clear();
    let _ = terminal.flush();
}

// Terminal cleanup guard
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Force restore terminal state on drop
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.mode {
                    AppMode::Search => {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.should_quit = true;
                                return Ok(());
                            }
                            KeyCode::Char('a') => app.add_entry(),
                            KeyCode::Char('e') => app.edit_entry(),
                            KeyCode::Char('d') => app.delete_entry(),
                            KeyCode::Char(c) => {
                                app.search_query.push(c);
                                app.filter_entries();
                            }
                            KeyCode::Backspace => {
                                app.search_query.pop();
                                app.filter_entries();
                            }
                            KeyCode::Down => app.next_entry(),
                            KeyCode::Up => app.previous_entry(),
                            KeyCode::Enter => app.copy_password(),
                            KeyCode::Esc => {
                                app.search_query.clear();
                                app.filter_entries();
                            }
                            _ => {}
                        }
                    }
                    AppMode::Add | AppMode::Edit => {
                        match key.code {
                            KeyCode::Enter => {
                                let input = app.search_query.clone();
                                app.process_input(&input);
                                app.search_query.clear();
                            }
                            KeyCode::Esc => {
                                app.mode = AppMode::Search;
                                app.search_query.clear();
                                app.status_message.clear();
                            }
                            KeyCode::Char(c) => {
                                app.search_query.push(c);
                            }
                            KeyCode::Backspace => {
                                app.search_query.pop();
                            }
                            _ => {}
                        }
                    }
                    AppMode::Delete => {
                        match key.code {
                            KeyCode::Char(c) => {
                                app.process_input(&c.to_string());
                            }
                            KeyCode::Esc => {
                                app.mode = AppMode::Search;
                                app.status_message.clear();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Clear status message after timeout
        if let Some(timer) = app.status_timer {
            if Instant::now() >= timer {
                app.status_message.clear();
                app.status_timer = None;
            }
        }

        // Check if we should quit (e.g., after copying password)
        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Length(3), // Search bar
                Constraint::Min(10),    // List
                Constraint::Length(3),  // Status
            ]
            .as_ref(),
        )
        .split(f.size());

    // Header
    let header = Paragraph::new("🔐 Keytui - Password Manager")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Search bar
    let search_text = if app.mode == AppMode::Search {
        format!("🔍 Search: {}", app.search_query)
    } else {
        format!("📝 {}: {}", 
            match app.mode {
                AppMode::Add => "Add Entry",
                AppMode::Edit => "Edit Entry", 
                AppMode::Delete => "Delete Entry",
                AppMode::Search => "Search",
            },
            app.search_query
        )
    };
    
    let search_style = match app.mode {
        AppMode::Search => Style::default().fg(Color::Green),
        AppMode::Add => Style::default().fg(Color::Blue),
        AppMode::Edit => Style::default().fg(Color::Yellow),
        AppMode::Delete => Style::default().fg(Color::Red),
    };
    
    let search = Paragraph::new(search_text)
        .style(search_style)
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(search, chunks[1]);

    // List
    let items: Vec<ListItem> = app.filtered_entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let is_selected = i == app.selected_index;
            let style = if is_selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{}", entry.name),
                    style.fg(Color::Cyan),
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Entries"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, chunks[2], &mut app.list_state.clone());

    // Status bar
    let status_text = if !app.status_message.is_empty() {
        app.status_message.clone()
    } else {
        match app.mode {
            AppMode::Search => "↑↓ Navigate | Enter: Copy | a: Add | e: Edit | d: Delete | Esc: Clear | q: Quit".to_string(),
            AppMode::Add => "Enter: Save | Esc: Cancel".to_string(),
            AppMode::Edit => "Enter: Save | Esc: Cancel".to_string(),
            AppMode::Delete => "y: Confirm | n: Cancel | Esc: Cancel".to_string(),
        }
    };
    
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status, chunks[3]);
}
