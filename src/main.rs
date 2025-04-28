// Basic Variables and Types in Rust
// This example demonstrates the fundamental variable types and how to use them

// Control Flow in Rust
// This example demonstrates if/else statements, loops, and pattern matching

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
    
    // If/Else Statements
    let number = 7;
    
    if number < 5 {
        println!("Number is less than 5");
    } else if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is 5");
    }
    
    // Using if in a let statement (ternary-like)
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("Value is: {}", value);
    
    // Loops
    println!("\nLoop Examples:");
    
    // loop (infinite loop)
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
    }
    println!("Counter reached: {}", counter);
    
    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");
    
    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Value is: {}", element);
    }
    
    // Range with for
    for number in 1..4 {
        println!("{}!", number);
    }
    
    // Pattern Matching with match
    println!("\nPattern Matching Examples:");
    
    let number = 13;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    
    // Pattern matching with destructuring
    let point = (3, 4);
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("Y axis at {}", y),
        (x, 0) => println!("X axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
} 