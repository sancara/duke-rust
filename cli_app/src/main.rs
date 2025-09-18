//! # CLI Application Examples
//! 
//! This module demonstrates building command-line applications in Rust:
//! - Argument parsing with clap
//! - File I/O operations
//! - JSON processing with serde
//! - Error handling in CLI apps

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Parser)]
#[command(name = "rust-cli")]
#[command(about = "A CLI application demonstrating Rust concepts")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greet someone
    Greet {
        /// Name of the person to greet
        #[arg(short, long)]
        name: String,
        /// Number of times to greet
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
    /// File operations
    File {
        #[command(subcommand)]
        file_command: FileCommands,
    },
    /// JSON operations
    Json {
        #[command(subcommand)]
        json_command: JsonCommands,
    },
    /// Calculate operations
    Calc {
        /// First number
        a: f64,
        /// Second number  
        b: f64,
        /// Operation (+, -, *, /)
        #[arg(short, long)]
        operation: String,
    },
}

#[derive(Subcommand)]
enum FileCommands {
    /// Read a file
    Read {
        /// Path to the file
        path: String,
    },
    /// Write to a file
    Write {
        /// Path to the file
        path: String,
        /// Content to write
        content: String,
    },
    /// List directory contents
    List {
        /// Directory path (default: current directory)
        #[arg(default_value = ".")]
        path: String,
    },
}

#[derive(Subcommand)]
enum JsonCommands {
    /// Create a sample JSON file
    Create {
        /// Output file path
        path: String,
    },
    /// Read and parse JSON file
    Read {
        /// JSON file path
        path: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
    hobbies: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Greet { name, count } => {
            for _ in 0..*count {
                println!("Hello, {}!", name);
            }
        }
        
        Commands::File { file_command } => {
            if let Err(e) = handle_file_command(file_command) {
                eprintln!("File operation failed: {}", e);
                std::process::exit(1);
            }
        }
        
        Commands::Json { json_command } => {
            if let Err(e) = handle_json_command(json_command) {
                eprintln!("JSON operation failed: {}", e);
                std::process::exit(1);
            }
        }
        
        Commands::Calc { a, b, operation } => {
            match calculate(*a, *b, operation) {
                Ok(result) => println!("{} {} {} = {}", a, operation, b, result),
                Err(e) => {
                    eprintln!("Calculation error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn handle_file_command(command: &FileCommands) -> io::Result<()> {
    match command {
        FileCommands::Read { path } => {
            let content = fs::read_to_string(path)?;
            println!("File content:\n{}", content);
        }
        
        FileCommands::Write { path, content } => {
            fs::write(path, content)?;
            println!("Successfully wrote to {}", path);
        }
        
        FileCommands::List { path } => {
            let entries = fs::read_dir(path)?;
            println!("Contents of {}:", path);
            
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file_type = if path.is_dir() { "DIR " } else { "FILE" };
                let file_name = path.file_name().unwrap().to_string_lossy();
                println!("  {} {}", file_type, file_name);
            }
        }
    }
    Ok(())
}

fn handle_json_command(command: &JsonCommands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        JsonCommands::Create { path } => {
            let person = Person {
                name: "John Doe".to_string(),
                age: 30,
                email: "john.doe@example.com".to_string(),
                hobbies: vec![
                    "Reading".to_string(),
                    "Programming".to_string(),
                    "Hiking".to_string(),
                ],
            };
            
            let json = serde_json::to_string_pretty(&person)?;
            fs::write(path, json)?;
            println!("Created sample JSON file at {}", path);
        }
        
        JsonCommands::Read { path } => {
            let content = fs::read_to_string(path)?;
            let person: Person = serde_json::from_str(&content)?;
            println!("Parsed JSON:");
            println!("  Name: {}", person.name);
            println!("  Age: {}", person.age);
            println!("  Email: {}", person.email);
            println!("  Hobbies: {}", person.hobbies.join(", "));
        }
    }
    Ok(())
}

fn calculate(a: f64, b: f64, operation: &str) -> Result<f64, String> {
    match operation {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Unknown operation: {}", operation)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(10.0, 5.0, "+"), Ok(15.0));
        assert_eq!(calculate(10.0, 5.0, "-"), Ok(5.0));
        assert_eq!(calculate(10.0, 5.0, "*"), Ok(50.0));
        assert_eq!(calculate(10.0, 5.0, "/"), Ok(2.0));
        assert!(calculate(10.0, 0.0, "/").is_err());
        assert!(calculate(10.0, 5.0, "%").is_err());
    }

    #[test]
    fn test_person_serialization() {
        let person = Person {
            name: "Test User".to_string(),
            age: 25,
            email: "test@example.com".to_string(),
            hobbies: vec!["Testing".to_string()],
        };
        
        let json = serde_json::to_string(&person).unwrap();
        let deserialized: Person = serde_json::from_str(&json).unwrap();
        
        assert_eq!(person.name, deserialized.name);
        assert_eq!(person.age, deserialized.age);
        assert_eq!(person.email, deserialized.email);
        assert_eq!(person.hobbies, deserialized.hobbies);
    }
}