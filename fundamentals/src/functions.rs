//! Functions in Rust
//! 
//! This module demonstrates:
//! - Function definitions
//! - Parameters and arguments
//! - Return values
//! - Expressions vs statements

/// Demonstrates function definitions and usage
pub fn demonstrate_functions() {
    println!("=== Functions ===");
    
    // Call functions
    another_function(5);
    print_labeled_measurement(5, 'h');
    
    let x = five();
    println!("The value of x is: {}", x);
    
    let y = plus_one(5);
    println!("The value of y is: {}", y);
    
    // Demonstrate expressions vs statements
    let y = {
        let x = 3;
        x + 1 // Expression (no semicolon)
    };
    println!("The value of y from block expression: {}", y);
    
    // Demonstrate multiple return values using tuples
    let (sum, product) = calculate_sum_and_product(4, 5);
    println!("Sum: {}, Product: {}", sum, product);
}

/// Function with one parameter
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

/// Function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/// Function that returns a value
fn five() -> i32 {
    5 // Expression without semicolon
}

/// Function that takes a parameter and returns a value
fn plus_one(x: i32) -> i32 {
    x + 1
}

/// Function that returns multiple values using a tuple
fn calculate_sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions() {
        demonstrate_functions();
        assert_eq!(five(), 5);
        assert_eq!(plus_one(5), 6);
        assert_eq!(calculate_sum_and_product(3, 4), (7, 12));
    }
}