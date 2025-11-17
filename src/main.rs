use clap::{Parser, Subcommand};
use colored::*;
use heuristics::{load_heuristics, Heuristic};

#[derive(Parser)]
#[command(name = "heuristics")]
#[command(about = "Search computer science and Rust development heuristics", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for heuristics by keywords
    Search {
        /// Keywords to search for
        keywords: Vec<String>,

        /// Maximum number of results to show
        #[arg(short, long, default_value = "5")]
        limit: usize,
    },

    /// List all categories
    Categories,

    /// Show all heuristics in a category
    Category {
        /// Category name
        name: String,
    },

    /// List all heuristics
    List,
}

fn main() {
    let cli = Cli::parse();
    let db = load_heuristics();

    match cli.command {
        Commands::Search { keywords, limit } => {
            let keyword_refs: Vec<&str> = keywords.iter().map(|s| s.as_str()).collect();
            let results = db.search(&keyword_refs);

            if results.is_empty() {
                println!("{}", "No heuristics found matching your keywords.".yellow());
                println!("\nTry broader terms like: hash, cache, distributed, concurrent, etc.");
                return;
            }

            println!("{}", format!("Found {} heuristic(s):\n", results.len()).green().bold());

            for (i, heuristic) in results.iter().take(limit).enumerate() {
                print_heuristic(heuristic, i + 1);
            }

            if results.len() > limit {
                println!(
                    "\n{}",
                    format!("... and {} more. Use --limit to show more results.", results.len() - limit)
                        .dimmed()
                );
            }
        }

        Commands::Categories => {
            let categories = db.categories();
            println!("{}\n", "Available categories:".green().bold());

            for cat in categories {
                println!("  • {}", cat.cyan());
            }

            println!("\n{}", "Use 'heuristics category <name>' to see heuristics in a category.".dimmed());
        }

        Commands::Category { name } => {
            let results = db.by_category(&name);

            if results.is_empty() {
                println!("{}", format!("No category found: {}", name).red());
                println!("\nUse 'heuristics categories' to see available categories.");
                return;
            }

            println!("{}\n", format!("Heuristics in category '{}':", name).green().bold());

            for (i, heuristic) in results.iter().enumerate() {
                print_heuristic(heuristic, i + 1);
            }
        }

        Commands::List => {
            let all = db.all();
            println!("{}\n", format!("All {} heuristics:", all.len()).green().bold());

            for (i, heuristic) in all.iter().enumerate() {
                println!("{}. {} ({})",
                    format!("{:3}", i + 1).dimmed(),
                    heuristic.title.cyan(),
                    heuristic.category.yellow()
                );
            }
        }
    }
}

fn print_heuristic(heuristic: &Heuristic, index: usize) {
    println!("{}", format!("{}. {}", index, heuristic.title).cyan().bold());

    if !heuristic.action.is_empty() {
        println!("   {} {}", "Action:".green().bold(), heuristic.action);
    }

    if !heuristic.crates.is_empty() {
        println!("   {} {}",
            "Crates:".green().bold(),
            heuristic.crates.join(", ").yellow()
        );
    }

    if !heuristic.std_types.is_empty() {
        println!("   {} {}",
            "Std types:".green().bold(),
            heuristic.std_types.join(", ").yellow()
        );
    }

    println!("   {} {}", "Category:".green().bold(), heuristic.category.dimmed());
    println!();
}
