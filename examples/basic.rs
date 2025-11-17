use heuristics::load_heuristics;

fn main() {
    // Load the heuristics database
    let db = load_heuristics();

    println!("--- Heuristics Library Example ---\n");

    // Search for heuristics
    println!("Searching for 'hashmap' and 'performance':");
    let results = db.search(&["hashmap", "performance"]);
    for (i, heuristic) in results.iter().take(3).enumerate() {
        println!("\n{}. {}", i + 1, heuristic.title);
        println!("   Action: {}", heuristic.action);
        if !heuristic.crates.is_empty() {
            println!("   Crates: {}", heuristic.crates.join(", "));
        }
    }

    // List categories
    println!("\n\n--- Available Categories ---");
    for category in db.categories() {
        let count = db.by_category(&category).len();
        println!("  • {} ({} heuristics)", category, count);
    }

    // Get heuristics by category
    println!("\n\n--- Concurrency & Lock-Free Heuristics ---");
    let concurrent = db.by_category("Concurrency & Lock-Free Heuristics");
    for heuristic in concurrent.iter().take(2) {
        println!("\n  • {}", heuristic.title);
        println!("    → {}", heuristic.action);
    }

    println!("\n\nTotal heuristics loaded: {}", db.all().len());
}
