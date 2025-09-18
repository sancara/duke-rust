//! # Data Structures
//! 
//! This module covers Rust's built-in data structures:
//! - Vectors
//! - Hash Maps  
//! - Strings
//! - Structs and Enums

pub mod vectors;
pub mod hash_maps;
pub mod strings;
pub mod structs_enums;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_structures_modules() {
        vectors::demonstrate_vectors();
        hash_maps::demonstrate_hash_maps();
        strings::demonstrate_strings();
        structs_enums::demonstrate_structs_enums();
    }
}