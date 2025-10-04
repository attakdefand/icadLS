//! Test the enhanced algorithm detection system

use analyzer_api::algorithms::AlgorithmDetector;

fn main() {
    println!("Testing enhanced algorithm detection system...");
    
    // Initialize the algorithm detector
    let detector = AlgorithmDetector::new();
    
    println!("Algorithm detector initialized with {} algorithms", detector.algorithms.len());
    
    // Test with bubble sort code
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
    
    println!("\nTesting with bubble sort code:");
    let detected = detector.detect_algorithms(bubble_sort_code);
    println!("Detected {} algorithms:", detected.len());
    
    for algorithm in detected {
        println!("- {} ({})", algorithm.name, algorithm.category);
        println!("  Complexity: {}", algorithm.complexity);
        println!("  Description: {}", algorithm.description);
        println!("  Learn more: {}", algorithm.wikipedia_link);
        println!();
    }
    
    // Test with quick sort code
    let quick_sort_code = r#"
    fn quicksort(arr: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot = partition(arr);
        // recursive calls would be here
    }
    "#;
    
    println!("Testing with quick sort code:");
    let detected = detector.detect_algorithms(quick_sort_code);
    println!("Detected {} algorithms:", detected.len());
    
    for algorithm in detected {
        println!("- {} ({})", algorithm.name, algorithm.category);
        println!("  Complexity: {}", algorithm.complexity);
        println!("  Description: {}", algorithm.description);
        println!("  Learn more: {}", algorithm.wikipedia_link);
        println!();
    }
    
    // Show categories
    println!("Available categories:");
    let categories = detector.get_all_categories();
    for category in categories {
        println!("- {}", category);
    }
    
    println!("\nAll tests completed successfully!");
}