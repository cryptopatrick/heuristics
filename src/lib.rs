//! A searchable collection of computer science and Rust development heuristics.
//!
//! This crate provides curated rules of thumb for choosing the right data structures,
//! algorithms, and architectural patterns in Rust development.

use std::collections::HashMap;

/// A single heuristic with its metadata
#[derive(Debug, Clone)]
pub struct Heuristic {
    /// The main title/question (e.g., "Need O(1) average-case lookups or inserts?")
    pub title: String,
    /// The recommended action
    pub action: String,
    /// The category this heuristic belongs to
    pub category: String,
    /// Full markdown content of this heuristic
    pub content: String,
    /// Associated Rust crates mentioned
    pub crates: Vec<String>,
    /// Standard library types mentioned
    pub std_types: Vec<String>,
    /// Keywords for searching
    pub keywords: Vec<String>,
}

/// Database of searchable heuristics
pub struct HeuristicDb {
    heuristics: Vec<Heuristic>,
    /// Inverted index: lowercase keyword -> heuristic indices
    index: HashMap<String, Vec<usize>>,
}

impl HeuristicDb {
    /// Create a new database from parsed heuristics
    pub fn new(heuristics: Vec<Heuristic>) -> Self {
        let mut index: HashMap<String, Vec<usize>> = HashMap::new();

        for (idx, heuristic) in heuristics.iter().enumerate() {
            // Index all keywords
            for keyword in &heuristic.keywords {
                index.entry(keyword.to_lowercase())
                    .or_default()
                    .push(idx);
            }

            // Index crate names
            for crate_name in &heuristic.crates {
                index.entry(crate_name.to_lowercase())
                    .or_default()
                    .push(idx);
            }

            // Index std types
            for std_type in &heuristic.std_types {
                index.entry(std_type.to_lowercase())
                    .or_default()
                    .push(idx);
            }

            // Index category
            index.entry(heuristic.category.to_lowercase())
                .or_default()
                .push(idx);
        }

        Self { heuristics, index }
    }

    /// Search for heuristics by keywords
    /// Returns heuristics ranked by number of keyword matches
    pub fn search(&self, keywords: &[&str]) -> Vec<&Heuristic> {
        let mut scores: HashMap<usize, usize> = HashMap::new();

        for keyword in keywords {
            let normalized = keyword.to_lowercase();

            // Exact matches
            if let Some(indices) = self.index.get(&normalized) {
                for &idx in indices {
                    *scores.entry(idx).or_default() += 2;
                }
            }

            // Partial matches
            for (indexed_keyword, indices) in &self.index {
                if indexed_keyword.contains(&normalized) || normalized.contains(indexed_keyword) {
                    for &idx in indices {
                        *scores.entry(idx).or_default() += 1;
                    }
                }
            }
        }

        // Sort by score (descending)
        let mut results: Vec<(usize, usize)> = scores.into_iter().collect();
        results.sort_by(|a, b| b.1.cmp(&a.1));

        results.into_iter()
            .map(|(idx, _score)| &self.heuristics[idx])
            .collect()
    }

    /// Get all heuristics in a category
    pub fn by_category(&self, category: &str) -> Vec<&Heuristic> {
        self.heuristics
            .iter()
            .filter(|h| h.category.to_lowercase() == category.to_lowercase())
            .collect()
    }

    /// Get all unique categories
    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self.heuristics
            .iter()
            .map(|h| h.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    /// Get all heuristics
    pub fn all(&self) -> &[Heuristic] {
        &self.heuristics
    }
}

/// Parse the base.md file and build the heuristic database
pub fn load_heuristics() -> HeuristicDb {
    let content = include_str!("../base.md");
    let heuristics = parse_markdown(content);
    HeuristicDb::new(heuristics)
}

/// Parse markdown content into heuristics
fn parse_markdown(content: &str) -> Vec<Heuristic> {
    let mut heuristics = Vec::new();
    let mut current_category = String::new();
    let mut current_title = String::new();
    let mut current_action = String::new();
    let mut current_content = String::new();
    let mut current_crates = Vec::new();
    let mut current_std_types = Vec::new();
    let mut current_keywords = Vec::new();

    let mut in_heuristic = false;

    for line in content.lines() {
        // Category headers (## ...)
        if let Some(cat) = line.strip_prefix("## ") {
            // Save previous heuristic if exists
            if in_heuristic && !current_title.is_empty() {
                heuristics.push(Heuristic {
                    title: current_title.clone(),
                    action: current_action.clone(),
                    category: current_category.clone(),
                    content: current_content.trim().to_string(),
                    crates: current_crates.clone(),
                    std_types: current_std_types.clone(),
                    keywords: current_keywords.clone(),
                });
            }

            current_category = cat.trim().to_string();
            in_heuristic = false;
            current_title.clear();
            current_action.clear();
            current_content.clear();
            current_crates.clear();
            current_std_types.clear();
            current_keywords.clear();
            continue;
        }

        // Heuristic headers (### Need ...)
        if let Some(title) = line.strip_prefix("### ") {
            // Save previous heuristic if exists
            if in_heuristic && !current_title.is_empty() {
                heuristics.push(Heuristic {
                    title: current_title.clone(),
                    action: current_action.clone(),
                    category: current_category.clone(),
                    content: current_content.trim().to_string(),
                    crates: current_crates.clone(),
                    std_types: current_std_types.clone(),
                    keywords: current_keywords.clone(),
                });
            }

            current_title = title.trim().to_string();
            current_content = line.to_string() + "\n";
            in_heuristic = true;
            current_action.clear();
            current_crates.clear();
            current_std_types.clear();
            current_keywords.clear();

            // Extract keywords from title
            extract_keywords(&current_title, &mut current_keywords);
            continue;
        }

        if in_heuristic {
            current_content.push_str(line);
            current_content.push('\n');

            // Extract action
            if let Some(action) = line.strip_prefix("**Action:**") {
                current_action = action.trim().to_string();
                extract_keywords(&current_action, &mut current_keywords);
            }

            // Extract crates
            if line.contains("- **Crates:**") {
                // Next lines contain crate info
            } else if line.trim().starts_with("- `") && line.contains("` -") {
                if let Some(crate_name) = extract_crate_name(line) {
                    current_crates.push(crate_name.clone());
                    current_keywords.push(crate_name);
                }
            }

            // Extract std types
            if line.contains("- **Std types:**") {
                if let Some(types) = line.split("**Std types:**").nth(1) {
                    for part in types.split(',') {
                        if let Some(type_name) = extract_code_name(part.trim()) {
                            current_std_types.push(type_name.clone());
                            current_keywords.push(type_name);
                        }
                    }
                }
            }

            // Extract keywords from various patterns
            if line.contains("**When to use:**") {
                if let Some(use_case) = line.split("**When to use:**").nth(1) {
                    extract_keywords(use_case, &mut current_keywords);
                }
            }
        }
    }

    // Save last heuristic
    if in_heuristic && !current_title.is_empty() {
        heuristics.push(Heuristic {
            title: current_title,
            action: current_action,
            category: current_category,
            content: current_content.trim().to_string(),
            crates: current_crates,
            std_types: current_std_types,
            keywords: current_keywords,
        });
    }

    heuristics
}

fn extract_crate_name(line: &str) -> Option<String> {
    line.trim()
        .strip_prefix("- `")?
        .split('`')
        .next()
        .map(|s| s.to_string())
}

fn extract_code_name(text: &str) -> Option<String> {
    text.trim()
        .strip_prefix('`')?
        .strip_suffix('`')
        .map(|s| s.to_string())
}

fn extract_keywords(text: &str, keywords: &mut Vec<String>) {
    // Extract technical terms (simplified version)
    let terms = [
        "hash", "hashmap", "hashset", "btree", "binary search", "lookup", "insert",
        "cache", "lru", "ttl", "bloom", "filter", "probabilistic",
        "disk", "persistence", "wal", "log", "lsm", "compression",
        "distributed", "shard", "replicate", "consensus", "crdt", "merkle",
        "concurrent", "lock-free", "atomic", "skip list",
        "trie", "prefix", "autocomplete", "heap", "priority queue",
        "geospatial", "rtree", "quadtree", "rope", "text",
        "event sourcing", "time-series", "batch", "async", "append-only",
        "performance", "throughput", "latency", "columnar", "parquet",
    ];

    let lower = text.to_lowercase();
    for term in terms {
        if lower.contains(term) && !keywords.contains(&term.to_string()) {
            keywords.push(term.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_heuristics() {
        let db = load_heuristics();
        assert!(!db.all().is_empty(), "Should load heuristics from base.md");
    }

    #[test]
    fn test_search_hashmap() {
        let db = load_heuristics();
        let results = db.search(&["hashmap", "lookup"]);
        assert!(!results.is_empty(), "Should find HashMap-related heuristics");
    }

    #[test]
    fn test_categories() {
        let db = load_heuristics();
        let cats = db.categories();
        assert!(cats.len() > 5, "Should have multiple categories");
    }
}
