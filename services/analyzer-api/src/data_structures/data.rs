//! Comprehensive data structure database

use super::{DataStructureInfo, DataStructureCategory};

/// Returns a comprehensive list of data structures with detailed information
pub fn get_data_structures_database() -> Vec<DataStructureInfo> {
    vec![
        // Linear Data Structures
        DataStructureInfo {
            name: "Array".to_string(),
            category: DataStructureCategory::Linear,
            complexity: "O(1) access, O(n) search".to_string(),
            description: "A collection of elements identified by array index or key, stored in contiguous memory locations.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Array_data_structure".to_string(),
            examples: vec![
                "let arr = [1, 2, 3, 4, 5];".to_string(),
                "arr[0] = 10;".to_string(),
            ],
        },
        DataStructureInfo {
            name: "Linked List".to_string(),
            category: DataStructureCategory::Linear,
            complexity: "O(n) access, O(1) insertion/deletion".to_string(),
            description: "A linear collection of data elements whose order is not given by their physical placement in memory. Instead, each element points to the next.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Linked_list".to_string(),
            examples: vec![
                "struct ListNode { val: i32, next: Option<Box<ListNode>> }".to_string(),
                "list.next = Some(Box::new(ListNode::new(val)));".to_string(),
            ],
        },
        DataStructureInfo {
            name: "Dynamic Array".to_string(),
            category: DataStructureCategory::Linear,
            complexity: "O(1) amortized insertion, O(1) access".to_string(),
            description: "A resizable array that allows elements to be added or removed, automatically managing its size.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Dynamic_array".to_string(),
            examples: vec![
                "let mut vec = Vec::new();".to_string(),
                "vec.push(1);".to_string(),
                "vec![1, 2, 3];".to_string(),
            ],
        },

        // Tree Data Structures
        DataStructureInfo {
            name: "Binary Search Tree".to_string(),
            category: DataStructureCategory::Tree,
            complexity: "O(log n) average, O(n) worst case".to_string(),
            description: "A tree data structure where each node has at most two children, and the left subtree contains only nodes with keys less than the node's key.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Binary_search_tree".to_string(),
            examples: vec![
                "struct BST { root: Option<Box<Node>> }".to_string(),
                "fn insert(&mut self, value: i32) { /* recursive insertion */ }".to_string(),
            ],
        },
        DataStructureInfo {
            name: "AVL Tree".to_string(),
            category: DataStructureCategory::Tree,
            complexity: "O(log n) for all operations".to_string(),
            description: "A self-balancing binary search tree where the difference between heights of left and right subtrees cannot be more than one for all nodes.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/AVL_tree".to_string(),
            examples: vec![
                "struct AVLNode { value: i32, height: i32, left: Option<Box<AVLNode>>, right: Option<Box<AVLNode>> }".to_string(),
            ],
        },

        // Hash-Based Data Structures
        DataStructureInfo {
            name: "Hash Table".to_string(),
            category: DataStructureCategory::HashBased,
            complexity: "O(1) average case for search/insert/delete".to_string(),
            description: "A data structure that implements an associative array, mapping keys to values using a hash function.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Hash_table".to_string(),
            examples: vec![
                "use std::collections::HashMap;".to_string(),
                "let mut map = HashMap::new();".to_string(),
                "map.insert(key, value);".to_string(),
            ],
        },
        DataStructureInfo {
            name: "HashSet".to_string(),
            category: DataStructureCategory::HashBased,
            complexity: "O(1) average case for insert/contains/remove".to_string(),
            description: "A set implementation that uses a hash table for storage, providing fast lookup times.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Set_(abstract_data_type)".to_string(),
            examples: vec![
                "use std::collections::HashSet;".to_string(),
                "let mut set = HashSet::new();".to_string(),
                "set.insert(value);".to_string(),
            ],
        },

        // Heap Data Structures
        DataStructureInfo {
            name: "Binary Heap".to_string(),
            category: DataStructureCategory::Heap,
            complexity: "O(1) find max/min, O(log n) insertion/deletion".to_string(),
            description: "A complete binary tree that satisfies the heap property, where parents are compared to their children.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Binary_heap".to_string(),
            examples: vec![
                "use std::collections::BinaryHeap;".to_string(),
                "let mut heap = BinaryHeap::new();".to_string(),
                "heap.push(value);".to_string(),
            ],
        },

        // Queue Data Structures
        DataStructureInfo {
            name: "Queue".to_string(),
            category: DataStructureCategory::Queue,
            complexity: "O(1) enqueue/dequeue".to_string(),
            description: "A collection in which elements are added at one end (rear) and removed from the other end (front), following FIFO (First In First Out) principle.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Queue_(abstract_data_type)".to_string(),
            examples: vec![
                "use std::collections::VecDeque;".to_string(),
                "let mut queue = VecDeque::new();".to_string(),
                "queue.push_back(value);".to_string(),
                "queue.pop_front();".to_string(),
            ],
        },

        // Stack Data Structures
        DataStructureInfo {
            name: "Stack".to_string(),
            category: DataStructureCategory::Stack,
            complexity: "O(1) push/pop".to_string(),
            description: "A collection that follows the LIFO (Last In First Out) principle, where the last element added is the first one to be removed.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Stack_(abstract_data_type)".to_string(),
            examples: vec![
                "let mut stack = Vec::new();".to_string(),
                "stack.push(value);".to_string(),
                "stack.pop();".to_string(),
            ],
        },

        // Graph Data Structures
        DataStructureInfo {
            name: "Graph".to_string(),
            category: DataStructureCategory::Graph,
            complexity: "Varies by implementation".to_string(),
            description: "A collection of nodes (vertices) and edges connecting some pairs of nodes, representing relationships between objects.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Graph_(abstract_data_type)".to_string(),
            examples: vec![
                "struct Graph { vertices: Vec<Node>, edges: Vec<Edge> }".to_string(),
                "adjacency_list: HashMap<Node, Vec<Node>>".to_string(),
            ],
        },
    ]
}