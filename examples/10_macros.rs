// Macros in Rust
// This example demonstrates different types of macros and their usage

// Declarative macro (macro_rules!)
// This macro creates a function that prints a greeting
macro_rules! create_greeting {
    // Match a single argument
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    
    // Match multiple arguments
    ($name:expr, $greeting:expr) => {
        println!("{}, {}!", $greeting, $name);
    };
}

// Declarative macro with repetition
macro_rules! create_vector {
    // Match zero or more expressions
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Declarative macro with different patterns
macro_rules! calculate {
    // Match addition
    (add $x:expr, $y:expr) => {
        $x + $y
    };
    
    // Match multiplication
    (multiply $x:expr, $y:expr) => {
        $x * $y
    };
}

// Procedural macro (attribute macro)
// Note: This is just a demonstration. Real procedural macros need to be in a separate crate
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Function-like procedural macro
// Note: This is just a demonstration. Real procedural macros need to be in a separate crate
fn create_point(x: i32, y: i32) -> Point {
    Point { x, y }
}

// Example of using built-in macros
fn built_in_macros() {
    // println! macro
    println!("This is a println! macro");
    
    // format! macro
    let s = format!("Hello, {}!", "World");
    println!("{}", s);
    
    // vec! macro
    let v = vec![1, 2, 3];
    println!("Vector: {:?}", v);
    
    // assert! macro
    assert!(true, "This assertion will pass");
    
    // panic! macro
    // panic!("This would cause a panic");
}

fn main() {
    // Using declarative macros
    println!("=== Declarative Macro Examples ===");
    
    create_greeting!("Alice");
    create_greeting!("Bob", "Good morning");
    
    let numbers = create_vector![1, 2, 3, 4, 5];
    println!("Created vector: {:?}", numbers);
    
    let sum = calculate!(add 5, 3);
    let product = calculate!(multiply 4, 2);
    println!("Sum: {}, Product: {}", sum, product);
    
    // Using procedural macros
    println!("\n=== Procedural Macro Examples ===");
    
    let point = Point { x: 10, y: 20 };
    println!("Point: {:?}", point);
    
    let new_point = create_point(30, 40);
    println!("New point: {:?}", new_point);
    
    // Using built-in macros
    println!("\n=== Built-in Macro Examples ===");
    built_in_macros();
    
    // Example of macro hygiene
    println!("\n=== Macro Hygiene Example ===");
    
    macro_rules! create_counter {
        () => {
            {
                let mut count = 0;
                move || {
                    count += 1;
                    count
                }
            }
        };
    }
    
    let counter1 = create_counter!();
    let counter2 = create_counter!();
    
    println!("Counter 1: {}", counter1());
    println!("Counter 1: {}", counter1());
    println!("Counter 2: {}", counter2());
} 