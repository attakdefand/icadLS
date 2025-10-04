// Binary search implementation
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    
    None
}

fn main() {
    let sorted_array = [1, 3, 5, 7, 9, 11, 13, 15];
    let target = 7;
    
    match binary_search(&sorted_array, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}