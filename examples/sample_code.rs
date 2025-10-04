// Sample code for testing the analyzer
fn main() {
    // This is a sample implementation with various patterns
    let mut data = vec![5, 2, 8, 1, 9];
    
    // Bubble sort implementation
    for i in 0..data.len() {
        for j in 0..data.len() - 1 - i {
            if data[j] > data[j + 1] {
                let temp = data[j];
                data[j] = data[j + 1];
                data[j + 1] = temp;
            }
        }
    }
    
    // Print sorted data
    for i in 0..<data.len() {
        println!("Value at index {}: {}", i, data[i]);
    }
    
    // Using HashMap
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("first", data[0]);
    map.insert("last", data[data.len() - 1]);
    
    println!("Map: {:?}", map);
}