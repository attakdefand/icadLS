//! Comprehensive algorithm database

use super::{AlgorithmInfo, AlgorithmCategory};

/// Returns a comprehensive list of algorithms with detailed information
pub fn get_algorithms_database() -> Vec<AlgorithmInfo> {
    vec![
        // Sorting Algorithms
        AlgorithmInfo {
            name: "Bubble Sort".to_string(),
            category: AlgorithmCategory::Sorting,
            complexity: "O(n²)".to_string(),
            description: "A simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Bubble_sort".to_string(),
            examples: vec![
                "for i in 0..arr.len() { for j in 0..arr.len() - 1 - i { if arr[j] > arr[j + 1] { /* swap */ } } }".to_string(),
                "while swapped { swapped = false; for i in 1..n { if arr[i-1] > arr[i] { /* swap */ } } }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Quick Sort".to_string(),
            category: AlgorithmCategory::Sorting,
            complexity: "O(n log n)".to_string(),
            description: "An efficient sorting algorithm that uses a divide-and-conquer approach to sort elements by selecting a 'pivot' element and partitioning the array around it.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Quicksort".to_string(),
            examples: vec![
                "fn quicksort(arr: &mut [i32]) { if arr.len() <= 1 { return; } let pivot = partition(arr); /* recursive calls */ }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Merge Sort".to_string(),
            category: AlgorithmCategory::Sorting,
            complexity: "O(n log n)".to_string(),
            description: "A divide-and-conquer algorithm that divides the array into halves, sorts them recursively, and then merges the sorted halves.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Merge_sort".to_string(),
            examples: vec![
                "fn merge_sort(arr: &mut [i32]) { if arr.len() <= 1 { return; } let mid = arr.len() / 2; /* recursive calls and merge */ }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Heap Sort".to_string(),
            category: AlgorithmCategory::Sorting,
            complexity: "O(n log n)".to_string(),
            description: "A comparison-based sorting algorithm that uses a binary heap data structure to create a sorted array.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Heapsort".to_string(),
            examples: vec![
                "fn heap_sort(arr: &mut [i32]) { let len = arr.len(); /* build heap */ for i in (0..len/2).rev() { heapify(arr, len, i); } /* extract elements */ }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Insertion Sort".to_string(),
            category: AlgorithmCategory::Sorting,
            complexity: "O(n²)".to_string(),
            description: "A simple sorting algorithm that builds the final sorted array one item at a time by comparing each element with the previous elements and inserting it into its correct position.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Insertion_sort".to_string(),
            examples: vec![
                "fn insertion_sort(arr: &mut [i32]) { for i in 1..arr.len() { let key = arr[i]; let mut j = i; while j > 0 && arr[j-1] > key { arr[j] = arr[j-1]; j -= 1; } arr[j] = key; } }".to_string(),
            ],
        },

        // Searching Algorithms
        AlgorithmInfo {
            name: "Binary Search".to_string(),
            category: AlgorithmCategory::Searching,
            complexity: "O(log n)".to_string(),
            description: "A search algorithm that finds the position of a target value within a sorted array by repeatedly dividing the search interval in half.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Binary_search_algorithm".to_string(),
            examples: vec![
                "fn binary_search(arr: &[i32], target: i32) -> Option<usize> { let mut left = 0; let mut right = arr.len(); while left < right { /* midpoint calculation */ } }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Linear Search".to_string(),
            category: AlgorithmCategory::Searching,
            complexity: "O(n)".to_string(),
            description: "A simple search algorithm that checks every element in the list until it finds the target value or reaches the end of the list.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Linear_search".to_string(),
            examples: vec![
                "fn linear_search(arr: &[i32], target: i32) -> Option<usize> { for (i, &item) in arr.iter().enumerate() { if item == target { return Some(i); } } None }".to_string(),
            ],
        },

        // Graph Algorithms
        AlgorithmInfo {
            name: "Dijkstra's Algorithm".to_string(),
            category: AlgorithmCategory::Graph,
            complexity: "O((V + E) log V)".to_string(),
            description: "An algorithm for finding the shortest paths between nodes in a graph with non-negative edge weights.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm".to_string(),
            examples: vec![
                "fn dijkstra(graph: &Graph, start: Node) -> HashMap<Node, Distance> { let mut distances = HashMap::new(); /* priority queue operations */ }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Depth-First Search".to_string(),
            category: AlgorithmCategory::Graph,
            complexity: "O(V + E)".to_string(),
            description: "An algorithm for traversing or searching tree or graph data structures by exploring as far as possible along each branch before backtracking.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Depth-first_search".to_string(),
            examples: vec![
                "fn dfs(graph: &Graph, node: Node, visited: &mut HashSet<Node>) { visited.insert(node); for neighbor in graph.neighbors(node) { if !visited.contains(&neighbor) { dfs(graph, neighbor, visited); } } }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Breadth-First Search".to_string(),
            category: AlgorithmCategory::Graph,
            complexity: "O(V + E)".to_string(),
            description: "An algorithm for traversing or searching tree or graph data structures that explores all the vertices at the present depth level before moving on to vertices at the next depth level.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Breadth-first_search".to_string(),
            examples: vec![
                "fn bfs(graph: &Graph, start: Node) -> Vec<Node> { let mut visited = HashSet::new(); let mut queue = VecDeque::new(); /* queue operations */ }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "A* Search Algorithm".to_string(),
            category: AlgorithmCategory::Graph,
            complexity: "O(b^d)".to_string(),
            description: "A graph traversal and path search algorithm that uses a heuristic to estimate the cost of the cheapest path from a given node to the goal.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/A*_search_algorithm".to_string(),
            examples: vec![
                "fn a_star(graph: &Graph, start: Node, goal: Node) -> Option<Path> { let mut open_set = BinaryHeap::new(); /* heuristic function */ }".to_string(),
            ],
        },

        // Dynamic Programming
        AlgorithmInfo {
            name: "Fibonacci Sequence".to_string(),
            category: AlgorithmCategory::DynamicProgramming,
            complexity: "O(n)".to_string(),
            description: "A sequence where each number is the sum of the two preceding ones, often implemented with dynamic programming to avoid redundant calculations.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Fibonacci_number".to_string(),
            examples: vec![
                "fn fibonacci(n: usize) -> u64 { let mut memo = vec![0; n+1]; memo[0] = 0; memo[1] = 1; for i in 2..=n { memo[i] = memo[i-1] + memo[i-2]; } memo[n] }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Longest Common Subsequence".to_string(),
            category: AlgorithmCategory::DynamicProgramming,
            complexity: "O(m*n)".to_string(),
            description: "A dynamic programming algorithm that finds the longest subsequence common to two sequences.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Longest_common_subsequence_problem".to_string(),
            examples: vec![
                "fn lcs(str1: &str, str2: &str) -> usize { let m = str1.len(); let n = str2.len(); let mut dp = vec![vec![0; n+1]; m+1]; /* fill dp table */ }".to_string(),
            ],
        },

        // Greedy Algorithms
        AlgorithmInfo {
            name: "Dijkstra's Algorithm".to_string(),
            category: AlgorithmCategory::Greedy,
            complexity: "O((V + E) log V)".to_string(),
            description: "A greedy algorithm for finding the shortest paths between nodes in a graph with non-negative edge weights.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm".to_string(),
            examples: vec![
                "fn dijkstra(graph: &Graph, start: Node) -> HashMap<Node, Distance> { let mut distances = HashMap::new(); /* priority queue operations */ }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Kruskal's Algorithm".to_string(),
            category: AlgorithmCategory::Greedy,
            complexity: "O(E log E)".to_string(),
            description: "A minimum spanning tree algorithm that finds an edge of the least possible weight that connects any two trees in the forest.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Kruskal%27s_algorithm".to_string(),
            examples: vec![
                "fn kruskal(edges: &mut [Edge]) -> Vec<Edge> { edges.sort_by(|a, b| a.weight.cmp(&b.weight)); /* union-find operations */ }".to_string(),
            ],
        },

        // Mathematical Algorithms
        AlgorithmInfo {
            name: "Euclidean Algorithm".to_string(),
            category: AlgorithmCategory::Mathematical,
            complexity: "O(log min(a,b))".to_string(),
            description: "An efficient method for computing the greatest common divisor (GCD) of two numbers.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Euclidean_algorithm".to_string(),
            examples: vec![
                "fn gcd(a: u32, b: u32) -> u32 { if b == 0 { a } else { gcd(b, a % b) } }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Sieve of Eratosthenes".to_string(),
            category: AlgorithmCategory::Mathematical,
            complexity: "O(n log log n)".to_string(),
            description: "An ancient algorithm for finding all prime numbers up to any given limit.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes".to_string(),
            examples: vec![
                "fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> { let mut is_prime = vec![true; limit + 1]; is_prime[0] = false; is_prime[1] = false; /* marking multiples */ }".to_string(),
            ],
        },

        // String Algorithms
        AlgorithmInfo {
            name: "KMP Algorithm".to_string(),
            category: AlgorithmCategory::String,
            complexity: "O(n + m)".to_string(),
            description: "A string searching algorithm that searches for occurrences of a pattern within a main text string by employing the observation that when a mismatch occurs, the word itself embodies sufficient information to determine where the next match could begin.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm".to_string(),
            examples: vec![
                "fn kmp_search(text: &str, pattern: &str) -> Vec<usize> { let lps = compute_lps(pattern); /* matching logic */ }".to_string(),
            ],
        },
        AlgorithmInfo {
            name: "Rabin-Karp Algorithm".to_string(),
            category: AlgorithmCategory::String,
            complexity: "O(n+m)".to_string(),
            description: "A string searching algorithm that uses hashing to find any one of a set of pattern strings in a text.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm".to_string(),
            examples: vec![
                "fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> { let pattern_hash = hash(pattern); /* rolling hash */ }".to_string(),
            ],
        },

        // Tree Algorithms
        AlgorithmInfo {
            name: "Binary Search Tree".to_string(),
            category: AlgorithmCategory::Tree,
            complexity: "O(log n)".to_string(),
            description: "A node-based binary tree data structure that has the following properties: the left subtree of a node contains only nodes with keys lesser than the node's key, and the right subtree contains only nodes with keys greater than the node's key.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Binary_search_tree".to_string(),
            examples: vec![
                "struct BST { root: Option<Box<Node>> } impl BST { fn insert(&mut self, value: i32) { /* recursive insertion */ } }".to_string(),
            ],
        },

        // Hashing Algorithms
        AlgorithmInfo {
            name: "Hash Table".to_string(),
            category: AlgorithmCategory::Hashing,
            complexity: "O(1)".to_string(),
            description: "A data structure that implements an associative array, a structure that can map keys to values using a hash function to compute an index into an array of buckets or slots.".to_string(),
            wikipedia_link: "https://en.wikipedia.org/wiki/Hash_table".to_string(),
            examples: vec![
                "struct HashTable { buckets: Vec<Vec<(K, V)>> } impl HashTable { fn insert(&mut self, key: K, value: V) { /* hash function */ } }".to_string(),
            ],
        },
    ]
}