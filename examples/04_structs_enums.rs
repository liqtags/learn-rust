// Structs and Enums in Rust
// This example demonstrates how to create and use custom types in Rust

// Define a struct (similar to a class in other languages)
struct Person {
    name: String,
    age: u32,
    email: String,
}

// Define an enum (similar to a union type in other languages)
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Multiple values
}

// Define a struct with methods
impl Person {
    // Constructor method (static)
    fn new(name: String, age: u32, email: String) -> Person {
        Person { name, age, email }
    }

    // Instance method
    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }

    // Method that takes ownership
    fn into_email(self) -> String {
        self.email
    }
}

// Define methods for the Message enum
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message received"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    // Creating a struct instance
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };

    // Using struct methods
    person.greet();
    let email = person.into_email();
    println!("Email: {}", email);

    // Creating a struct using the constructor
    let person2 = Person::new(
        String::from("Bob"),
        25,
        String::from("bob@example.com"),
    );
    person2.greet();

    // Using enums
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    // Processing enum variants
    for message in messages.iter() {
        message.call();
    }

    // Pattern matching with enums
    let msg = Message::Write(String::from("Hello"));
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
    }

    // Using if let for single pattern matching
    if let Message::Write(text) = msg {
        println!("Got a message: {}", text);
    }
} 