// Quick sort implementation
fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(arr);
    
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() - 1;
    let mut i = 0;
    
    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, pivot_index);
    i
}

fn main() {
    let mut data = vec![64, 34, 25, 12, 22, 11, 90];
    
    println!("Original array: {:?}", data);
    
    quick_sort(&mut data);
    
    println!("Sorted array: {:?}", data);
}