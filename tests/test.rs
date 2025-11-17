use heuristics::*;

#[test]
fn test_load_heuristics_not_empty() {
    let db = load_heuristics();
    assert!(!db.all().is_empty(), "Should load heuristics from base.md");
}

#[test]
fn test_search_basic() {
    let db = load_heuristics();
    let results = db.search(&["hashmap"]);
    assert!(!results.is_empty(), "Should find HashMap-related heuristics");
}

#[test]
fn test_search_multiple_keywords() {
    let db = load_heuristics();
    let results = db.search(&["hashmap", "lookup"]);
    assert!(!results.is_empty(), "Should find heuristics matching multiple keywords");
}

#[test]
fn test_search_case_insensitive() {
    let db = load_heuristics();
    let results_lower = db.search(&["hashmap"]);
    let results_upper = db.search(&["HashMap"]);
    let results_mixed = db.search(&["HashMAP"]);

    assert_eq!(results_lower.len(), results_upper.len(), "Search should be case-insensitive");
    assert_eq!(results_lower.len(), results_mixed.len(), "Search should be case-insensitive");
}

#[test]
fn test_categories_not_empty() {
    let db = load_heuristics();
    let cats = db.categories();
    assert!(!cats.is_empty(), "Should have at least one category");
}

#[test]
fn test_by_category() {
    let db = load_heuristics();
    let cats = db.categories();

    if let Some(first_category) = cats.first() {
        let results = db.by_category(first_category);
        assert!(!results.is_empty(), "Category should contain heuristics");
    }
}

#[test]
fn test_by_category_case_insensitive() {
    let db = load_heuristics();
    let cats = db.categories();

    if let Some(first_category) = cats.first() {
        let results_lower = db.by_category(&first_category.to_lowercase());
        let results_upper = db.by_category(&first_category.to_uppercase());

        assert_eq!(results_lower.len(), results_upper.len(), "Category search should be case-insensitive");
    }
}

#[test]
fn test_heuristic_structure() {
    let db = load_heuristics();
    let all = db.all();

    assert!(!all.is_empty(), "Should have heuristics loaded");

    // Check that first heuristic has required fields
    let first = &all[0];
    assert!(!first.title.is_empty(), "Heuristic should have a title");
    assert!(!first.category.is_empty(), "Heuristic should have a category");
    assert!(!first.content.is_empty(), "Heuristic should have content");
}

#[test]
fn test_search_ranking() {
    let db = load_heuristics();

    // Search with a common term
    let results = db.search(&["hash"]);

    // Results should be ordered by relevance (score)
    // We can't test exact order, but we can verify results are returned
    assert!(!results.is_empty(), "Should find results for common term");
}

#[test]
fn test_search_no_results() {
    let db = load_heuristics();

    // Search for something unlikely to exist
    let results = db.search(&["zzzzzznonexistent"]);

    // Should return empty vector, not panic
    assert!(results.is_empty(), "Should handle searches with no results gracefully");
}

#[test]
fn test_categories_sorted_and_unique() {
    let db = load_heuristics();
    let cats = db.categories();

    // Check that categories are sorted
    for i in 0..cats.len().saturating_sub(1) {
        assert!(cats[i] <= cats[i + 1], "Categories should be sorted");
    }

    // Check that categories are unique
    for i in 0..cats.len().saturating_sub(1) {
        assert_ne!(cats[i], cats[i + 1], "Categories should be unique");
    }
}

#[test]
fn test_all_returns_all_heuristics() {
    let db = load_heuristics();
    let all = db.all();

    // Count heuristics from all categories
    let cats = db.categories();
    let mut total_from_categories = 0;
    for cat in &cats {
        total_from_categories += db.by_category(cat).len();
    }

    assert_eq!(all.len(), total_from_categories, "all() should return all heuristics");
}
