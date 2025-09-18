//! Structs and Enums in Rust
//! 
//! This module demonstrates:
//! - Defining and using structs
//! - Methods and associated functions
//! - Enums and pattern matching
//! - Option and Result types

/// Demonstrates structs and enums
pub fn demonstrate_structs_enums() {
    println!("=== Structs and Enums ===");
    
    demonstrate_structs();
    demonstrate_enums();
    demonstrate_option_result();
}

// Struct definition
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);

// Unit struct
#[derive(Debug)]
struct AlwaysEqual;

// Rectangle struct for method examples
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Associated function (like static method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn demonstrate_structs() {
    println!("--- Structs ---");
    
    // Creating struct instances
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("User1: {:?}", user1);
    
    // Struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Takes remaining fields from user1
    };
    
    println!("User2: {:?}", user2);
    
    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);
    
    println!("Colors: {:?}, {:?}", black, origin);
    
    // Unit struct
    let subject = AlwaysEqual;
    println!("Unit struct: {:?}", subject);
    
    // Rectangle methods
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("Rectangle: {:?}", rect1);
    println!("Area: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // Associated function
    let square = Rectangle::square(25);
    println!("Square: {:?}", square);
    println!("Square area: {}", square.area());
}

// Enum definitions
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        }
    }
}

fn demonstrate_enums() {
    println!("--- Enums ---");
    
    // Basic enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IP versions: {:?}, {:?}", four, six);
    
    // Enums with data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("Addresses: {:?}, {:?}", home, loopback);
    
    // Complex enums
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);
    
    m1.call();
    m2.call();
    m3.call();
    m4.call();
}

fn demonstrate_option_result() {
    println!("--- Option and Result ---");
    
    // Option enum
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Options: {:?}, {:?}, {:?}", some_number, some_string, absent_number);
    
    // Pattern matching with Option
    match some_number {
        Some(value) => println!("Got a value: {}", value),
        None => println!("Got nothing"),
    }
    
    // Using if let
    if let Some(value) = some_number {
        println!("if let value: {}", value);
    }
    
    // Result enum for error handling
    let good_result: Result<i32, &str> = Ok(42);
    let bad_result: Result<i32, &str> = Err("something went wrong");
    
    println!("Results: {:?}, {:?}", good_result, bad_result);
    
    // Pattern matching with Result
    match good_result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    match bad_result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    // Parsing example with Result
    let number_str = "42";
    let parsed_number: Result<i32, _> = number_str.parse();
    
    match parsed_number {
        Ok(num) => println!("Parsed successfully: {}", num),
        Err(e) => println!("Failed to parse: {}", e),
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs_enums() {
        demonstrate_structs_enums();
    }
    
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };
        assert_eq!(rect.area(), 1500);
    }
    
    #[test]
    fn test_build_user() {
        let user = build_user(
            String::from("test@example.com"),
            String::from("testuser")
        );
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
        assert_eq!(user.active, true);
        assert_eq!(user.sign_in_count, 1);
    }
}