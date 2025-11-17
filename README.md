<h1 align="center">
  <br>
  Heuristics
  <br>
</h1>

<h4 align="center">
  A searchable collection of computer science and Rust development heuristics.
</h4>

<p align="center">
  <a href="https://crates.io/crates/heuristics" target="_blank">
    <img src="https://img.shields.io/crates/v/heuristics" alt="Crates.io"/>
  </a>
  <a href="https://crates.io/crates/heuristics" target="_blank">
    <img src="https://img.shields.io/crates/d/heuristics" alt="Downloads"/>
  </a>
  <a href="https://docs.rs/heuristics" target="_blank">
    <img src="https://docs.rs/heuristics/badge.svg" alt="Documentation"/>
  </a>
  <a href="LICENSE" target="_blank">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"/>
  </a>
</p>

<b>Author's bio:</b> 👋😀 Hi, I'm CryptoPatrick! I'm currently enrolled as an
Undergraduate student in Mathematics, at Chalmers & the University of Gothenburg, Sweden. <br>
If you have any questions or need more info, then please <a href="https://discord.gg/T8EWmJZpCB">join my Discord Channel: AiMath</a>

---

<p align="center">
  <a href="#-what-is-heuristics">What is heuristics</a> •
  <a href="#-features">Features</a> •
  <a href="#-how-to-use">How To Use</a> •
  <a href="#-documentation">Documentation</a> •
  <a href="#-license">License</a>
</p>

## 🛎 Important Notices
* **Curated Knowledge**: Compiled rules of thumb for choosing the right data structures, algorithms, and patterns
* **Searchable Database**: Quickly find relevant heuristics by keywords, crates, or categories
* **Rust-Focused**: Specific recommendations for Rust crates and standard library types

<!-- TABLE OF CONTENTS -->
<h2 id="table-of-contents"> :pushpin: Table of Contents</h2>

<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#-what-is-heuristics">What is heuristics</a></li>
    <li><a href="#-features">Features</a></li>
      <ul>
        <li><a href="#-core-functionality">Core Functionality</a></li>
        <li><a href="#-searchable-database">Searchable Database</a></li>
        <li><a href="#-categorization">Categorization</a></li>
        <li><a href="#-rust-specific">Rust-Specific</a></li>
      </ul>
    <li><a href="#-how-to-use">How to Use</a></li>
    <li><a href="#-testing">Testing</a></li>
    <li><a href="#-documentation">Documentation</a></li>
    <li><a href="#-health-status">Health Status</a></li>
    <li><a href="#-license">License</a></li>
  </ol>
</details>

## 🤔 What is heuristics

`heuristics` is a comprehensive Rust library that provides a searchable collection of computer science and Rust development heuristics. It helps developers make informed decisions about data structures, algorithms, and architectural patterns by providing curated rules of thumb with concrete recommendations.

The library enables quick discovery of the right tool for the job, whether you need O(1) lookups, persistent storage, concurrent data structures, or specialized algorithms.

### Use Cases

- **Architecture Decisions**: Quickly find the right data structure or pattern for your use case
- **Performance Optimization**: Discover performance-oriented alternatives to common patterns
- **Learning Resource**: Understand when to use different Rust crates and standard library types
- **Code Review**: Reference best practices and implementation patterns
- **Rapid Prototyping**: Get up to speed quickly with recommended crates for specific scenarios

## 📷 Features

`heuristics` provides a fast, searchable database of development heuristics with rich metadata and multiple access patterns.

### 🔧 Core Functionality
- **Heuristic Loading**: Parse and load heuristics from markdown format
- **Keyword Search**: Find relevant heuristics by searching keywords
- **Category Browsing**: Explore heuristics by category
- **Rich Metadata**: Access titles, actions, crates, standard types, and keywords

### 🛠 Searchable Database
- **Inverted Index**: Fast keyword lookup using inverted index
- **Relevance Ranking**: Results ranked by number of keyword matches
- **Partial Matching**: Finds results even with partial keyword matches
- **Case-Insensitive**: Search works regardless of case

### 📊 Categorization
- **Organized Topics**: Heuristics grouped into logical categories
- **Multiple Categories**: Performance, concurrency, persistence, specialized data structures, and more
- **Easy Navigation**: Browse all heuristics within a specific category
- **Category Listing**: Get all available categories

### 🔤 Rust-Specific
- **Crate Recommendations**: Specific Rust crates for each use case
- **Standard Library**: Links to relevant std types (HashMap, Vec, etc.)
- **Code Examples**: Practical examples in Rust
- **Best Practices**: Rust-specific implementation patterns

## 🚙 How to Use

### Installation

Add `heuristics` to your `Cargo.toml`:

```toml
[dependencies]
heuristics = "0.1"
```

Or install with cargo:

```bash
cargo add heuristics
```

### Basic Example

```rust
use heuristics::load_heuristics;

fn main() {
    // Load the heuristics database
    let db = load_heuristics();

    // Search for heuristics
    let results = db.search(&["hashmap", "performance"]);
    for heuristic in results.iter().take(5) {
        println!("{}", heuristic.title);
        println!("Action: {}", heuristic.action);
        if !heuristic.crates.is_empty() {
            println!("Recommended crates: {}", heuristic.crates.join(", "));
        }
    }

    // Browse by category
    let categories = db.categories();
    println!("Available categories: {:?}", categories);

    // Get heuristics in a specific category
    let concurrent = db.by_category("Concurrency & Lock-Free Heuristics");
    for heuristic in concurrent {
        println!("• {}", heuristic.title);
    }

    // Get all heuristics
    println!("Total heuristics: {}", db.all().len());
}
```

### Advanced Usage

```rust
use heuristics::*;

fn main() {
    let db = load_heuristics();

    // Search with multiple keywords for better ranking
    let results = db.search(&["concurrent", "lock-free", "atomic"]);

    for (i, h) in results.iter().enumerate() {
        println!("\n{}. {}", i + 1, h.title);
        println!("   Category: {}", h.category);
        println!("   Action: {}", h.action);

        // Show related crates
        if !h.crates.is_empty() {
            println!("   Crates:");
            for crate_name in &h.crates {
                println!("     - {}", crate_name);
            }
        }

        // Show standard library types
        if !h.std_types.is_empty() {
            println!("   Std types: {}", h.std_types.join(", "));
        }

        // Access full markdown content
        // println!("\n{}", h.content);
    }

    // Find heuristics for a specific technology
    let results = db.search(&["sled"]);
    if !results.is_empty() {
        println!("\nHeuristics mentioning 'sled':");
        for h in results {
            println!("  • {} ({})", h.title, h.category);
        }
    }
}
```

### Command-Line Interface

The crate also includes a CLI tool:

```bash
# Install the binary
cargo install heuristics

# Search for heuristics
heuristics search hashmap lookup

# List all categories
heuristics categories

# Get heuristics in a category
heuristics category "General-Purpose Performance Heuristics"
```

## 🧪 Testing

The test suite includes comprehensive coverage of search, categorization, and data structure functionality.

```bash
# Run all tests
cargo test

# Run library tests only
cargo test --lib

# Run integration tests
cargo test --test tests

# Run with output
cargo test -- --nocapture
```

Test coverage includes:
- Loading heuristics from base.md
- Keyword search (basic, multi-keyword, case-insensitive)
- Category listing and filtering
- Data structure validation
- Search ranking
- Edge cases (no results, empty searches)

## 📚 Documentation

Comprehensive documentation is available at [docs.rs/heuristics](https://docs.rs/heuristics), including:
- API reference for all public types and functions
- Examples of searching and browsing heuristics
- Heuristic data structure details
- Performance considerations and indexing strategy

## 🖊 Author

<a href="https://x.com/cryptopatrick">CryptoPatrick</a>

Keybase Verification:
https://keybase.io/cryptopatrick/sigs/8epNh5h2FtIX1UNNmf8YQ-k33M8J-Md4LnAN

## 🐣 Support
Leave a ⭐ if you think this project is cool.

## 🗄 License

This project is licensed under MIT. See [LICENSE](LICENSE) for details.
