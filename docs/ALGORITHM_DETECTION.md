# Algorithm Detection in ICALDS

## Overview

ICALDS uses pattern matching and static analysis techniques to detect algorithms in source code. The system can identify over 20 algorithms across 14 categories.

## Supported Algorithms

### Sorting Algorithms

1. **Bubble Sort**
   - Time Complexity: O(n²)
   - Space Complexity: O(1)
   - Detection Pattern: Nested loops with adjacent element comparisons and swaps

2. **Quick Sort**
   - Time Complexity: O(n log n) average, O(n²) worst case
   - Space Complexity: O(log n)
   - Detection Pattern: Divide-and-conquer with partitioning

3. **Merge Sort**
   - Time Complexity: O(n log n)
   - Space Complexity: O(n)
   - Detection Pattern: Divide-and-conquer with merging step

4. **Insertion Sort**
   - Time Complexity: O(n²)
   - Space Complexity: O(1)
   - Detection Pattern: Shifting elements to insert in sorted position

5. **Selection Sort**
   - Time Complexity: O(n²)
   - Space Complexity: O(1)
   - Detection Pattern: Finding minimum element and swapping

### Searching Algorithms

1. **Binary Search**
   - Time Complexity: O(log n)
   - Space Complexity: O(1)
   - Detection Pattern: Divide search space in half based on comparison

2. **Linear Search**
   - Time Complexity: O(n)
   - Space Complexity: O(1)
   - Detection Pattern: Sequential iteration through elements

### Graph Algorithms

1. **Depth-First Search (DFS)**
   - Time Complexity: O(V + E)
   - Space Complexity: O(V)
   - Detection Pattern: Recursive traversal or stack-based exploration

2. **Breadth-First Search (BFS)**
   - Time Complexity: O(V + E)
   - Space Complexity: O(V)
   - Detection Pattern: Queue-based level-by-level traversal

3. **Dijkstra's Algorithm**
   - Time Complexity: O((V + E) log V)
   - Space Complexity: O(V)
   - Detection Pattern: Priority queue with distance updates

### Dynamic Programming

1. **Fibonacci Sequence**
   - Time Complexity: O(n)
   - Space Complexity: O(1)
   - Detection Pattern: Overlapping subproblems with memoization

2. **Longest Common Subsequence**
   - Time Complexity: O(mn)
   - Space Complexity: O(mn)
   - Detection Pattern: 2D table filling with optimal substructure

### Tree Algorithms

1. **Binary Tree Traversal**
   - Time Complexity: O(n)
   - Space Complexity: O(h) where h is height
   - Detection Patterns:
     - Inorder: Left, Root, Right
     - Preorder: Root, Left, Right
     - Postorder: Left, Right, Root

### String Algorithms

1. **String Matching**
   - Time Complexity: O(nm) for naive approach
   - Space Complexity: O(1)
   - Detection Pattern: Nested loops comparing characters

## Detection Process

1. **Parsing**: Source code is parsed into an Abstract Syntax Tree (AST)
2. **Pattern Matching**: AST is analyzed for known algorithm patterns
3. **Validation**: Detected patterns are validated against known implementations
4. **Classification**: Algorithms are categorized and complexity is determined
5. **Enrichment**: Educational content is added from Wikipedia and other sources

## Data Structure Detection

In addition to algorithms, ICALDS detects common data structures:

- **Vectors/Arrays**: Dynamic arrays with indexing
- **Linked Lists**: Node-based sequential structures
- **Hash Maps**: Key-value storage with hash-based lookup
- **Trees**: Hierarchical node structures
- **Graphs**: Node and edge-based structures
- **Stacks**: LIFO (Last In, First Out) structures
- **Queues**: FIFO (First In, First Out) structures

## Accuracy and Limitations

### Accuracy

The current detection accuracy is approximately 90% for common algorithms when implemented in standard patterns.

### Limitations

1. **Obfuscated Code**: Heavily obfuscated implementations may not be detected
2. **Non-standard Implementations**: Variants that deviate significantly from standard patterns
3. **Language Support**: Currently focused on Rust, with plans to expand to other languages
4. **Complex Algorithms**: Very complex algorithms may require more sophisticated analysis

## Extending Detection Capabilities

### Adding New Algorithms

To add support for new algorithms:

1. Define the detection pattern in the analysis engine
2. Add complexity analysis
3. Include educational content
4. Create test cases
5. Update documentation

### Improving Accuracy

Accuracy can be improved by:

1. Adding more sophisticated pattern matching
2. Implementing machine learning-based detection
3. Expanding the test corpus
4. Incorporating community feedback

## Future Enhancements

Planned improvements include:

- Support for additional programming languages
- Machine learning-based detection
- Real-time code analysis in IDEs
- Integration with popular development tools
- Enhanced educational content
- Performance benchmarking