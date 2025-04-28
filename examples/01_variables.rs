// Basic Variables and Types in Rust
// This example demonstrates the fundamental variable types and how to use them

fn main() {
    // Integer types
    let integer: i32 = 42;  // 32-bit signed integer
    let unsigned: u32 = 42; // 32-bit unsigned integer
    
    // Floating-point types
    let float: f64 = 3.14;  // 64-bit floating point
    
    // Boolean type
    let boolean: bool = true;
    
    // Character type (Unicode)
    let character: char = 'A';
    
    // String type
    let string: String = String::from("Hello, Rust!");
    
    // Printing variables
    println!("Integer: {}", integer);
    println!("Unsigned: {}", unsigned);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("String: {}", string);
    
    // Type inference
    let inferred = 42; // Rust will infer this as i32
    println!("Inferred type: {}", inferred);
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);
    
    // Shadowing
    let x = 5;
    let x = x + 1; // Shadowing the previous x
    println!("Shadowed x: {}", x);
} 