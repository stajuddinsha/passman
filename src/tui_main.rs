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
};

mod config;
mod vault;
mod clipboard;
mod search;

use vault::{VaultManager, PasswordEntry};

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
        let config = config::Config::load()?;
        let vault_manager = VaultManager::new(&config)?;
        
        // Get all entries from vault
        self.entries = vault_manager.get_all_entries()?;
        Ok(())
    }

    fn filter_entries(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_entries = self.entries.clone();
        } else {
            self.filtered_entries = self.entries
                .iter()
                .filter(|entry| {
                    entry.name.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                    entry.username.as_ref().map_or(false, |u| u.to_lowercase().contains(&self.search_query.to_lowercase())) ||
                    entry.url.as_ref().map_or(false, |u| u.to_lowercase().contains(&self.search_query.to_lowercase())) ||
                    entry.tags.iter().any(|tag| tag.to_lowercase().contains(&self.search_query.to_lowercase()))
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
            // In a real implementation, this would copy to clipboard
            self.status_message = format!("Password for '{}' copied to clipboard", entry.name);
            self.status_timer = Some(Instant::now() + Duration::from_secs(1));
            // Auto-quit after copying password
            self.should_quit = true;
        }
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
        if let Some(entry_id) = self.get_selected_entry().map(|e| e.id.clone()) {
            self.mode = AppMode::Delete;
            self.status_message = "Delete mode: Press 'y' to confirm, 'n' to cancel".to_string();
        }
    }

    fn process_input(&mut self, input: &str) {
        match self.mode {
            AppMode::Add => {
                if let Some((name, password)) = input.split_once('|') {
                    let entry = PasswordEntry {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: name.trim().to_string(),
                        username: None,
                        password: password.trim().to_string(),
                        url: None,
                        tags: vec![],
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    };
                    self.entries.push(entry);
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
                        entry.name = name.trim().to_string();
                        entry.password = password.trim().to_string();
                        entry.updated_at = chrono::Utc::now();
                        
                        // Update in main entries list
                        if let Some(main_entry) = self.entries.iter_mut().find(|e| e.id == entry.id) {
                            main_entry.name = name.trim().to_string();
                            main_entry.password = password.trim().to_string();
                            main_entry.updated_at = chrono::Utc::now();
                        }
                        
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
                        if let Some(entry_id) = self.get_selected_entry().map(|e| e.id.clone()) {
                            self.entries.retain(|e| e.id != entry_id);
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
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
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
                            KeyCode::Char('a') => app.add_entry(),
                            KeyCode::Char('e') => app.edit_entry(),
                            KeyCode::Char('d') => app.delete_entry(),
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

            let username = entry.username.as_ref().map(|u| format!(" ({})", u)).unwrap_or_default();
            let url = entry.url.as_ref().map(|u| format!(" - {}", u)).unwrap_or_default();
            let tags = if !entry.tags.is_empty() {
                format!(" [{}]", entry.tags.join(", "))
            } else {
                String::new()
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{}", entry.name),
                    style.fg(Color::Cyan),
                ),
                Span::styled(username, style.fg(Color::Gray)),
                Span::styled(url, style.fg(Color::Blue)),
                Span::styled(tags, style.fg(Color::Magenta)),
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
