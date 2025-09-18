//! Panic and Unrecoverable Errors
//! 
//! This module demonstrates:
//! - When to use panic!
//! - Panic behavior
//! - Backtrace information

/// Demonstrates panic scenarios (safely)
pub fn demonstrate_panic() {
    println!("=== Panic and Unrecoverable Errors ===");
    
    // This would panic - demonstrated conceptually
    println!("--- Panic Scenarios ---");
    println!("1. Out of bounds access would panic:");
    println!("   let v = vec![1, 2, 3]; let _ = v[99]; // panic!");
    
    println!("2. Explicit panic:");
    println!("   panic!(\"This is a panic message\"); // panic!");
    
    println!("3. unwrap() on None/Err would panic:");
    println!("   let x: Option<i32> = None; x.unwrap(); // panic!");
    
    // Safe demonstrations
    demonstrate_safe_access();
    demonstrate_expect();
}

fn demonstrate_safe_access() {
    println!("--- Safe Access Patterns ---");
    
    let v = vec![1, 2, 3];
    
    // Safe access with get()
    match v.get(2) {
        Some(value) => println!("Safe access v[2] = {}", value),
        None => println!("Index out of bounds"),
    }
    
    match v.get(10) {
        Some(value) => println!("Safe access v[10] = {}", value),
        None => println!("Index 10 is out of bounds"),
    }
}

fn demonstrate_expect() {
    println!("--- Using expect() for Better Error Messages ---");
    
    // expect() is like unwrap() but with custom error message
    let home = std::env::var("HOME")
        .expect("HOME environment variable should be set");
    
    println!("Home directory: {}", home);
    
    // This shows the concept - in real code this would panic with custom message
    let some_value = Some(42);
    let value = some_value.expect("Expected some_value to contain a value");
    println!("Value from expect: {}", value);
}

// Function that might panic
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");
    }
    a / b
}

// Safer version that returns Result
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Ok(5));
        assert!(safe_divide(10, 0).is_err());
    }
    
    #[test]
    #[should_panic(expected = "Cannot divide by zero!")]
    fn test_divide_panic() {
        divide(10, 0);
    }
}