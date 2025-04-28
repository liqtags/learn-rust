// Collections in Rust
// This example demonstrates various collection types and their common operations

use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::collections::hash_map::Entry;

fn main() {
    // Vector (Vec<T>) - A growable array
    println!("\n=== Vector Examples ===");
    
    // Creating vectors
    let mut vec1 = Vec::new();  // Empty vector
    let vec2 = vec![1, 2, 3];  // Vector with initial values
    
    // Adding elements
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    
    // Accessing elements
    println!("First element: {}", vec1[0]);  // Panics if out of bounds
    println!("First element (safe): {:?}", vec1.get(0));  // Returns Option
    
    // Iterating
    for x in &vec1 {
        print!("{} ", x);
    }
    println!();
    
    // Vector operations
    vec1.pop();  // Remove last element
    vec1.insert(1, 4);  // Insert at index
    vec1.remove(0);  // Remove at index
    vec1.sort();  // Sort in place
    
    // HashMap (HashMap<K, V>) - A key-value store
    println!("\n=== HashMap Examples ===");
    
    let mut scores = HashMap::new();
    
    // Inserting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue team score: {:?}", score);
    
    // Updating values
    scores.insert(String::from("Blue"), 25);  // Overwrite
    scores.entry(String::from("Yellow")).or_insert(50);  // Insert if not exists
    
    // Iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // HashSet (HashSet<T>) - A set of unique values
    println!("\n=== HashSet Examples ===");
    
    let mut fruits = HashSet::new();
    
    // Adding values
    fruits.insert("apple");
    fruits.insert("banana");
    fruits.insert("orange");
    
    // Checking membership
    println!("Contains apple: {}", fruits.contains("apple"));
    
    // Set operations
    let mut other_fruits = HashSet::new();
    other_fruits.insert("banana");
    other_fruits.insert("mango");
    
    // Union
    let union: HashSet<_> = fruits.union(&other_fruits).collect();
    println!("Union: {:?}", union);
    
    // Intersection
    let intersection: HashSet<_> = fruits.intersection(&other_fruits).collect();
    println!("Intersection: {:?}", intersection);
    
    // VecDeque (VecDeque<T>) - A double-ended queue
    println!("\n=== VecDeque Examples ===");
    
    let mut queue = VecDeque::new();
    
    // Adding elements
    queue.push_back(1);
    queue.push_back(2);
    queue.push_front(0);
    
    // Removing elements
    println!("Front: {:?}", queue.pop_front());
    println!("Back: {:?}", queue.pop_back());
    
    // BinaryHeap (BinaryHeap<T>) - A priority queue
    println!("\n=== BinaryHeap Examples ===");
    
    let mut heap = BinaryHeap::new();
    
    // Adding elements
    heap.push(1);
    heap.push(5);
    heap.push(2);
    
    // Removing elements (always returns the largest)
    println!("Largest: {:?}", heap.pop());
    println!("Next largest: {:?}", heap.pop());
    
    // Common collection operations
    println!("\n=== Common Operations ===");
    
    // Filtering
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);
    
    // Mapping
    let doubled: Vec<_> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // Folding/Reducing
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    
    // Collecting into different types
    let word_count: HashMap<_, _> = vec!["hello", "world", "hello"]
        .iter()
        .map(|&s| (s, 1))
        .collect();
    println!("Word count: {:?}", word_count);
} 