//! Vectors in Rust
//! 
//! This module demonstrates:
//! - Creating vectors
//! - Accessing elements
//! - Updating vectors
//! - Iterating over vectors

/// Demonstrates vector operations
pub fn demonstrate_vectors() {
    println!("=== Vectors ===");
    
    // Creating vectors
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3]; // Using macro
    
    println!("Empty vector: {:?}", v1);
    println!("Vector with values: {:?}", v2);
    
    // Adding elements
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    println!("After pushing: {:?}", v1);
    
    // Accessing elements
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);
    
    // Safe access with get method
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // Iterating over vectors
    println!("--- Iterating ---");
    for i in &v2 {
        println!("{}", i);
    }
    
    // Mutable iteration
    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50; // Dereference to modify
    }
    println!("After modification: {:?}", v3);
    
    // Using enums to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    println!("Spreadsheet row: {:?}", row);
    
    // Vector methods
    let mut v4 = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", v4);
    println!("Length: {}", v4.len());
    println!("Is empty: {}", v4.is_empty());
    
    v4.pop(); // Remove last element
    println!("After pop: {:?}", v4);
    
    v4.insert(1, 10); // Insert at index 1
    println!("After insert: {:?}", v4);
    
    v4.remove(2); // Remove at index 2
    println!("After remove: {:?}", v4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vectors() {
        demonstrate_vectors();
    }
}