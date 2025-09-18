//! # Error Handling
//! 
//! This module covers Rust's error handling patterns:
//! - panic! and unrecoverable errors
//! - Result<T, E> for recoverable errors
//! - Error propagation with ?
//! - Custom error types

pub mod panic_errors;
pub mod result_errors;
pub mod custom_errors;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_handling_modules() {
        panic_errors::demonstrate_panic();
        result_errors::demonstrate_result();
        custom_errors::demonstrate_custom_errors();
    }
}