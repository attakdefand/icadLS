use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AnalysisResult {
    pub id: Uuid,
    pub code_sample_id: Uuid,
    pub patterns: String, // JSON string
    pub algorithms: String, // JSON string
    pub data_structures: String, // JSON string
    pub complexity: String,
    pub recommendations: String, // JSON string
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedAlgorithm {
    pub name: String,
    pub category: String,
    pub complexity: String,
    pub description: String,
    pub wikipedia_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnalysisResult {
    pub code_sample_id: Uuid,
    pub patterns: Vec<String>,
    pub algorithms: Vec<String>,
    pub detailed_algorithms: Vec<DetailedAlgorithm>,
    pub data_structures: Vec<String>,
    pub complexity: String,
    pub recommendations: Vec<String>,
}