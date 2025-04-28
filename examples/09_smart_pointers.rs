// Smart Pointers in Rust
// This example demonstrates Box, Rc, Arc, and RefCell smart pointers

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

// Custom smart pointer implementation
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement Deref trait for MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Example of a recursive type using Box
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Example of a type that needs interior mutability
struct Messenger {
    message: RefCell<String>,
}

impl Messenger {
    fn new(message: String) -> Messenger {
        Messenger {
            message: RefCell::new(message),
        }
    }

    fn update_message(&self, new_message: String) {
        *self.message.borrow_mut() = new_message;
    }

    fn get_message(&self) -> String {
        self.message.borrow().clone()
    }
}

// Example of a type that needs shared ownership
struct SharedData {
    data: Rc<String>,
}

impl SharedData {
    fn new(data: String) -> SharedData {
        SharedData {
            data: Rc::new(data),
        }
    }

    fn get_data(&self) -> Rc<String> {
        Rc::clone(&self.data)
    }
}

fn main() {
    // Box<T> - For heap allocation
    println!("=== Box Examples ===");
    
    let b = Box::new(5);
    println!("Box contains: {}", b);
    
    // Using Box for recursive types
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("List: {:?}", list);
    
    // Custom smart pointer
    println!("\n=== Custom Smart Pointer Example ===");
    
    let x = 5;
    let y = MyBox::new(x);
    println!("Custom smart pointer value: {}", *y);
    
    // RefCell<T> - For interior mutability
    println!("\n=== RefCell Examples ===");
    
    let messenger = Messenger::new(String::from("Hello"));
    println!("Initial message: {}", messenger.get_message());
    
    messenger.update_message(String::from("Goodbye"));
    println!("Updated message: {}", messenger.get_message());
    
    // Rc<T> - For shared ownership
    println!("\n=== Rc Examples ===");
    
    let shared_data = SharedData::new(String::from("Shared data"));
    let data1 = shared_data.get_data();
    let data2 = shared_data.get_data();
    
    println!("Data 1: {}", data1);
    println!("Data 2: {}", data2);
    println!("Reference count: {}", Rc::strong_count(&data1));
    
    // Combining Rc and RefCell
    println!("\n=== Rc + RefCell Example ===");
    
    let shared_messenger = Rc::new(Messenger::new(String::from("Shared message")));
    let messenger1 = Rc::clone(&shared_messenger);
    let messenger2 = Rc::clone(&shared_messenger);
    
    messenger1.update_message(String::from("Updated shared message"));
    println!("Message from messenger1: {}", messenger1.get_message());
    println!("Message from messenger2: {}", messenger2.get_message());
    
    // Box<dyn Trait> - For trait objects
    println!("\n=== Trait Object Example ===");
    
    trait Draw {
        fn draw(&self);
    }
    
    struct Circle;
    struct Square;
    
    impl Draw for Circle {
        fn draw(&self) {
            println!("Drawing a circle");
        }
    }
    
    impl Draw for Square {
        fn draw(&self) {
            println!("Drawing a square");
        }
    }
    
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];
    
    for shape in shapes {
        shape.draw();
    }
} 