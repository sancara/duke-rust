//! Variables and Mutability in Rust
//! 
//! This module demonstrates:
//! - Variable declarations
//! - Mutability vs Immutability
//! - Shadowing
//! - Constants

/// Demonstrates variable declarations and mutability
pub fn demonstrate_variables() {
    println!("=== Variables and Mutability ===");
    
    // Immutable variable (default)
    let x = 5;
    println!("The value of x is: {}", x);
    
    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The value of y after change is: {}", y);
    
    // Shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z after shadowing is: {}", z);
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("Maximum points: {}", MAX_POINTS);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variables_compilation() {
        // This test ensures the function compiles and runs without panic
        demonstrate_variables();
    }
}