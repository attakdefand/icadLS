use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware::Logger};
use actix_cors::Cors; // Add CORS import
use serde::{Deserialize, Serialize};
use std::sync::Arc;
// Add dotenv import
use dotenv::dotenv;

mod db;
mod models;
mod algorithms;
mod data_structures;

#[derive(Serialize, Deserialize)]
struct CodeAnalysis {
    code: String,
}

#[derive(Serialize)]
struct AnalysisResult {
    patterns: Vec<String>,
    algorithms: Vec<String>,
    detailed_algorithms: Vec<algorithms::DetectedAlgorithm>,
    data_structures: Vec<String>,
    detailed_data_structures: Vec<data_structures::DetectedDataStructure>,
    complexity: String,
    recommendations: Vec<String>,
}

#[derive(Serialize)]
struct HealthCheck {
    status: String,
    version: String,
}

// Application state with database connection
struct AppState {
    db: Arc<db::Database>,
    algorithm_detector: Arc<algorithms::AlgorithmDetector>,
    data_structure_detector: Arc<data_structures::DataStructureDetector>,
}

async fn health_check() -> Result<HttpResponse> {
    let health = HealthCheck {
        status: "healthy".to_string(),
        version: "0.1.0".to_string(),
    };
    Ok(HttpResponse::Ok().json(health))
}

async fn analyze_code(
    item: web::Json<CodeAnalysis>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    // This is a simplified analysis - in a real implementation, 
    // this would contain actual logic to analyze code
    let result = analyze_code_logic(&item.code, &data.algorithm_detector, &data.data_structure_detector);
    
    // Save code sample to database
    let new_code_sample = models::code_sample::NewCodeSample {
        code: item.code.clone(),
        language: Some("rust".to_string()),
    };
    
    match data.db.save_code_sample(new_code_sample).await {
        Ok(code_sample_id) => {
            // Convert detailed algorithms to the database model
            let detailed_algorithms: Vec<models::analysis_result::DetailedAlgorithm> = result
                .detailed_algorithms
                .iter()
                .map(|alg| models::analysis_result::DetailedAlgorithm {
                    name: alg.name.clone(),
                    category: alg.category.to_string(),
                    complexity: alg.complexity.clone(),
                    description: alg.description.clone(),
                    wikipedia_link: alg.wikipedia_link.clone(),
                })
                .collect();
            
            // Save analysis result to database
            let new_analysis_result = models::analysis_result::NewAnalysisResult {
                code_sample_id,
                patterns: result.patterns.clone(),
                algorithms: result.algorithms.clone(),
                detailed_algorithms,
                data_structures: result.data_structures.clone(),
                complexity: result.complexity.clone(),
                recommendations: result.recommendations.clone(),
            };
            
            match data.db.save_analysis_result(new_analysis_result).await {
                Ok(_) => {
                    println!("Saved analysis result to database");
                }
                Err(e) => {
                    eprintln!("Failed to save analysis result: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to save code sample: {}", e);
        }
    }
    
    Ok(HttpResponse::Ok().json(result))
}

fn analyze_code_logic(
    code: &str, 
    algorithm_detector: &algorithms::AlgorithmDetector,
    data_structure_detector: &data_structures::DataStructureDetector
) -> AnalysisResult {
    let mut patterns = Vec::new();
    let mut algorithms = Vec::new();
    let mut data_structures = Vec::new();
    let mut recommendations = Vec::new();
    
    // Detect algorithms using the new system
    let detected_algorithms = algorithm_detector.detect_algorithms(code);
    
    // Extract algorithm names for the simple list
    for alg in &detected_algorithms {
        algorithms.push(alg.name.clone());
    }
    
    // Detect data structures
    let detected_data_structures = data_structure_detector.detect_data_structures(code);
    
    // Extract data structure names for the simple list
    for ds in &detected_data_structures {
        data_structures.push(ds.name.clone());
    }
    
    // Simple pattern matching for demonstration
    if code.contains("for") && code.contains("..<") {
        patterns.push("Range-based loop".to_string());
    }
    
    if code.contains("fn ") && code.contains("recursive") {
        patterns.push("Recursive function".to_string());
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
    
    // Add educational recommendations based on detected algorithms
    for alg in &detected_algorithms {
        recommendations.push(format!("Learn more about {} at {}", alg.name, alg.wikipedia_link));
    }
    
    // Add educational recommendations based on detected data structures
    for ds in &detected_data_structures {
        recommendations.push(format!("Learn more about {} at {}", ds.name, ds.wikipedia_link));
    }
    
    AnalysisResult {
        patterns,
        algorithms,
        detailed_algorithms: detected_algorithms,
        data_structures,
        detailed_data_structures: detected_data_structures,
        complexity,
        recommendations,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    env_logger::init();
    
    // Initialize database
    let db = db::Database::new().await.expect("Failed to connect to database");
    db.init().await.expect("Failed to initialize database");
    
    // Initialize algorithm detector
    let algorithm_detector = algorithms::AlgorithmDetector::new();
    
    // Initialize data structure detector
    let data_structure_detector = data_structures::DataStructureDetector::new();
    
    let app_state = web::Data::new(AppState {
        db: Arc::new(db),
        algorithm_detector: Arc::new(algorithm_detector),
        data_structure_detector: Arc::new(data_structure_detector),
    });
    
    println!("Starting analyzer API server at http://localhost:8081");
    
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        
        App::new()
            .app_data(app_state.clone())
            .wrap(cors) // Add CORS middleware
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/analyze", web::post().to(analyze_code))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}