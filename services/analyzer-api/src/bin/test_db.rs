//! Test database connectivity and functionality

use analyzer_api::db::Database;
use analyzer_api::models::{code_sample::NewCodeSample, analysis_result::{NewAnalysisResult, DetailedAlgorithm}};
use uuid::Uuid;
// Add dotenv import
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    println!("Testing database connectivity...");
    
    // Initialize database
    let db = Database::new().await?;
    db.init().await?;
    
    println!("Database initialized successfully!");
    
    // Test saving a code sample
    let new_code_sample = NewCodeSample {
        code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
        language: Some("rust".to_string()),
    };
    
    let code_sample_id = db.save_code_sample(new_code_sample).await?;
    println!("Saved code sample with ID: {}", code_sample_id);
    
    // Test saving an analysis result
    let new_analysis_result = NewAnalysisResult {
        code_sample_id,
        patterns: vec!["Hello World pattern".to_string()],
        algorithms: vec![],
        detailed_algorithms: vec![DetailedAlgorithm {
            name: "Hello World Algorithm".to_string(),
            category: "Other".to_string(),
            complexity: "O(1)".to_string(),
            description: "A simple algorithm that prints a greeting".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/%22Hello,_World!%22_program".to_string(),
        }],
        data_structures: vec![],
        complexity: "Low".to_string(),
        recommendations: vec!["Add more functionality".to_string()],
    };
    
    let analysis_result_id = db.save_analysis_result(new_analysis_result).await?;
    println!("Saved analysis result with ID: {}", analysis_result_id);
    
    println!("All tests passed!");
    Ok(())
}