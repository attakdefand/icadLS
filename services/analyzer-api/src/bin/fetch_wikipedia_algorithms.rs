//! Fetch algorithms from Wikipedia and store them in the database

use analyzer_api::{db::Database, wikipedia::WikipediaScraper};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching algorithms from Wikipedia...");

    // Initialize database
    let db = Database::new().await?;
    db.init().await?;

    // Initialize Wikipedia scraper
    let scraper = WikipediaScraper::new(db);

    // Fetch and store Wikipedia algorithms
    match scraper.store_wikipedia_algorithms().await {
        Ok(count) => {
            println!("Successfully fetched and processed {} algorithms from Wikipedia", count);
        }
        Err(e) => {
            eprintln!("Error fetching Wikipedia algorithms: {}", e);
        }
    }

    // Show what's in the database
    match scraper.db.get_all_wikipedia_algorithms().await {
        Ok(algorithms) => {
            println!("\nAlgorithms stored in database:");
            for algorithm in algorithms {
                println!("- {} ({})", algorithm.title, algorithm.category);
            }
        }
        Err(e) => {
            eprintln!("Error fetching algorithms from database: {}", e);
        }
    }

    println!("\nWikipedia algorithm fetching completed!");
    Ok(())
}