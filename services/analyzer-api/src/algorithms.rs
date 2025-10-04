//! Algorithm detection and classification system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod data;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AlgorithmInfo {
    pub name: String,
    pub category: AlgorithmCategory,
    pub complexity: String,
    pub description: String,
    pub wikipedia_link: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AlgorithmCategory {
    Sorting,
    Searching,
    Graph,
    DynamicProgramming,
    Greedy,
    Backtracking,
    DivideConquer,
    Mathematical,
    Cryptographic,
    MachineLearning,
    String,
    Tree,
    Hashing,
    Other,
}

impl std::fmt::Display for AlgorithmCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgorithmCategory::Sorting => write!(f, "Sorting"),
            AlgorithmCategory::Searching => write!(f, "Searching"),
            AlgorithmCategory::Graph => write!(f, "Graph"),
            AlgorithmCategory::DynamicProgramming => write!(f, "Dynamic Programming"),
            AlgorithmCategory::Greedy => write!(f, "Greedy"),
            AlgorithmCategory::Backtracking => write!(f, "Backtracking"),
            AlgorithmCategory::DivideConquer => write!(f, "Divide and Conquer"),
            AlgorithmCategory::Mathematical => write!(f, "Mathematical"),
            AlgorithmCategory::Cryptographic => write!(f, "Cryptographic"),
            AlgorithmCategory::MachineLearning => write!(f, "Machine Learning"),
            AlgorithmCategory::String => write!(f, "String"),
            AlgorithmCategory::Tree => write!(f, "Tree"),
            AlgorithmCategory::Hashing => write!(f, "Hashing"),
            AlgorithmCategory::Other => write!(f, "Other"),
        }
    }
}

pub struct AlgorithmDetector {
    pub algorithms: HashMap<String, AlgorithmInfo>,
}

impl AlgorithmDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            algorithms: HashMap::new(),
        };
        detector.initialize_algorithms();
        detector
    }

    fn initialize_algorithms(&mut self) {
        // Load comprehensive algorithm database
        let algorithms_db = data::get_algorithms_database();
        
        for algorithm in algorithms_db {
            self.algorithms.insert(algorithm.name.clone(), algorithm);
        }
    }

    pub fn detect_algorithms(&self, code: &str) -> Vec<DetectedAlgorithm> {
        let mut detected = Vec::new();
        
        for (name, info) in &self.algorithms {
            // Check for algorithm patterns in the code
            if self.pattern_matches(code, info) {
                detected.push(DetectedAlgorithm {
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

    fn pattern_matches(&self, code: &str, algorithm_info: &AlgorithmInfo) -> bool {
        // Check if any of the examples are found in the code
        for example in &algorithm_info.examples {
            if code.contains(example) || self.fuzzy_match(code, example) {
                return true;
            }
        }
        
        // Check for algorithm name in the code
        if code.to_lowercase().contains(&algorithm_info.name.to_lowercase()) {
            return true;
        }
        
        // Check for category-related keywords
        let category_keywords = match algorithm_info.category {
            AlgorithmCategory::Sorting => vec!["sort", "sorted", "ordering"],
            AlgorithmCategory::Searching => vec!["search", "find", "lookup"],
            AlgorithmCategory::Graph => vec!["graph", "node", "edge", "vertex"],
            AlgorithmCategory::DynamicProgramming => vec!["dp", "memo", "subproblem"],
            AlgorithmCategory::Greedy => vec!["greedy", "optimal", "choice"],
            AlgorithmCategory::Backtracking => vec!["backtrack", "recurse", "prune"],
            AlgorithmCategory::DivideConquer => vec!["divide", "conquer", "merge"],
            AlgorithmCategory::Mathematical => vec!["math", "prime", "gcd", "lcm"],
            AlgorithmCategory::Cryptographic => vec!["encrypt", "decrypt", "hash", "cipher"],
            AlgorithmCategory::MachineLearning => vec!["train", "predict", "model", "neural"],
            AlgorithmCategory::String => vec!["string", "substring", "pattern"],
            AlgorithmCategory::Tree => vec!["tree", "bst", "binary", "traversal"],
            AlgorithmCategory::Hashing => vec!["hash", "map", "dict", "table"],
            AlgorithmCategory::Other => vec![],
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
    
    pub fn get_algorithms_by_category(&self, category: AlgorithmCategory) -> Vec<&AlgorithmInfo> {
        self.algorithms
            .values()
            .filter(|algo| algo.category == category)
            .collect()
    }
    
    pub fn get_all_categories(&self) -> Vec<AlgorithmCategory> {
        let mut categories = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for algo in self.algorithms.values() {
            if !seen.contains(&algo.category) {
                categories.push(algo.category.clone());
                seen.insert(algo.category.clone());
            }
        }
        
        categories
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedAlgorithm {
    pub name: String,
    pub category: AlgorithmCategory,
    pub complexity: String,
    pub description: String,
    pub wikipedia_link: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_detection() {
        let detector = AlgorithmDetector::new();
        
        let bubble_sort_code = r#"
        fn bubble_sort(arr: &mut [i32]) {
            let len = arr.len();
            for i in 0..len {
                for j in 0..len - 1 - i {
                    if arr[j] > arr[j + 1] {
                        arr.swap(j, j + 1);
                    }
                }
            }
        }
        "#;
        
        let detected = detector.detect_algorithms(bubble_sort_code);
        assert!(!detected.is_empty());
        
        let bubble_sort_detected = detected.iter().any(|alg| alg.name == "Bubble Sort");
        assert!(bubble_sort_detected);
    }
    
    #[test]
    fn test_category_filtering() {
        let detector = AlgorithmDetector::new();
        let sorting_algorithms = detector.get_algorithms_by_category(AlgorithmCategory::Sorting);
        assert!(!sorting_algorithms.is_empty());
    }
}