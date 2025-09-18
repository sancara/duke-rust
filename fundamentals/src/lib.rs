//! # Fundamentals
//! 
//! This module covers fundamental Rust concepts including:
//! - Variables and mutability
//! - Data types
//! - Functions
//! - Control flow
//! - Ownership and borrowing

pub mod variables;
pub mod data_types;
pub mod functions;
pub mod control_flow;
pub mod ownership;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fundamentals_modules() {
        // Basic test to ensure modules compile
        variables::demonstrate_variables();
        data_types::demonstrate_types();
        functions::demonstrate_functions();
        control_flow::demonstrate_control_flow();
        ownership::demonstrate_ownership();
    }
}
