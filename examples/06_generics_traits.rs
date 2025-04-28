// Generics and Traits in Rust
// This example demonstrates how to use generics and traits for writing reusable code

// Define a trait (similar to an interface in other languages)
trait Printable {
    fn print(&self);
}

// Define a trait with a default implementation
trait Describable {
    fn describe(&self) -> String {
        String::from("This is a describable object")
    }
}

// Define a struct with a generic type parameter
struct Container<T> {
    value: T,
}

// Implement methods for Container with a generic type
impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn get_value(&self) -> &T {
        &self.value
    }
}

// Implement Printable trait for Container with any type that implements Display
impl<T: std::fmt::Display> Printable for Container<T> {
    fn print(&self) {
        println!("Container value: {}", self.value);
    }
}

// Implement Describable trait for Container
impl<T> Describable for Container<T> {}

// Define a struct that implements both traits
struct Person {
    name: String,
    age: u32,
}

// Implement Printable trait for Person
impl Printable for Person {
    fn print(&self) {
        println!("Person: {}, Age: {}", self.name, self.age);
    }
}

// Implement Describable trait for Person with custom implementation
impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

// Function that takes any type that implements Printable
fn print_anything<T: Printable>(item: &T) {
    item.print();
}

// Function that takes any type that implements both Printable and Describable
fn print_and_describe<T: Printable + Describable>(item: &T) {
    item.print();
    println!("Description: {}", item.describe());
}

// Generic function with multiple type parameters
fn swap<T>(x: T, y: T) -> (T, T) {
    (y, x)
}

fn main() {
    // Using generic Container with different types
    let number_container = Container::new(42);
    let string_container = Container::new(String::from("Hello"));
    
    // Using trait methods
    number_container.print();
    string_container.print();
    println!("Description: {}", number_container.describe());

    // Using Person struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    // Using trait methods on Person
    person.print();
    println!("Description: {}", person.describe());

    // Using generic function with trait bounds
    print_anything(&number_container);
    print_anything(&person);
    
    // Using function with multiple trait bounds
    print_and_describe(&person);

    // Using generic swap function
    let (a, b) = swap(1, 2);
    println!("Swapped: {}, {}", a, b);
    
    let (s1, s2) = swap(String::from("Hello"), String::from("World"));
    println!("Swapped: {}, {}", s1, s2);

    // Using where clause for complex trait bounds
    fn process<T>(item: T)
    where
        T: Printable + Describable + std::fmt::Debug,
    {
        item.print();
        println!("Debug: {:?}", item);
    }

    // Using associated types in traits
    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    // Using generic associated types
    trait Container {
        type Item;
        fn get(&self) -> &Self::Item;
    }
} 