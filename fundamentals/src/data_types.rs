//! Data Types in Rust
//! 
//! This module demonstrates:
//! - Scalar types (integers, floating-point, booleans, characters)
//! - Compound types (tuples, arrays)
//! - Type annotations

/// Demonstrates Rust's data types
pub fn demonstrate_types() {
    println!("=== Data Types ===");
    
    // Integer types
    let small: i8 = -128;
    let medium: i32 = 42;
    let large: i64 = 1_000_000;
    let unsigned: u32 = 42;
    
    println!("Integer types: i8={}, i32={}, i64={}, u32={}", small, medium, large, unsigned);
    
    // Floating-point types
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32
    
    println!("Float types: f64={}, f32={}", x, y);
    
    // Boolean type
    let t = true;
    let f: bool = false;
    
    println!("Boolean types: t={}, f={}", t, f);
    
    // Character type
    let c = 'z';
    let heart_eyed_cat = '😻';
    
    println!("Character types: c='{}', emoji='{}'", c, heart_eyed_cat);
    
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    
    println!("Tuple: ({}, {}, {})", x, y, z);
    println!("Tuple access: tup.0={}, tup.1={}", tup.0, tup.1);
    
    // Array type
    let arr = [1, 2, 3, 4, 5];
    let arr_with_type: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_repeat = [3; 5]; // [3, 3, 3, 3, 3]
    
    println!("Arrays: {:?}, {:?}, {:?}", arr, arr_with_type, arr_repeat);
    println!("Array access: arr[0]={}, arr[2]={}", arr[0], arr[2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_types_compilation() {
        demonstrate_types();
    }
}