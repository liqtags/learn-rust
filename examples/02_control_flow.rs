// Control Flow in Rust
// This example demonstrates if/else statements, loops, and pattern matching

fn main() {
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