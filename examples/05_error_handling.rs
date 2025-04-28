// Error Handling in Rust
// This example demonstrates different ways to handle errors in Rust

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// Custom error type using enum
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(ParseIntError),
    CustomError(String),
}

// Implement conversion from io::Error to AppError
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

// Implement conversion from ParseIntError to AppError
impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}

// Function that returns a Result
fn read_file(path: &str) -> Result<String, AppError> {
    let mut file = File::open(path)?;  // ? operator propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Function that returns a Result with custom error
fn parse_number(s: &str) -> Result<i32, AppError> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(AppError::from(e)),
    }
}

// Function that uses Option
fn find_first_even(numbers: &[i32]) -> Option<&i32> {
    numbers.iter().find(|&&n| n % 2 == 0)
}

// Function that demonstrates panic
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

fn main() {
    // Using Result with match
    match read_file("nonexistent.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {:?}", e),
    }

    // Using Result with unwrap_or_else
    let number = parse_number("42").unwrap_or_else(|e| {
        println!("Error parsing number: {:?}", e);
        0
    });
    println!("Parsed number: {}", number);

    // Using Option with match
    let numbers = vec![1, 3, 5, 7, 9];
    match find_first_even(&numbers) {
        Some(n) => println!("Found even number: {}", n),
        None => println!("No even numbers found"),
    }

    // Using Option with if let
    let numbers = vec![1, 2, 3, 4, 5];
    if let Some(n) = find_first_even(&numbers) {
        println!("Found even number: {}", n);
    }

    // Using Option with unwrap_or
    let numbers = vec![1, 3, 5];
    let first_even = find_first_even(&numbers).unwrap_or(&0);
    println!("First even number or 0: {}", first_even);

    // Demonstrating panic
    // Uncomment to see panic in action
    // let result = divide(10, 0);
    // println!("Result: {}", result);

    // Using the ? operator in a function that returns Result
    fn process_file(path: &str) -> Result<String, AppError> {
        let contents = read_file(path)?;
        let number = parse_number(&contents)?;
        Ok(format!("Processed number: {}", number))
    }

    // Error handling with custom error type
    match process_file("nonexistent.txt") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {:?}", e),
    }
} 