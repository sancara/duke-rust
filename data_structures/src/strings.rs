//! Strings in Rust
//! 
//! This module demonstrates:
//! - String vs &str
//! - Creating and updating strings
//! - String methods and operations
//! - UTF-8 handling

/// Demonstrates string operations
pub fn demonstrate_strings() {
    println!("=== Strings ===");
    
    // Creating strings
    let mut s1 = String::new();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");
    
    println!("Empty string: '{}'", s1);
    println!("String from literal: '{}'", s2);
    println!("String::from: '{}'", s3);
    
    // Updating strings
    s1.push_str("hello");
    s1.push(' ');
    s1.push('w');
    s1.push_str("orld");
    
    println!("After updates: '{}'", s1);
    
    // Concatenation
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // s4 is moved here and can no longer be used
    
    println!("Concatenated: '{}'", s6);
    
    // Format macro
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let s10 = format!("{}-{}-{}", s7, s8, s9);
    
    println!("Formatted: '{}'", s10);
    
    // String slicing (be careful with UTF-8!)
    let hello = "Здравствуйте"; // Russian
    let s = &hello[0..4]; // Gets first 4 bytes (2 characters in this case)
    println!("String slice: '{}'", s);
    
    // Iterating over strings
    println!("--- Character iteration ---");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    println!("--- Byte iteration ---");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    
    // String methods
    let s = String::from("  Hello, World!  ");
    println!("Original: '{}'", s);
    println!("Length: {}", s.len());
    println!("Is empty: {}", s.is_empty());
    println!("Trimmed: '{}'", s.trim());
    println!("To lowercase: '{}'", s.to_lowercase());
    println!("To uppercase: '{}'", s.to_uppercase());
    println!("Contains 'World': {}", s.contains("World"));
    println!("Starts with '  Hello': {}", s.starts_with("  Hello"));
    println!("Ends with '!  ': {}", s.ends_with("!  "));
    
    // String splitting
    let data = "name,age,city";
    let parts: Vec<&str> = data.split(',').collect();
    println!("Split result: {:?}", parts);
    
    // String replacement
    let s = "I like apples";
    let new_s = s.replace("apples", "oranges");
    println!("Replaced: '{}'", new_s);
    
    // Parsing strings
    let num_str = "42";
    match num_str.parse::<i32>() {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Parse error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings() {
        demonstrate_strings();
    }
}