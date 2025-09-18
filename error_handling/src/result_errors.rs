//! Result and Recoverable Errors
//! 
//! This module demonstrates:
//! - Result<T, E> type
//! - Error propagation with ?
//! - Combining Results
//! - Error handling patterns

use std::fs::File;
use std::io::{self, Read};

/// Demonstrates Result error handling
pub fn demonstrate_result() {
    println!("=== Result and Recoverable Errors ===");
    
    demonstrate_basic_result();
    demonstrate_error_propagation();
    demonstrate_result_methods();
}

fn demonstrate_basic_result() {
    println!("--- Basic Result Usage ---");
    
    // File operations return Result
    match File::open("hello.txt") {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Problem opening file: {:?}", error),
    }
    
    // Nested match for specific error handling
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                println!("File not found, this is expected in demo");
                return;
            },
            other_error => {
                println!("Problem opening file: {:?}", other_error);
                return;
            }
        },
    };
}

fn demonstrate_error_propagation() {
    println!("--- Error Propagation ---");
    
    // Verbose error propagation
    match read_username_from_file_verbose() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    // Using ? operator for cleaner code
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    // Even shorter version
    match read_username_short() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
}

// Verbose error propagation
fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Using ? operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Even shorter
fn read_username_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Shortest using fs::read_to_string
fn read_username_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

fn demonstrate_result_methods() {
    println!("--- Result Methods ---");
    
    let good_result: Result<i32, &str> = Ok(42);
    let bad_result: Result<i32, &str> = Err("oops");
    
    // is_ok() and is_err()
    println!("good_result.is_ok(): {}", good_result.is_ok());
    println!("bad_result.is_err(): {}", bad_result.is_err());
    
    // unwrap_or() with default value
    let value1 = good_result.unwrap_or(0);
    let value2 = bad_result.unwrap_or(0);
    println!("unwrap_or values: {}, {}", value1, value2);
    
    // unwrap_or_else() with closure
    let value3 = bad_result.unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        -1
    });
    println!("unwrap_or_else value: {}", value3);
    
    // map() to transform Ok value
    let doubled = good_result.map(|x| x * 2);
    println!("Doubled result: {:?}", doubled);
    
    // map_err() to transform Err value
    let mapped_err = bad_result.map_err(|err| format!("Error: {}", err));
    println!("Mapped error: {:?}", mapped_err);
    
    // and_then() for chaining operations
    let result = good_result.and_then(|x| {
        if x > 40 {
            Ok(x + 10)
        } else {
            Err("Too small")
        }
    });
    println!("and_then result: {:?}", result);
}

// Helper function for parsing
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

// Function that combines multiple Results
pub fn add_numbers(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    Ok(num_a + num_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42"), Ok(42));
        assert!(parse_number("not_a_number").is_err());
    }
    
    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers("10", "20"), Ok(30));
        assert!(add_numbers("10", "not_a_number").is_err());
        assert!(add_numbers("not_a_number", "20").is_err());
    }
}