// Functions and Ownership in Rust
// This example demonstrates Rust's unique ownership system and function concepts

fn main() {
    // Basic Function Call
    let result = add_numbers(5, 3);
    println!("Sum is: {}", result);

    // Ownership Example
    let s1 = String::from("hello");  // s1 owns the string
    let s2 = s1;                     // ownership moves to s2
    // println!("{}", s1);           // This would cause a compile error - s1 no longer owns the string
    println!("{}", s2);              // This works fine

    // Clone Example (Deep Copy)
    let s3 = String::from("world");
    let s4 = s3.clone();             // Creates a deep copy
    println!("s3: {}, s4: {}", s3, s4);  // Both are valid

    // Function with Ownership Transfer
    let s5 = String::from("hello");
    takes_ownership(s5);             // s5's value moves into the function
    // println!("{}", s5);           // This would cause a compile error - s5 no longer owns the string

    // Function with References (Borrowing)
    let s6 = String::from("hello");
    let len = calculate_length(&s6);  // s6 is borrowed, not moved
    println!("Length of '{}' is {}", s6, len);  // s6 is still valid

    // Mutable References
    let mut s7 = String::from("hello");
    change(&mut s7);                 // s7 is mutably borrowed
    println!("{}", s7);              // s7 is still valid and modified

    // Multiple References
    let mut s8 = String::from("hello");
    let r1 = &s8;                    // First immutable borrow
    let r2 = &s8;                    // Second immutable borrow
    println!("{} and {}", r1, r2);   // Multiple immutable borrows are allowed
    // let r3 = &mut s8;            // This would cause a compile error - can't have mutable borrow while immutable borrows exist
}

// Basic function with parameters and return value
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y  // No semicolon means this is the return value
}

// Function that takes ownership of a String
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is dropped

// Function that borrows a String (takes a reference)
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but doesn't drop the string because it doesn't own it

// Function that mutably borrows a String
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Example of a function that returns ownership
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string  // some_string is returned and moves out to the calling function
}

// Example of a function that takes and gives back ownership
fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string is returned and moves out to the calling function
} 