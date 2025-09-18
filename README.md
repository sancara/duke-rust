# Duke Rust Learning Repository

A comprehensive Rust programming learning repository following the Duke University specialization program. This repository contains practical examples, exercises, and projects to master Rust programming concepts.

## 🦀 Repository Structure

This repository is organized as a Rust workspace with multiple learning modules:

```
duke-rust/
├── fundamentals/          # Core Rust concepts
├── data_structures/       # Built-in data structures
├── error_handling/        # Error handling patterns
├── cli_app/              # Command-line application examples
└── Cargo.toml            # Workspace configuration
```

## 📚 Learning Modules

### 1. Fundamentals (`fundamentals/`)

Core Rust programming concepts including:

- **Variables & Mutability** (`variables.rs`)
  - Variable declarations
  - Mutability vs Immutability  
  - Shadowing and constants

- **Data Types** (`data_types.rs`)
  - Scalar types (integers, floats, booleans, chars)
  - Compound types (tuples, arrays)
  - Type annotations

- **Functions** (`functions.rs`)
  - Function definitions and parameters
  - Return values and expressions
  - Multiple return values with tuples

- **Control Flow** (`control_flow.rs`)
  - if expressions and conditionals
  - Loops (loop, while, for)
  - Pattern matching with match

- **Ownership & Borrowing** (`ownership.rs`)
  - Ownership rules and move semantics
  - References and borrowing
  - String slices and array slices

### 2. Data Structures (`data_structures/`)

Rust's built-in data structures and collections:

- **Vectors** (`vectors.rs`)
  - Dynamic arrays and operations
  - Iteration and modification
  - Storing multiple types with enums

- **Hash Maps** (`hash_maps.rs`)
  - Key-value storage and retrieval
  - Updating and iterating
  - Real-world usage patterns

- **Strings** (`strings.rs`)
  - String vs &str differences
  - String operations and methods
  - UTF-8 handling and slicing

- **Structs & Enums** (`structs_enums.rs`)
  - Struct definitions and methods
  - Enum variants and pattern matching
  - Option and Result types

### 3. Error Handling (`error_handling/`)

Comprehensive error handling in Rust:

- **Panic Errors** (`panic_errors.rs`)
  - When to use panic!
  - Safe access patterns
  - expect() for better error messages

- **Result Errors** (`result_errors.rs`)
  - Result<T, E> type usage
  - Error propagation with ?
  - Result methods and combinations

- **Custom Errors** (`custom_errors.rs`)
  - Creating custom error types
  - Error trait implementation
  - Error conversion and chaining

### 4. CLI Application (`cli_app/`)

A complete command-line application demonstrating:

- Argument parsing with `clap`
- File I/O operations
- JSON processing with `serde`
- Error handling in CLI context
- Real-world application structure

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Basic understanding of programming concepts

### Installation

1. Clone the repository:
```bash
git clone https://github.com/sancara/duke-rust.git
cd duke-rust
```

2. Build the workspace:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## 🎯 Running Examples

### Fundamentals Examples

```bash
# Run all fundamental examples
cargo test -p fundamentals

# Run specific module examples
cargo test -p fundamentals variables::demonstrate_variables
```

### Data Structures Examples

```bash
# Run all data structure examples  
cargo test -p data_structures

# Run specific examples
cargo test -p data_structures vectors::demonstrate_vectors
```

### Error Handling Examples

```bash
# Run all error handling examples
cargo test -p error_handling

# Run specific examples
cargo test -p error_handling result_errors::demonstrate_result
```

### CLI Application

```bash
# Build the CLI app
cargo build -p cli_app

# Run the CLI app with examples
cargo run -p cli_app -- greet --name "Rust Learner" --count 3
cargo run -p cli_app -- calc 10 5 --operation "+"
cargo run -p cli_app -- file list
cargo run -p cli_app -- json create sample.json
```

## 📖 Learning Path

### Beginner Level
1. Start with `fundamentals/` module
2. Work through variables, data types, and functions
3. Master ownership and borrowing concepts

### Intermediate Level  
1. Explore `data_structures/` module
2. Learn about vectors, hash maps, and strings
3. Understand structs, enums, and pattern matching

### Advanced Level
1. Dive into `error_handling/` module
2. Learn panic vs Result patterns
3. Create custom error types

### Project Level
1. Study the `cli_app/` implementation
2. Build your own CLI applications
3. Apply all learned concepts in practice

## 🧪 Testing

Each module includes comprehensive tests demonstrating proper usage:

```bash
# Run all tests
cargo test

# Run tests for specific module
cargo test -p fundamentals
cargo test -p data_structures
cargo test -p error_handling
cargo test -p cli_app

# Run tests with output
cargo test -- --nocapture
```

## 🔧 Development

### Adding New Examples

1. Navigate to the appropriate module
2. Create new .rs files following the established patterns
3. Add module declarations to lib.rs
4. Include tests demonstrating functionality
5. Update documentation

### Code Style

This repository follows standard Rust conventions:
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive tests
- Include documentation comments

## 📝 Additional Resources

### Official Rust Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)

### Duke University Resources  
- [Duke Rust Specialization](https://www.coursera.org/specializations/rust-programming)
- [Applied Machine Learning in Python](https://www.coursera.org/specializations/data-science-python)

### Community Resources
- [Rust Users Forum](https://users.rust-lang.org/)
- [r/rust Subreddit](https://www.reddit.com/r/rust/)
- [Rust Discord](https://discord.gg/rust-lang)

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add your examples with tests
4. Ensure all tests pass
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎓 Learning Outcomes

After completing this repository, you should be able to:

- ✅ Understand Rust's ownership system
- ✅ Write memory-safe code without garbage collection
- ✅ Handle errors effectively with Result types
- ✅ Work with Rust's type system and pattern matching
- ✅ Build command-line applications
- ✅ Use Rust's standard library collections
- ✅ Write idiomatic Rust code
- ✅ Test and document Rust projects

Happy learning! 🦀
