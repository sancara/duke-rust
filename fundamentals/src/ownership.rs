//! Ownership and Borrowing in Rust
//! 
//! This module demonstrates:
//! - Ownership rules
//! - Moving and copying
//! - References and borrowing
//! - Slices

/// Demonstrates ownership concepts
pub fn demonstrate_ownership() {
    println!("=== Ownership and Borrowing ===");
    
    demonstrate_ownership_rules();
    demonstrate_references_and_borrowing();
    demonstrate_slices();
}

fn demonstrate_ownership_rules() {
    println!("--- Ownership Rules ---");
    
    // String literals are immutable and stored in the binary
    let s1 = "hello"; // String literal
    println!("String literal: {}", s1);
    
    // String type is mutable and stored on the heap
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("Mutable String: {}", s2);
    
    // Move semantics
    let s3 = String::from("hello");
    let s4 = s3; // s3 is moved to s4, s3 is no longer valid
    println!("After move: {}", s4);
    // println!("This would cause error: {}", s3); // Error!
    
    // Clone for deep copy
    let s5 = String::from("hello");
    let s6 = s5.clone();
    println!("After clone: s5={}, s6={}", s5, s6);
    
    // Copy types (stack-only data)
    let x = 5;
    let y = x; // Copy, not move
    println!("After copy: x={}, y={}", x, y);
}

fn demonstrate_references_and_borrowing() {
    println!("--- References and Borrowing ---");
    
    let s1 = String::from("hello");
    
    // Immutable reference
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable reference
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("After change: {}", s2);
    
    // Multiple immutable references are allowed
    let r1 = &s1;
    let r2 = &s1;
    println!("r1: {}, r2: {}", r1, r2);
    
    // But only one mutable reference at a time
    let mut s3 = String::from("hello");
    {
        let r3 = &mut s3;
        r3.push_str(" world");
    } // r3 goes out of scope here
    println!("s3: {}", s3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but because it's a reference, nothing happens

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn demonstrate_slices() {
    println!("--- Slices ---");
    
    let s = String::from("hello world");
    
    // String slices
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Slices: '{}' and '{}'", hello, world);
    
    // Slice syntax variations
    let slice1 = &s[0..2];
    let slice2 = &s[..2]; // Same as above
    let slice3 = &s[3..];
    let slice4 = &s[..]; // Entire string
    println!("Slice variations: '{}', '{}', '{}', '{}'", slice1, slice2, slice3, slice4);
    
    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Array slice: {:?}", slice);
    
    // First word function using slices
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word: '{}'", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership() {
        demonstrate_ownership();
    }
    
    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("hello"), "hello");
    }
}