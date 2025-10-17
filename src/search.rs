use crate::vault::PasswordEntry;

pub struct SearchEngine {
    // Simple search implementation without external dependencies
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn search(&self, entries: &[PasswordEntry], query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();

        for entry in entries {
            let search_text = entry.name.clone();

            if self.matches(&search_text, query) {
                let score = self.calculate_score(&search_text, query);
                results.push(SearchResult {
                    entry: entry.clone(),
                    score,
                });
            }
        }

        // Sort by score (higher is better)
        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }

    fn matches(&self, text: &str, query: &str) -> bool {
        let text_lower = text.to_lowercase();
        let query_lower = query.to_lowercase();
        text_lower.contains(&query_lower)
    }

    fn calculate_score(&self, text: &str, query: &str) -> i32 {
        let text_lower = text.to_lowercase();
        let query_lower = query.to_lowercase();
        
        if text_lower.starts_with(&query_lower) {
            100
        } else if text_lower.contains(&query_lower) {
            50
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub entry: PasswordEntry,
    pub score: i32,
}
