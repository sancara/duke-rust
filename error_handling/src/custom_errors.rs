//! Custom Error Types
//! 
//! This module demonstrates:
//! - Creating custom error types
//! - Error trait implementation
//! - Error conversion with From trait
//! - Error chaining

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Demonstrates custom error types
pub fn demonstrate_custom_errors() {
    println!("=== Custom Error Types ===");
    
    demonstrate_simple_custom_error();
    demonstrate_complex_custom_error();
    demonstrate_error_conversion();
}

// Simple custom error
#[derive(Debug)]
pub struct SimpleError {
    message: String,
}

impl SimpleError {
    pub fn new(message: &str) -> SimpleError {
        SimpleError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleError: {}", self.message)
    }
}

impl Error for SimpleError {}

// More complex custom error with different variants
#[derive(Debug)]
pub enum CalculationError {
    DivisionByZero,
    NegativeSquareRoot,
    ParseError(ParseIntError),
    InvalidInput(String),
}

impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculationError::DivisionByZero => write!(f, "Cannot divide by zero"),
            CalculationError::NegativeSquareRoot => write!(f, "Cannot calculate square root of negative number"),
            CalculationError::ParseError(err) => write!(f, "Parse error: {}", err),
            CalculationError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl Error for CalculationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CalculationError::ParseError(err) => Some(err),
            _ => None,
        }
    }
}

// Automatic conversion from ParseIntError
impl From<ParseIntError> for CalculationError {
    fn from(err: ParseIntError) -> CalculationError {
        CalculationError::ParseError(err)
    }
}

fn demonstrate_simple_custom_error() {
    println!("--- Simple Custom Error ---");
    
    let result = validate_age(-5);
    match result {
        Ok(age) => println!("Valid age: {}", age),
        Err(e) => println!("Error: {}", e),
    }
    
    let result2 = validate_age(25);
    match result2 {
        Ok(age) => println!("Valid age: {}", age),
        Err(e) => println!("Error: {}", e),
    }
}

fn validate_age(age: i32) -> Result<i32, SimpleError> {
    if age < 0 {
        Err(SimpleError::new("Age cannot be negative"))
    } else if age > 150 {
        Err(SimpleError::new("Age seems unrealistic"))
    } else {
        Ok(age)
    }
}

fn demonstrate_complex_custom_error() {
    println!("--- Complex Custom Error ---");
    
    let operations = vec![
        ("10", "2", "divide"),
        ("10", "0", "divide"),
        ("-4", "0", "sqrt"),
        ("not_a_number", "5", "add"),
        ("10", "5", "add"),
    ];
    
    for (a, b, op) in operations {
        match perform_calculation(a, b, op) {
            Ok(result) => println!("{} {} {} = {}", a, op, b, result),
            Err(e) => {
                println!("Error in {} {} {}: {}", a, op, b, e);
                
                // Print error chain
                let mut current_error: &dyn Error = &e;
                while let Some(source) = current_error.source() {
                    println!("  Caused by: {}", source);
                    current_error = source;
                }
            }
        }
    }
}

fn perform_calculation(a: &str, b: &str, operation: &str) -> Result<f64, CalculationError> {
    let num_a: f64 = a.parse::<i32>()? as f64; // ? automatically converts ParseIntError
    let num_b: f64 = b.parse::<i32>()? as f64;
    
    match operation {
        "add" => Ok(num_a + num_b),
        "subtract" => Ok(num_a - num_b),
        "multiply" => Ok(num_a * num_b),
        "divide" => {
            if num_b == 0.0 {
                Err(CalculationError::DivisionByZero)
            } else {
                Ok(num_a / num_b)
            }
        },
        "sqrt" => {
            if num_a < 0.0 {
                Err(CalculationError::NegativeSquareRoot)
            } else {
                Ok(num_a.sqrt())
            }
        },
        _ => Err(CalculationError::InvalidInput(format!("Unknown operation: {}", operation))),
    }
}

fn demonstrate_error_conversion() {
    println!("--- Error Conversion ---");
    
    // Demonstrate automatic error conversion
    let result = parse_and_double("42");
    match result {
        Ok(value) => println!("Parsed and doubled: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    let result2 = parse_and_double("not_a_number");
    match result2 {
        Ok(value) => println!("Parsed and doubled: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

fn parse_and_double(s: &str) -> Result<i32, CalculationError> {
    let num: i32 = s.parse()?; // ParseIntError automatically converted to CalculationError
    Ok(num * 2)
}

// Type alias for common Result pattern
pub type CalculationResult<T> = Result<T, CalculationError>;

pub fn safe_divide_custom(a: i32, b: i32) -> CalculationResult<f64> {
    if b == 0 {
        Err(CalculationError::DivisionByZero)
    } else {
        Ok(a as f64 / b as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_age() {
        assert!(validate_age(25).is_ok());
        assert!(validate_age(-5).is_err());
        assert!(validate_age(200).is_err());
    }
    
    #[test]
    fn test_safe_divide_custom() {
        assert!(safe_divide_custom(10, 2).is_ok());
        assert!(safe_divide_custom(10, 0).is_err());
    }
    
    #[test]
    fn test_perform_calculation() {
        assert!(perform_calculation("10", "2", "add").is_ok());
        assert!(perform_calculation("10", "0", "divide").is_err());
        assert!(perform_calculation("not_a_number", "5", "add").is_err());
    }
}