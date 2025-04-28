// Unsafe Rust
// This example demonstrates when and how to use unsafe code safely

use std::slice;

// Safe wrapper around unsafe code
struct SafeVec {
    data: Vec<i32>,
}

impl SafeVec {
    // Safe constructor
    fn new() -> SafeVec {
        SafeVec { data: Vec::new() }
    }

    // Safe method that uses unsafe code internally
    fn get(&self, index: usize) -> Option<&i32> {
        if index < self.data.len() {
            // Unsafe block to get raw pointer
            unsafe {
                Some(&*self.data.as_ptr().add(index))
            }
        } else {
            None
        }
    }

    // Safe method that uses unsafe code for performance
    fn sum(&self) -> i32 {
        let mut sum = 0;
        let len = self.data.len();
        
        // Unsafe block for manual iteration
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..len {
                sum += *ptr.add(i);
            }
        }
        sum
    }
}

// Example of raw pointers
fn raw_pointers_example() {
    let mut num = 5;
    
    // Creating raw pointers
    let r1 = &mut num as *mut i32;
    let r2 = &num as *const i32;
    
    // Unsafe block to dereference raw pointers
    unsafe {
        *r1 = 10;
        println!("Value through r1: {}", *r1);
        println!("Value through r2: {}", *r2);
    }
}

// Example of calling unsafe functions
extern "C" {
    fn abs(input: i32) -> i32;
}

// Example of implementing unsafe traits
unsafe trait UnsafeTrait {
    fn do_something(&self);
}

struct UnsafeStruct;

unsafe impl UnsafeTrait for UnsafeStruct {
    fn do_something(&self) {
        println!("Doing something unsafe");
    }
}

// Example of static mutable data
static mut COUNTER: u32 = 0;

fn increment_counter() {
    // Unsafe block to modify static mutable data
    unsafe {
        COUNTER += 1;
    }
}

// Example of FFI (Foreign Function Interface)
#[link(name = "c")]
extern "C" {
    fn rand() -> i32;
}

// Example of memory safety
fn memory_safety_example() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    // Unsafe block to get raw pointer
    let ptr = v.as_mut_ptr();
    let len = v.len();
    
    // Create a slice from raw parts
    let slice = unsafe {
        slice::from_raw_parts_mut(ptr, len)
    };
    
    // Safe to use the slice
    for x in slice {
        *x *= 2;
    }
    
    println!("Doubled vector: {:?}", v);
}

fn main() {
    // Using SafeVec
    println!("=== SafeVec Example ===");
    let safe_vec = SafeVec {
        data: vec![1, 2, 3, 4, 5],
    };
    
    println!("Element at index 2: {:?}", safe_vec.get(2));
    println!("Sum of elements: {}", safe_vec.sum());
    
    // Raw pointers
    println!("\n=== Raw Pointers Example ===");
    raw_pointers_example();
    
    // Calling unsafe functions
    println!("\n=== Unsafe Function Call Example ===");
    let result = unsafe { abs(-5) };
    println!("Absolute value: {}", result);
    
    // Using unsafe trait
    println!("\n=== Unsafe Trait Example ===");
    let unsafe_struct = UnsafeStruct;
    unsafe { unsafe_struct.do_something() };
    
    // Static mutable data
    println!("\n=== Static Mutable Data Example ===");
    increment_counter();
    unsafe {
        println!("Counter value: {}", COUNTER);
    }
    
    // Memory safety
    println!("\n=== Memory Safety Example ===");
    memory_safety_example();
    
    // FFI example
    println!("\n=== FFI Example ===");
    let random = unsafe { rand() };
    println!("Random number: {}", random);
} 