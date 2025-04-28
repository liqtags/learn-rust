// Concurrency in Rust
// This example demonstrates threads, message passing, and shared state

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::atomic::{AtomicUsize, Ordering};

// Function that runs in a separate thread
fn thread_function(id: u32) {
    for i in 1..=5 {
        println!("Thread {}: Count {}", id, i);
        thread::sleep(Duration::from_millis(100));
    }
}

// Function that demonstrates message passing between threads
fn message_passing_example() {
    // Create a channel for message passing
    let (tx, rx) = mpsc::channel();
    
    // Spawn a thread that sends messages
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive messages in the main thread
    for received in rx {
        println!("Got: {}", received);
    }
}

// Function that demonstrates shared state with Mutex
fn shared_state_example() {
    // Create a shared counter protected by a Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // Spawn 10 threads that increment the counter
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter to {}", i, *num);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
}

// Function that demonstrates atomic operations
fn atomic_example() {
    // Create an atomic counter
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    // Spawn 10 threads that increment the counter atomically
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let current = counter.fetch_add(1, Ordering::SeqCst);
            println!("Thread {} incremented counter from {} to {}", i, current, current + 1);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final atomic counter value: {}", counter.load(Ordering::SeqCst));
}

// Function that demonstrates thread synchronization with barriers
fn barrier_example() {
    use std::sync::Barrier;
    
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} is waiting at the barrier", i);
            barrier.wait();
            println!("Thread {} has passed the barrier", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    println!("=== Basic Thread Example ===");
    // Spawn two threads
    let handle1 = thread::spawn(|| thread_function(1));
    let handle2 = thread::spawn(|| thread_function(2));
    
    // Wait for both threads to complete
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("\n=== Message Passing Example ===");
    message_passing_example();
    
    println!("\n=== Shared State Example ===");
    shared_state_example();
    
    println!("\n=== Atomic Operations Example ===");
    atomic_example();
    
    println!("\n=== Barrier Example ===");
    barrier_example();
} 