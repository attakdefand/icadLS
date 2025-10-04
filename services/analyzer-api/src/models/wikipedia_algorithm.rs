use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikipediaAlgorithm {
    pub title: String,
    pub url: String,
    pub category: String,
    pub description: Option<String>,
    pub complexity: Option<String>,
}