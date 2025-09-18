//! Hash Maps in Rust
//! 
//! This module demonstrates:
//! - Creating hash maps
//! - Accessing values
//! - Updating hash maps
//! - Iterating over hash maps

use std::collections::HashMap;

/// Demonstrates hash map operations
pub fn demonstrate_hash_maps() {
    println!("=== Hash Maps ===");
    
    // Creating hash maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("Scores: {:?}", scores);
    
    // Creating from vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    println!("Scores from vectors: {:?}", scores2);
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    match score {
        Some(s) => println!("Blue team score: {}", s),
        None => println!("Team not found"),
    }
    
    // Iterating
    println!("--- All scores ---");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Updating values
    println!("--- Updating ---");
    
    // Overwriting
    scores.insert(String::from("Blue"), 25);
    println!("After overwrite: {:?}", scores);
    
    // Only insert if key doesn't exist
    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(100); // Won't change Blue
    println!("After entry: {:?}", scores);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word count: {:?}", map);
    
    // Hash map methods
    let mut inventory = HashMap::new();
    inventory.insert("apples", 20);
    inventory.insert("bananas", 15);
    inventory.insert("oranges", 10);
    
    println!("Inventory: {:?}", inventory);
    println!("Contains apples: {}", inventory.contains_key("apples"));
    println!("Number of items: {}", inventory.len());
    
    inventory.remove("bananas");
    println!("After removing bananas: {:?}", inventory);
    
    // Clear all
    inventory.clear();
    println!("After clear: {:?}", inventory);
    println!("Is empty: {}", inventory.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_maps() {
        demonstrate_hash_maps();
    }
}