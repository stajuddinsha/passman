use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: String,
    pub name: String,
    pub username: Option<String>,
    pub password: String,
    pub url: Option<String>,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct VaultManager {
    config: Config,
    entries: HashMap<String, PasswordEntry>,
    is_locked: bool,
}

impl VaultManager {
    pub fn new(config: &Config) -> Result<Self> {
        let mut vault = Self {
            config: config.clone(),
            entries: HashMap::new(),
            is_locked: true,
        };
        
        // For now, unlock the vault automatically
        // In a real implementation, this would require user authentication
        vault.unlock("")?;
        
        Ok(vault)
    }

    pub fn unlock(&mut self, _password: &str) -> Result<()> {
        // Simple implementation - in real version would decrypt vault
        self.is_locked = false;
        self.load_entries()?;
        Ok(())
    }

    pub fn lock(&mut self) {
        self.entries.clear();
        self.is_locked = true;
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    pub fn add_entry(&mut self, entry: PasswordEntry) -> Result<()> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Vault is locked"));
        }

        self.entries.insert(entry.id.clone(), entry);
        self.save_entries()?;
        Ok(())
    }

    pub fn delete_entry(&mut self, id: &str) -> Result<()> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Vault is locked"));
        }

        if self.entries.remove(id).is_some() {
            self.save_entries()?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Entry not found"))
        }
    }

    pub fn update_entry(&mut self, entry: PasswordEntry) -> Result<()> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Vault is locked"));
        }

        if self.entries.contains_key(&entry.id) {
            self.entries.insert(entry.id.clone(), entry);
            self.save_entries()?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Entry not found"))
        }
    }

    pub fn search_entries(&self, query: &str) -> Result<Vec<PasswordEntry>> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Vault is locked"));
        }

        let mut results: Vec<PasswordEntry> = self.entries
            .values()
            .filter(|entry| {
                entry.name.to_lowercase().contains(&query.to_lowercase()) ||
                entry.username.as_ref().map_or(false, |u| u.to_lowercase().contains(&query.to_lowercase())) ||
                entry.tags.iter().any(|tag| tag.to_lowercase().contains(&query.to_lowercase()))
            })
            .cloned()
            .collect();

        // Sort by relevance (simple implementation)
        results.sort_by(|a, b| {
            let a_score = self.calculate_relevance_score(a, query);
            let b_score = self.calculate_relevance_score(b, query);
            b_score.cmp(&a_score)
        });

        Ok(results)
    }

    fn calculate_relevance_score(&self, entry: &PasswordEntry, query: &str) -> i32 {
        let query_lower = query.to_lowercase();
        let mut score = 0;

        if entry.name.to_lowercase().starts_with(&query_lower) {
            score += 100;
        } else if entry.name.to_lowercase().contains(&query_lower) {
            score += 50;
        }

        if let Some(username) = &entry.username {
            if username.to_lowercase().contains(&query_lower) {
                score += 25;
            }
        }

        for tag in &entry.tags {
            if tag.to_lowercase().contains(&query_lower) {
                score += 10;
            }
        }

        score
    }

    fn load_entries(&mut self) -> Result<()> {
        // Try to load from vault.json file
        let vault_file = std::path::Path::new("vault.json");
        if vault_file.exists() {
            println!("[VAULT] Loading entries from vault.json");
            let contents = std::fs::read_to_string(vault_file)?;
            let loaded_entries: HashMap<String, PasswordEntry> = serde_json::from_str(&contents)?;
            self.entries = loaded_entries;
            println!("[VAULT] Loaded {} entries from vault.json", self.entries.len());
        } else {
            println!("[VAULT] No vault.json found, creating sample entries");
            // Create some sample entries if no vault file exists
            let sample_entries = vec![
                PasswordEntry {
                    id: "1".to_string(),
                    name: "Gmail".to_string(),
                    username: Some("user@gmail.com".to_string()),
                    password: "encrypted_password_1".to_string(),
                    url: Some("https://gmail.com".to_string()),
                    tags: vec!["email".to_string(), "google".to_string()],
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
                PasswordEntry {
                    id: "2".to_string(),
                    name: "GitHub".to_string(),
                    username: Some("developer".to_string()),
                    password: "encrypted_password_2".to_string(),
                    url: Some("https://github.com".to_string()),
                    tags: vec!["development".to_string(), "git".to_string()],
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
            ];

            for entry in sample_entries {
                self.entries.insert(entry.id.clone(), entry);
            }
        }

        Ok(())
    }

    fn save_entries(&self) -> Result<()> {
        // Simple file-based persistence for now
        let vault_file = std::path::Path::new("vault.json");
        let entries_json = serde_json::to_string_pretty(&self.entries)?;
        std::fs::write(vault_file, entries_json)?;
        println!("[VAULT] Saved {} entries to vault.json", self.entries.len());
        Ok(())
    }

    pub fn get_entry(&self, id: &str) -> Option<&PasswordEntry> {
        self.entries.get(id)
    }

    pub fn get_entry_password(&self, id: &str) -> Result<Option<String>> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Vault is locked"));
        }

        if let Some(entry) = self.entries.get(id) {
            // In a real implementation, this would decrypt the password
            Ok(Some(entry.password.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_entries(&self) -> Result<Vec<PasswordEntry>> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Vault is locked"));
        }

        Ok(self.entries.values().cloned().collect())
    }

    pub fn get_entries_count(&self) -> usize {
        self.entries.len()
    }
}