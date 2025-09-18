//! Control Flow in Rust
//! 
//! This module demonstrates:
//! - if expressions
//! - Loops (loop, while, for)
//! - Pattern matching with match

/// Demonstrates control flow constructs
pub fn demonstrate_control_flow() {
    println!("=== Control Flow ===");
    
    // if expressions
    demonstrate_if_expressions();
    
    // Loops
    demonstrate_loops();
    
    // Pattern matching
    demonstrate_pattern_matching();
}

fn demonstrate_if_expressions() {
    println!("--- If Expressions ---");
    
    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    // if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn demonstrate_loops() {
    println!("--- Loops ---");
    
    // loop with break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result from loop is: {}", result);
    
    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    // for loop with range
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
    // for loop with array
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
    
    // for loop with index
    for (index, value) in a.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }
}

fn demonstrate_pattern_matching() {
    println!("--- Pattern Matching ---");
    
    let number = 13;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    
    // Match with destructuring
    let pair = (0, -2);
    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }
    
    // Match with guards
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_flow() {
        demonstrate_control_flow();
    }
}