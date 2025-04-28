// Performance Optimization in Rust
// This example demonstrates various performance optimization techniques

use std::time::Instant;
use std::collections::HashMap;

// Example of a struct that can be optimized
#[derive(Clone)]
struct Data {
    values: Vec<i32>,
    cache: HashMap<i32, i32>,
}

impl Data {
    // Constructor
    fn new(values: Vec<i32>) -> Self {
        Data {
            values,
            cache: HashMap::new(),
        }
    }

    // Unoptimized version - recalculates every time
    fn sum_unoptimized(&self) -> i32 {
        self.values.iter().sum()
    }

    // Optimized version - uses caching
    fn sum_optimized(&mut self) -> i32 {
        // Check cache first
        if let Some(&sum) = self.cache.get(&0) {
            return sum;
        }

        // Calculate and cache
        let sum = self.values.iter().sum();
        self.cache.insert(0, sum);
        sum
    }

    // Example of iterator optimization
    fn find_even_unoptimized(&self) -> Option<&i32> {
        for x in &self.values {
            if x % 2 == 0 {
                return Some(x);
            }
        }
        None
    }

    // Optimized version using iterators
    fn find_even_optimized(&self) -> Option<&i32> {
        self.values.iter().find(|&&x| x % 2 == 0)
    }
}

// Example of memory optimization
struct MemoryOptimized {
    // Using Box to store large data on heap
    large_data: Box<[i32]>,
    // Using small integers for better memory alignment
    small_value: u8,
}

impl MemoryOptimized {
    fn new(data: Vec<i32>) -> Self {
        MemoryOptimized {
            large_data: data.into_boxed_slice(),
            small_value: 0,
        }
    }
}

// Example of parallel processing
fn parallel_sum(values: &[i32]) -> i32 {
    use std::thread;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    let chunk_size = values.len() / 4; // Split into 4 chunks

    for chunk in values.chunks(chunk_size) {
        let tx = tx.clone();
        let chunk = chunk.to_vec();
        
        thread::spawn(move || {
            let sum: i32 = chunk.iter().sum();
            tx.send(sum).unwrap();
        });
    }

    // Collect results
    rx.iter().take(4).sum()
}

// Example of zero-cost abstractions
trait Processor {
    fn process(&self, value: i32) -> i32;
}

struct FastProcessor;
struct SlowProcessor;

impl Processor for FastProcessor {
    fn process(&self, value: i32) -> i32 {
        value * 2
    }
}

impl Processor for SlowProcessor {
    fn process(&self, value: i32) -> i32 {
        // Simulate slow processing
        std::thread::sleep(std::time::Duration::from_millis(1));
        value * 2
    }
}

// Generic function that uses monomorphization
fn process_values<P: Processor>(processor: &P, values: &[i32]) -> Vec<i32> {
    values.iter().map(|&v| processor.process(v)).collect()
}

fn main() {
    // Benchmarking example
    println!("=== Benchmarking Examples ===");
    
    let data = Data::new(vec![1, 2, 3, 4, 5]);
    let mut optimized_data = data.clone();
    
    // Benchmark unoptimized version
    let start = Instant::now();
    let _ = data.sum_unoptimized();
    let unoptimized_time = start.elapsed();
    
    // Benchmark optimized version
    let start = Instant::now();
    let _ = optimized_data.sum_optimized();
    let optimized_time = start.elapsed();
    
    println!("Unoptimized time: {:?}", unoptimized_time);
    println!("Optimized time: {:?}", optimized_time);
    
    // Memory optimization example
    println!("\n=== Memory Optimization Example ===");
    
    let large_vec = vec![1; 1000];
    let optimized = MemoryOptimized::new(large_vec);
    println!("Size of optimized struct: {} bytes", std::mem::size_of_val(&optimized));
    
    // Parallel processing example
    println!("\n=== Parallel Processing Example ===");
    
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let parallel_result = parallel_sum(&values);
    println!("Parallel sum: {}", parallel_result);
    
    // Zero-cost abstractions example
    println!("\n=== Zero-cost Abstractions Example ===");
    
    let fast = FastProcessor;
    let slow = SlowProcessor;
    
    let values = vec![1, 2, 3, 4, 5];
    
    let start = Instant::now();
    let _ = process_values(&fast, &values);
    let fast_time = start.elapsed();
    
    let start = Instant::now();
    let _ = process_values(&slow, &values);
    let slow_time = start.elapsed();
    
    println!("Fast processor time: {:?}", fast_time);
    println!("Slow processor time: {:?}", slow_time);
} 