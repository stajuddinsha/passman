use anyhow::Result;
use std::io::{self, Write};
use std::env;

mod config;
mod vault;
mod clipboard;
mod search;

use vault::{VaultManager, PasswordEntry};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];
    
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a name for the entry");
                eprintln!("Usage: passman add <name>");
                return Ok(());
            }
            add_entry(&args[2])?;
        }
        "delete" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a name for the entry to delete");
                eprintln!("Usage: passman delete <name>");
                return Ok(());
            }
            delete_entry(&args[2])?;
        }
        "list" => {
            list_entries()?;
        }
        "search" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a search term");
                eprintln!("Usage: passman search <term>");
                return Ok(());
            }
            search_entries(&args[2])?;
        }
        "help" | "--help" | "-h" => {
            print_usage();
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage();
        }
    }
    
    Ok(())
}

fn print_usage() {
    println!("Keytui CLI - Password Manager");
    println!();
    println!("Usage: passman <command> [options]");
    println!();
    println!("Commands:");
    println!("  add <name>     Add a new password entry");
    println!("  delete <name>  Delete a password entry");
    println!("  list           List all password entries");
    println!("  search <term>  Search for password entries");
    println!("  help           Show this help message");
    println!();
    println!("Examples:");
    println!("  passman add gmail");
    println!("  passman delete gmail");
    println!("  passman list");
    println!("  passman search gmail");
}

fn add_entry(name: &str) -> Result<()> {
    // Load vault
    let config = config::Config::load()?;
    let mut vault_manager = VaultManager::new(&config)?;
    
    // Check if entry already exists
    if vault_manager.get_all_entries()?.iter().any(|e| e.name == name) {
        eprintln!("Error: Entry '{}' already exists", name);
        return Ok(());
    }
    
    // Get password from user
    print!("Enter password for '{}': ", name);
    io::stdout().flush()?;
    
    let password = rpassword::read_password()?;
    if password.is_empty() {
        eprintln!("Error: Password cannot be empty");
        return Ok(());
    }
    
    // Create entry
    let entry = PasswordEntry {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.to_string(),
        username: None,
        password,
        url: None,
        tags: vec![],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    // Add to vault
    vault_manager.add_entry(entry)?;
    vault_manager.save_entries()?;
    
    println!("✅ Entry '{}' added successfully!", name);
    Ok(())
}

fn delete_entry(name: &str) -> Result<()> {
    // Load vault
    let config = config::Config::load()?;
    let mut vault_manager = VaultManager::new(&config)?;
    
    // Find entry by name
    let entries = vault_manager.get_all_entries()?;
    let entry = entries.iter().find(|e| e.name == name);
    
    match entry {
        Some(entry) => {
            vault_manager.delete_entry(&entry.id)?;
            vault_manager.save_entries()?;
            println!("✅ Entry '{}' deleted successfully!", name);
        }
        None => {
            eprintln!("Error: Entry '{}' not found", name);
        }
    }
    
    Ok(())
}

fn list_entries() -> Result<()> {
    // Load vault
    let config = config::Config::load()?;
    let vault_manager = VaultManager::new(&config)?;
    
    let entries = vault_manager.get_all_entries()?;
    
    if entries.is_empty() {
        println!("No entries found.");
        return Ok(());
    }
    
    println!("Password Entries:");
    println!("=================");
    
    for entry in entries {
        let username = entry.username.as_ref().map(|u| format!(" ({})", u)).unwrap_or_default();
        let url = entry.url.as_ref().map(|u| format!(" - {}", u)).unwrap_or_default();
        let tags = if !entry.tags.is_empty() {
            format!(" [{}]", entry.tags.join(", "))
        } else {
            String::new()
        };
        
        println!("• {}{}{}{}", entry.name, username, url, tags);
    }
    
    Ok(())
}

fn search_entries(term: &str) -> Result<()> {
    // Load vault
    let config = config::Config::load()?;
    let vault_manager = VaultManager::new(&config)?;
    
    let entries = vault_manager.get_all_entries()?;
    let matches: Vec<_> = entries
        .iter()
        .filter(|entry| {
            entry.name.to_lowercase().contains(&term.to_lowercase()) ||
            entry.username.as_ref().map_or(false, |u| u.to_lowercase().contains(&term.to_lowercase())) ||
            entry.url.as_ref().map_or(false, |u| u.to_lowercase().contains(&term.to_lowercase())) ||
            entry.tags.iter().any(|tag| tag.to_lowercase().contains(&term.to_lowercase()))
        })
        .collect();
    
    if matches.is_empty() {
        println!("No entries found matching '{}'", term);
        return Ok(());
    }
    
    println!("Search results for '{}':", term);
    println!("==========================");
    
    for entry in matches {
        let username = entry.username.as_ref().map(|u| format!(" ({})", u)).unwrap_or_default();
        let url = entry.url.as_ref().map(|u| format!(" - {}", u)).unwrap_or_default();
        let tags = if !entry.tags.is_empty() {
            format!(" [{}]", entry.tags.join(", "))
        } else {
            String::new()
        };
        
        println!("• {}{}{}{}", entry.name, username, url, tags);
    }
    
    Ok(())
}
