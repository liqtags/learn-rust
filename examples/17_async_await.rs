// Async/Await in Rust
// This example demonstrates how to use async/await for concurrent programming

use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

// Async function that simulates a long-running operation
async fn fetch_data(id: i32) -> String {
    // Simulate network delay
    sleep(Duration::from_millis(100)).await;
    format!("Data from source {}", id)
}

// Async function that processes data
async fn process_data(data: String) -> String {
    // Simulate processing time
    sleep(Duration::from_millis(50)).await;
    format!("Processed: {}", data)
}

// Async function that demonstrates error handling
async fn fetch_with_retry(id: i32, retries: u32) -> Result<String, String> {
    for attempt in 0..retries {
        match fetch_data(id).await {
            Ok(data) => return Ok(data),
            Err(_) if attempt < retries - 1 => {
                sleep(Duration::from_millis(100 * (attempt + 1) as u64)).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err("Max retries exceeded".to_string())
}

// Async function that demonstrates parallel execution
async fn fetch_all_data() -> Vec<String> {
    let mut handles = vec![];
    
    // Spawn multiple tasks
    for i in 1..=5 {
        let handle = tokio::spawn(async move {
            fetch_data(i).await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    
    results
}

// Async function that demonstrates channels
async fn channel_example() {
    let (tx, mut rx) = mpsc::channel(32);
    
    // Spawn sender task
    let sender = tokio::spawn(async move {
        for i in 1..=5 {
            let data = fetch_data(i).await;
            tx.send(data).await.unwrap();
        }
    });
    
    // Spawn receiver task
    let receiver = tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            let processed = process_data(data).await;
            println!("Received: {}", processed);
        }
    });
    
    // Wait for both tasks to complete
    let _ = tokio::join!(sender, receiver);
}

// Async function that demonstrates shared state
async fn shared_state_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // Spawn multiple tasks that modify shared state
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            let mut count = counter.lock().await;
            *count += 1;
            println!("Task {} incremented counter to {}", i, *count);
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

// Async function that demonstrates select
async fn select_example() {
    let mut data1 = fetch_data(1).await;
    let mut data2 = fetch_data(2).await;
    
    tokio::select! {
        result1 = process_data(data1) => {
            println!("First data processed: {}", result1);
        }
        result2 = process_data(data2) => {
            println!("Second data processed: {}", result2);
        }
    }
}

// Main async function
#[tokio::main]
async fn main() {
    println!("=== Basic Async Example ===");
    let data = fetch_data(1).await;
    println!("{}", data);
    
    println!("\n=== Parallel Execution Example ===");
    let results = fetch_all_data().await;
    println!("All data: {:?}", results);
    
    println!("\n=== Channel Example ===");
    channel_example().await;
    
    println!("\n=== Shared State Example ===");
    shared_state_example().await;
    
    println!("\n=== Select Example ===");
    select_example().await;
    
    println!("\n=== Error Handling Example ===");
    match fetch_with_retry(1, 3).await {
        Ok(data) => println!("Success: {}", data),
        Err(e) => println!("Error: {}", e),
    }
} 