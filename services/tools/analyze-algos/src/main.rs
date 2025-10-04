use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the file to analyze
    #[arg(short = 'f', long)]
    file: Option<String>,

    /// Code to analyze as a string
    #[arg(short = 'c', long)]
    code: Option<String>,

    /// Output format (json or text)
    #[arg(short = 'o', long, default_value = "text")]
    format: String,
}

#[derive(Serialize, Deserialize)]
struct AnalysisResult {
    patterns: Vec<String>,
    algorithms: Vec<String>,
    data_structures: Vec<String>,
    complexity: String,
    recommendations: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let code = if let Some(file_path) = cli.file {
        match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {}: {}", file_path, e);
                return;
            }
        }
    } else if let Some(code) = cli.code {
        code
    } else {
        eprintln!("Either --file or --code must be provided");
        return;
    };

    let result = analyze_code(&code);

    if cli.format == "json" {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        print_analysis_result(&result);
    }
}

fn analyze_code(code: &str) -> AnalysisResult {
    let mut patterns = Vec::new();
    let mut algorithms = Vec::new();
    let mut data_structures = Vec::new();
    let mut recommendations = Vec::new();
    
    // Simple pattern matching for demonstration
    if code.contains("for") && code.contains("..<") {
        patterns.push("Range-based loop".to_string());
    }
    
    if code.contains("fn ") && code.contains("recursive") {
        patterns.push("Recursive function".to_string());
    }
    
    if code.contains("sort") {
        algorithms.push("Sorting algorithm".to_string());
    }
    
    if code.contains("Vec") || code.contains("vec!") {
        data_structures.push("Dynamic array".to_string());
    }
    
    if code.contains("HashMap") {
        data_structures.push("Hash map".to_string());
    }
    
    if code.contains("LinkedList") {
        data_structures.push("Linked list".to_string());
    }
    
    // Simple complexity estimation
    let complexity = if code.lines().count() > 100 {
        "High".to_string()
    } else if code.lines().count() > 50 {
        "Medium".to_string()
    } else {
        "Low".to_string()
    };
    
    // Basic recommendations
    if code.lines().count() > 100 {
        recommendations.push("Consider breaking this into smaller functions".to_string());
    }
    
    if !code.contains("///") && !code.contains("//") {
        recommendations.push("Add documentation comments".to_string());
    }
    
    AnalysisResult {
        patterns,
        algorithms,
        data_structures,
        complexity,
        recommendations,
    }
}

fn print_analysis_result(result: &AnalysisResult) {
    println!("=== Code Analysis Results ===");
    println!("Complexity: {}", result.complexity);
    
    if !result.patterns.is_empty() {
        println!("\nPatterns detected:");
        for pattern in &result.patterns {
            println!("  - {}", pattern);
        }
    }
    
    if !result.algorithms.is_empty() {
        println!("\nAlgorithms detected:");
        for algorithm in &result.algorithms {
            println!("  - {}", algorithm);
        }
    }
    
    if !result.data_structures.is_empty() {
        println!("\nData structures detected:");
        for ds in &result.data_structures {
            println!("  - {}", ds);
        }
    }
    
    if !result.recommendations.is_empty() {
        println!("\nRecommendations:");
        for recommendation in &result.recommendations {
            println!("  - {}", recommendation);
        }
    }
}