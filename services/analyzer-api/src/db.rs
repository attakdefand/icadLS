use sqlx::{MySql, Pool, MySqlPool};
use std::env;
use crate::models::{analysis_result::NewAnalysisResult, code_sample::NewCodeSample, wikipedia_algorithm::WikipediaAlgorithm};
use uuid::Uuid;
use chrono::Utc;

pub struct Database {
    pool: Pool<MySql>,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@localhost:3306/icalds".to_string());
        
        let pool = MySqlPool::connect(&database_url).await?;
        Ok(Self { pool })
    }
    
    pub fn pool(&self) -> &Pool<MySql> {
        &self.pool
    }
    
    pub async fn init(&self) -> Result<(), sqlx::Error> {
        // Create tables if they don't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS code_samples (
                id CHAR(36) PRIMARY KEY,
                code TEXT NOT NULL,
                language VARCHAR(50),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS analysis_results (
                id CHAR(36) PRIMARY KEY,
                code_sample_id CHAR(36) NOT NULL,
                patterns JSON,
                algorithms JSON,
                detailed_algorithms JSON,
                data_structures JSON,
                complexity VARCHAR(50),
                recommendations JSON,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (code_sample_id) REFERENCES code_samples(id)
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS wikipedia_algorithms (
                id CHAR(36) PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                url VARCHAR(512) NOT NULL,
                category VARCHAR(100) NOT NULL,
                description TEXT,
                complexity VARCHAR(50),
                fetched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn save_code_sample(&self, code_sample: NewCodeSample) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO code_samples (id, code, language, created_at) VALUES (?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(code_sample.code)
        .bind(code_sample.language)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        
        Ok(id)
    }
    
    pub async fn save_analysis_result(&self, analysis_result: NewAnalysisResult) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        
        // Convert Vec<String> to JSON strings
        let patterns_json = serde_json::to_string(&analysis_result.patterns).unwrap_or("[]".to_string());
        let algorithms_json = serde_json::to_string(&analysis_result.algorithms).unwrap_or("[]".to_string());
        let detailed_algorithms_json = serde_json::to_string(&analysis_result.detailed_algorithms).unwrap_or("[]".to_string());
        let data_structures_json = serde_json::to_string(&analysis_result.data_structures).unwrap_or("[]".to_string());
        let recommendations_json = serde_json::to_string(&analysis_result.recommendations).unwrap_or("[]".to_string());
        
        sqlx::query(
            "INSERT INTO analysis_results (id, code_sample_id, patterns, algorithms, detailed_algorithms, data_structures, complexity, recommendations, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(analysis_result.code_sample_id.to_string())
        .bind(patterns_json)
        .bind(algorithms_json)
        .bind(detailed_algorithms_json)
        .bind(data_structures_json)
        .bind(analysis_result.complexity)
        .bind(recommendations_json)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        
        Ok(id)
    }
    
    pub async fn save_wikipedia_algorithm(&self, wiki_algo: WikipediaAlgorithm) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO wikipedia_algorithms (id, title, url, category, description, complexity, fetched_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(&wiki_algo.title)
        .bind(&wiki_algo.url)
        .bind(&wiki_algo.category)
        .bind(&wiki_algo.description)
        .bind(&wiki_algo.complexity)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        
        Ok(id)
    }
    
    pub async fn get_all_wikipedia_algorithms(&self) -> Result<Vec<WikipediaAlgorithm>, sqlx::Error> {
        let algorithms = sqlx::query_as::<_, WikipediaAlgorithmRow>(
            "SELECT title, url, category, description, complexity FROM wikipedia_algorithms ORDER BY title"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(algorithms.into_iter().map(|row| row.into()).collect())
    }
}

// Helper struct for database queries
#[derive(sqlx::FromRow)]
struct WikipediaAlgorithmRow {
    title: String,
    url: String,
    category: String,
    description: Option<String>,
    complexity: Option<String>,
}

impl From<WikipediaAlgorithmRow> for WikipediaAlgorithm {
    fn from(row: WikipediaAlgorithmRow) -> Self {
        WikipediaAlgorithm {
            title: row.title,
            url: row.url,
            category: row.category,
            description: row.description,
            complexity: row.complexity,
        }
    }
}