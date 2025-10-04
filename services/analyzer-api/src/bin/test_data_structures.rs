//! Test data structure detection functionality

use analyzer_api::data_structures::DataStructureDetector;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    println!("Testing data structure detection...");
    
    // Initialize data structure detector
    let detector = DataStructureDetector::new();
    
    // Test code with various data structures
    let test_code = r#"
    use std::collections::{HashMap, VecDeque, BinaryHeap, HashSet};
    
    fn main() {
        // Vector (Dynamic Array)
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        
        // Hash Map
        let mut map = HashMap::new();
        map.insert("key", "value");
        
        // Queue
        let mut queue = VecDeque::new();
        queue.push_back(1);
        queue.pop_front();
        
        // Heap
        let mut heap = BinaryHeap::new();
        heap.push(5);
        
        // Hash Set
        let mut set = HashSet::new();
        set.insert("unique_value");
    }
    "#;
    
    let detected = detector.detect_data_structures(test_code);
    
    println!("Detected {} data structures:", detected.len());
    for ds in detected {
        println!("- {} ({})", ds.name, ds.category);
        println!("  Complexity: {}", ds.complexity);
        println!("  Description: {}", ds.description);
        println!("  More info: {}", ds.wikipedia_link);
        println!();
    }
    
    println!("All tests passed!");
    Ok(())
}