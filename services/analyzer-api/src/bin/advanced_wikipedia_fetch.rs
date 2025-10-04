//! Advanced Wikipedia algorithm fetcher

use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AlgorithmEntry {
    name: String,
    url: String,
    category: String,
    description: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Advanced Wikipedia algorithm fetcher");
    println!("====================================");

    let client = Client::new();
    
    // Fetch the main list of algorithms page
    let url = "https://en.wikipedia.org/wiki/List_of_algorithms";
    println!("Fetching: {}", url);
    
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);

    // Parse the page to extract algorithm categories and links
    let mut algorithms: HashMap<String, Vec<AlgorithmEntry>> = HashMap::new();
    
    // Look for section headings (h2 elements with mw-headline class)
    let heading_selector = Selector::parse("h2 span.mw-headline").unwrap();
    // Note: link_selector is not used in the current implementation
    // let link_selector = Selector::parse("a[href]").unwrap();
    
    let mut current_category = String::new();
    
    // This is a simplified approach - in a real implementation, you'd want to be more sophisticated
    // about parsing the document structure
    for element in document.select(&heading_selector) {
        let category = element.text().collect::<String>();
        if !category.is_empty() && 
           !category.contains("See also") && 
           !category.contains("References") && 
           !category.contains("External links") &&
           !category.contains("Notes") {
            current_category = category;
            algorithms.insert(current_category.clone(), Vec::new());
            println!("Found category: {}", current_category);
        }
    }
    
    // For demonstration, let's manually add some well-known algorithms
    add_sample_algorithms(&mut algorithms);
    
    // Display results
    println!("\nFound algorithms by category:");
    for (category, algos) in &algorithms {
        println!("\n{}:", category);
        for algo in algos {
            println!("  - {} ({})", algo.name, algo.url);
        }
    }
    
    // Count total algorithms
    let total: usize = algorithms.values().map(|v| v.len()).sum();
    println!("\nTotal algorithms found: {}", total);
    
    Ok(())
}

fn add_sample_algorithms(algorithms: &mut HashMap<String, Vec<AlgorithmEntry>>) {
    // Add some sample algorithms to demonstrate the structure
    let sorting_algorithms = vec![
        AlgorithmEntry {
            name: "Bubble Sort".to_string(),
            url: "https://en.wikipedia.org/wiki/Bubble_sort".to_string(),
            category: "Sorting".to_string(),
            description: Some("A simple sorting algorithm that repeatedly steps through the list.".to_string()),
        },
        AlgorithmEntry {
            name: "Quick Sort".to_string(),
            url: "https://en.wikipedia.org/wiki/Quicksort".to_string(),
            category: "Sorting".to_string(),
            description: Some("An efficient sorting algorithm that uses a divide-and-conquer approach.".to_string()),
        },
        AlgorithmEntry {
            name: "Merge Sort".to_string(),
            url: "https://en.wikipedia.org/wiki/Merge_sort".to_string(),
            category: "Sorting".to_string(),
            description: Some("A divide-and-conquer algorithm that divides the array into halves.".to_string()),
        },
    ];
    
    algorithms.insert("Sorting algorithms".to_string(), sorting_algorithms);
    
    let search_algorithms = vec![
        AlgorithmEntry {
            name: "Binary Search".to_string(),
            url: "https://en.wikipedia.org/wiki/Binary_search_algorithm".to_string(),
            category: "Search".to_string(),
            description: Some("A search algorithm that finds the position of a target value.".to_string()),
        },
    ];
    
    algorithms.insert("Search algorithms".to_string(), search_algorithms);
    
    let graph_algorithms = vec![
        AlgorithmEntry {
            name: "Dijkstra's Algorithm".to_string(),
            url: "https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm".to_string(),
            category: "Graph".to_string(),
            description: Some("An algorithm for finding the shortest paths between nodes in a graph.".to_string()),
        },
        AlgorithmEntry {
            name: "A* Search Algorithm".to_string(),
            url: "https://en.wikipedia.org/wiki/A*_search_algorithm".to_string(),
            category: "Graph".to_string(),
            description: Some("A graph traversal and path search algorithm.".to_string()),
        },
    ];
    
    algorithms.insert("Graph algorithms".to_string(), graph_algorithms);
}