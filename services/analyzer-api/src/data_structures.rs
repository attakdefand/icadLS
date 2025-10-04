//! Data structure detection and classification system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod data;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DataStructureInfo {
    pub name: String,
    pub category: DataStructureCategory,
    pub complexity: String,
    pub description: String,
    pub wikipedia_link: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataStructureCategory {
    Linear,
    Tree,
    Graph,
    HashBased,
    Heap,
    Queue,
    Stack,
    Set,
    Other,
}

impl std::fmt::Display for DataStructureCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataStructureCategory::Linear => write!(f, "Linear"),
            DataStructureCategory::Tree => write!(f, "Tree"),
            DataStructureCategory::Graph => write!(f, "Graph"),
            DataStructureCategory::HashBased => write!(f, "Hash-Based"),
            DataStructureCategory::Heap => write!(f, "Heap"),
            DataStructureCategory::Queue => write!(f, "Queue"),
            DataStructureCategory::Stack => write!(f, "Stack"),
            DataStructureCategory::Set => write!(f, "Set"),
            DataStructureCategory::Other => write!(f, "Other"),
        }
    }
}

pub struct DataStructureDetector {
    pub data_structures: HashMap<String, DataStructureInfo>,
}

impl DataStructureDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            data_structures: HashMap::new(),
        };
        detector.initialize_data_structures();
        detector
    }

    fn initialize_data_structures(&mut self) {
        // Load comprehensive data structure database
        let data_structures_db = data::get_data_structures_database();
        
        for data_structure in data_structures_db {
            self.data_structures.insert(data_structure.name.clone(), data_structure);
        }
    }

    pub fn detect_data_structures(&self, code: &str) -> Vec<DetectedDataStructure> {
        let mut detected = Vec::new();
        
        for (name, info) in &self.data_structures {
            // Check for data structure patterns in the code
            if self.pattern_matches(code, info) {
                detected.push(DetectedDataStructure {
                    name: name.clone(),
                    category: info.category.clone(),
                    complexity: info.complexity.clone(),
                    description: info.description.clone(),
                    wikipedia_link: info.wikipedia_link.clone(),
                });
            }
        }
        
        detected
    }

    fn pattern_matches(&self, code: &str, data_structure_info: &DataStructureInfo) -> bool {
        // Check if any of the examples are found in the code
        for example in &data_structure_info.examples {
            if code.contains(example) || self.fuzzy_match(code, example) {
                return true;
            }
        }
        
        // Check for data structure name in the code
        if code.to_lowercase().contains(&data_structure_info.name.to_lowercase()) {
            return true;
        }
        
        // Check for category-related keywords
        let category_keywords = match data_structure_info.category {
            DataStructureCategory::Linear => vec!["array", "list", "vector"],
            DataStructureCategory::Tree => vec!["tree", "bst", "binary", "avl", "red-black"],
            DataStructureCategory::Graph => vec!["graph", "node", "edge", "vertex"],
            DataStructureCategory::HashBased => vec!["hash", "map", "dict", "table"],
            DataStructureCategory::Heap => vec!["heap", "priority"],
            DataStructureCategory::Queue => vec!["queue", "fifo"],
            DataStructureCategory::Stack => vec!["stack", "lifo"],
            DataStructureCategory::Set => vec!["set", "unique"],
            DataStructureCategory::Other => vec![],
        };
        
        for keyword in category_keywords {
            if code.to_lowercase().contains(keyword) {
                return true;
            }
        }
        
        false
    }

    fn fuzzy_match(&self, code: &str, pattern: &str) -> bool {
        // Simple fuzzy matching - could be enhanced with more sophisticated algorithms
        let code_normalized = code.replace(" ", "").replace("\n", "").replace("\t", "");
        let pattern_normalized = pattern.replace(" ", "").replace("\n", "").replace("\t", "");
        
        code_normalized.contains(&pattern_normalized)
    }
    
    pub fn get_data_structures_by_category(&self, category: DataStructureCategory) -> Vec<&DataStructureInfo> {
        self.data_structures
            .values()
            .filter(|ds| ds.category == category)
            .collect()
    }
    
    pub fn get_all_categories(&self) -> Vec<DataStructureCategory> {
        let mut categories = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for ds in self.data_structures.values() {
            if !seen.contains(&ds.category) {
                categories.push(ds.category.clone());
                seen.insert(ds.category.clone());
            }
        }
        
        categories
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedDataStructure {
    pub name: String,
    pub category: DataStructureCategory,
    pub complexity: String,
    pub description: String,
    pub wikipedia_link: String,
}