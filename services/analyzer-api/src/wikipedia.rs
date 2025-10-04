//! Wikipedia scraper for algorithm information

use reqwest::Client;
use scraper::{Html, Selector};
use crate::algorithms::{AlgorithmCategory, AlgorithmInfo};
use crate::db::Database;
use crate::models::wikipedia_algorithm::WikipediaAlgorithm;

pub struct WikipediaScraper {
    client: Client,
    pub db: Database,
}

impl WikipediaScraper {
    pub fn new(db: Database) -> Self {
        Self {
            client: Client::new(),
            db,
        }
    }

    /// Fetch algorithm categories from Wikipedia
    pub async fn fetch_algorithm_categories(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = "https://en.wikipedia.org/wiki/List_of_algorithms";
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        let document = Html::parse_document(&body);

        let category_selector = Selector::parse("h2 span.mw-headline").unwrap();
        let mut categories = Vec::new();

        for element in document.select(&category_selector) {
            let category = element.text().collect::<String>();
            // Filter out non-algorithm categories
            if !category.contains("See also") && !category.contains("References") && !category.contains("External links") {
                categories.push(category);
            }
        }

        Ok(categories)
    }

    /// Fetch algorithms from a specific Wikipedia category page
    pub async fn fetch_algorithms_from_category(&self, category: &str) -> Result<Vec<WikipediaAlgorithm>, Box<dyn std::error::Error>> {
        // This is a simplified implementation - in a real-world scenario, you would parse the actual Wikipedia page
        // For now, we'll return some example algorithms
        let mut algorithms = Vec::new();

        match category {
            "Sorting algorithms" => {
                algorithms.push(WikipediaAlgorithm {
                    title: "Merge Sort".to_string(),
                    url: "https://en.wikipedia.org/wiki/Merge_sort".to_string(),
                    category: "Sorting".to_string(),
                    description: Some("An efficient, general-purpose, comparison-based sorting algorithm.".to_string()),
                    complexity: Some("O(n log n)".to_string()),
                });
                algorithms.push(WikipediaAlgorithm {
                    title: "Quick Sort".to_string(),
                    url: "https://en.wikipedia.org/wiki/Quicksort".to_string(),
                    category: "Sorting".to_string(),
                    description: Some("An efficient sorting algorithm that uses a divide-and-conquer approach.".to_string()),
                    complexity: Some("O(n log n)".to_string()),
                });
            },
            "Search algorithms" => {
                algorithms.push(WikipediaAlgorithm {
                    title: "Binary Search".to_string(),
                    url: "https://en.wikipedia.org/wiki/Binary_search_algorithm".to_string(),
                    category: "Searching".to_string(),
                    description: Some("A search algorithm that finds the position of a target value within a sorted array.".to_string()),
                    complexity: Some("O(log n)".to_string()),
                });
            },
            "Graph algorithms" => {
                algorithms.push(WikipediaAlgorithm {
                    title: "Dijkstra's Algorithm".to_string(),
                    url: "https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm".to_string(),
                    category: "Graph".to_string(),
                    description: Some("An algorithm for finding the shortest paths between nodes in a graph.".to_string()),
                    complexity: Some("O((V + E) log V)".to_string()),
                });
            },
            _ => {
                // Default case - add some generic algorithms
                algorithms.push(WikipediaAlgorithm {
                    title: "Algorithm from ".to_string() + category,
                    url: "https://en.wikipedia.org/wiki/List_of_algorithms".to_string(),
                    category: category.to_string(),
                    description: Some("An algorithm in the ".to_string() + category + " category."),
                    complexity: Some("Varies".to_string()),
                });
            }
        }

        Ok(algorithms)
    }

    /// Convert WikipediaAlgorithm to AlgorithmInfo
    pub fn convert_to_algorithm_info(&self, wiki_algo: WikipediaAlgorithm) -> AlgorithmInfo {
        // Map Wikipedia categories to our AlgorithmCategory enum
        let category = match wiki_algo.category.as_str() {
            "Sorting" => AlgorithmCategory::Sorting,
            "Searching" => AlgorithmCategory::Searching,
            "Graph" => AlgorithmCategory::Graph,
            "Dynamic Programming" => AlgorithmCategory::DynamicProgramming,
            "Greedy" => AlgorithmCategory::Greedy,
            "Backtracking" => AlgorithmCategory::Backtracking,
            "Divide and Conquer" => AlgorithmCategory::DivideConquer,
            "Mathematical" => AlgorithmCategory::Mathematical,
            "Cryptographic" => AlgorithmCategory::Cryptographic,
            "Machine Learning" => AlgorithmCategory::MachineLearning,
            "String" => AlgorithmCategory::String,
            "Tree" => AlgorithmCategory::Tree,
            "Hashing" => AlgorithmCategory::Hashing,
            _ => AlgorithmCategory::Other,
        };

        AlgorithmInfo {
            name: wiki_algo.title,
            category,
            complexity: wiki_algo.complexity.unwrap_or_else(|| "Not specified".to_string()),
            description: wiki_algo.description.unwrap_or_else(|| "No description available".to_string()),
            wikipedia_link: wiki_algo.url,
            examples: Vec::new(), // We don't have examples from Wikipedia, so we leave this empty
        }
    }

    /// Store Wikipedia algorithms in the database
    pub async fn store_wikipedia_algorithms(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let categories = self.fetch_algorithm_categories().await?;
        let mut total_stored = 0;

        for category in categories {
            let algorithms = self.fetch_algorithms_from_category(&category).await?;
            
            for wiki_algo in algorithms {
                // Convert to our internal format
                let algorithm_info = self.convert_to_algorithm_info(wiki_algo.clone());
                
                // Store the Wikipedia algorithm in the database
                match self.db.save_wikipedia_algorithm(wiki_algo).await {
                    Ok(_) => {
                        println!("Stored algorithm: {} ({})", algorithm_info.name, algorithm_info.category);
                        total_stored += 1;
                    }
                    Err(e) => {
                        eprintln!("Failed to store algorithm {}: {}", algorithm_info.name, e);
                    }
                }
            }
        }

        Ok(total_stored)
    }

    /// Update the algorithm detector with Wikipedia algorithms
    pub async fn update_algorithm_detector(&self) -> Result<usize, Box<dyn std::error::Error>> {
        // This would typically update the in-memory algorithm database
        // For now, we'll just return the count of algorithms that would be added
        let categories = self.fetch_algorithm_categories().await?;
        let mut total_added = 0;

        for category in categories {
            let algorithms = self.fetch_algorithms_from_category(&category).await?;
            total_added += algorithms.len();
        }

        Ok(total_added)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::wikipedia_algorithm::WikipediaAlgorithm;

    #[test]
    fn test_wikipedia_algorithm_creation() {
        let wiki_algo = WikipediaAlgorithm {
            title: "Test Algorithm".to_string(),
            url: "https://en.wikipedia.org/wiki/Test_Algorithm".to_string(),
            category: "Sorting".to_string(),
            description: Some("A test algorithm".to_string()),
            complexity: Some("O(n)".to_string()),
        };

        assert_eq!(wiki_algo.title, "Test Algorithm");
        assert_eq!(wiki_algo.category, "Sorting");
    }
}