# Algorithm Detection System

This document describes the algorithm detection system implemented in ICALDS.

## Wikipedia Integration

The system includes integration with Wikipedia to fetch and store algorithm information. This allows the system to maintain an up-to-date database of algorithms with accurate information from the authoritative source.

### Features

1. **Automatic Fetching**: The system can automatically fetch algorithm information from Wikipedia
2. **Database Storage**: All fetched information is stored in the MySQL database for persistent access
3. **Category Organization**: Algorithms are organized by categories as defined on Wikipedia
4. **Regular Updates**: The system can be configured to periodically update algorithm information

### Database Schema

The Wikipedia algorithms are stored in a dedicated table with the following structure:

```sql
CREATE TABLE wikipedia_algorithms (
    id CHAR(36) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    url VARCHAR(512) NOT NULL,
    category VARCHAR(100) NOT NULL,
    description TEXT,
    complexity VARCHAR(50),
    fetched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
```

### Usage

To fetch and store Wikipedia algorithms:

```bash
cd services/analyzer-api
cargo run --bin fetch_wikipedia_algorithms
```

This will:
1. Connect to the configured MySQL database
2. Fetch algorithm information from Wikipedia
3. Store the information in the wikipedia_algorithms table
4. Display a summary of fetched algorithms

## Overview

The algorithm detection system identifies and classifies algorithms in source code based on pattern matching and keyword analysis. It provides detailed information about detected algorithms including complexity analysis, descriptions, and educational resources.

## Algorithm Categories

The system classifies algorithms into the following categories:

1. **Sorting** - Algorithms for arranging data in a particular order
2. **Searching** - Algorithms for finding specific elements in data structures
3. **Graph** - Algorithms for traversing and analyzing graph structures
4. **Dynamic Programming** - Algorithms that solve complex problems by breaking them into simpler subproblems
5. **Greedy** - Algorithms that make locally optimal choices at each step
6. **Backtracking** - Algorithms that incrementally build candidates and abandon invalid ones
7. **Divide and Conquer** - Algorithms that recursively break problems into smaller subproblems
8. **Mathematical** - Algorithms for mathematical computations
9. **Cryptographic** - Algorithms for encryption, decryption, and security
10. **Machine Learning** - Algorithms for pattern recognition and prediction
11. **String** - Algorithms for string manipulation and pattern matching
12. **Tree** - Algorithms for tree data structure operations
13. **Hashing** - Algorithms for hash-based data structures
14. **Other** - Miscellaneous algorithms

## Detection Methods

The system uses multiple approaches to detect algorithms:

1. **Pattern Matching** - Looks for specific code patterns associated with known algorithms
2. **Keyword Analysis** - Identifies algorithms based on relevant keywords in the code
3. **Name Recognition** - Detects algorithms by their names mentioned in the code

## Supported Algorithms

The system currently supports detection of the following algorithms:

### Sorting Algorithms
- Bubble Sort
- Quick Sort
- Merge Sort
- Heap Sort
- Insertion Sort

### Searching Algorithms
- Binary Search
- Linear Search

### Graph Algorithms
- Dijkstra's Algorithm
- Depth-First Search (DFS)
- Breadth-First Search (BFS)
- A* Search Algorithm

### Dynamic Programming
- Fibonacci Sequence
- Longest Common Subsequence

### Greedy Algorithms
- Kruskal's Algorithm

### Mathematical Algorithms
- Euclidean Algorithm
- Sieve of Eratosthenes

### String Algorithms
- KMP Algorithm
- Rabin-Karp Algorithm

### Tree Algorithms
- Binary Search Tree

### Hashing Algorithms
- Hash Table

## Detailed Information

For each detected algorithm, the system provides:

1. **Name** - The algorithm's name
2. **Category** - Classification of the algorithm
3. **Complexity** - Time and space complexity analysis
4. **Description** - Detailed explanation of the algorithm
5. **Wikipedia Link** - Educational resource for further learning

## Extending the System

To add support for new algorithms:

1. Add the algorithm information to `src/algorithms/data.rs`
2. Include representative code patterns in the examples
3. Specify the appropriate category
4. Provide accurate complexity analysis
5. Include a valid Wikipedia link

## API Response Format

The API returns analysis results in the following format:

```json
{
  "patterns": ["Range-based loop", "Recursive function"],
  "algorithms": ["Bubble Sort"],
  "detailed_algorithms": [
    {
      "name": "Bubble Sort",
      "category": "Sorting",
      "complexity": "O(n²)",
      "description": "A simple sorting algorithm that repeatedly steps through the list...",
      "wikipedia_link": "https://en.wikipedia.org/wiki/Bubble_sort"
    }
  ],
  "data_structures": ["Dynamic array"],
  "complexity": "Low",
  "recommendations": [
    "Add documentation comments",
    "Learn more about Bubble Sort at https://en.wikipedia.org/wiki/Bubble_sort"
  ]
}
```

## Web Interface

The web interface displays detailed algorithm information in a user-friendly format, including:
- Algorithm names and categories
- Complexity analysis
- Descriptions
- Direct links to Wikipedia articles

This allows users to learn more about the algorithms detected in their code.