// Rust Error Handling
// Learning about Result, Option, and error propagation

use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::num::ParseIntError;

// Custom error types
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeLogarithm,
    NegativeSquareRoot,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeLogarithm => write!(f, "Cannot take logarithm of negative number"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
        }
    }
}

impl std::error::Error for MathError {}

// Functions that return Result
fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// Function that uses ? operator for error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Shorter version using ? operator
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Even shorter using fs::read_to_string
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

// Function that converts string to number
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

// Function that uses multiple error types
fn add_numbers_from_strings(a: &str, b: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let num_a: i32 = a.parse()?;
    let num_b: i32 = b.parse()?;
    Ok(num_a + num_b)
}

// Option examples
fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
}

fn get_first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}

// Combining Option and Result
fn divide_option(a: Option<f64>, b: Option<f64>) -> Option<Result<f64, MathError>> {
    match (a, b) {
        (Some(a), Some(b)) => Some(divide(a, b)),
        _ => None,
    }
}

fn main() {
    println!("=== Basic Result Usage ===");
    
    // Basic Result usage
    match divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Using unwrap and expect (use carefully!)
    let result = divide(8.0, 2.0).unwrap(); // panics if Err
    println!("8.0 / 2.0 = {}", result);
    
    let result = divide(6.0, 3.0).expect("Division should work");
    println!("6.0 / 3.0 = {}", result);
    
    // Using unwrap_or and unwrap_or_else
    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("10.0 / 0.0 with default = {}", result);
    
    let result = divide(10.0, 0.0).unwrap_or_else(|_| {
        println!("Division failed, using default");
        -1.0
    });
    println!("10.0 / 0.0 with closure default = {}", result);
    
    println!("\n=== Error Propagation ===");
    
    // Error propagation with ?
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Failed to read username: {}", e),
    }
    
    // Parsing numbers
    match parse_number("42") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }
    
    match parse_number("not_a_number") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }
    
    // Multiple error types
    match add_numbers_from_strings("10", "20") {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error adding numbers: {}", e),
    }
    
    println!("\n=== Option Usage ===");
    
    // Option examples
    let text = "Hello world, this is Rust programming";
    
    match find_word(text, "Rust") {
        Some(index) => println!("Found 'Rust' at index {}", index),
        None => println!("'Rust' not found"),
    }
    
    match find_word(text, "Python") {
        Some(index) => println!("Found 'Python' at index {}", index),
        None => println!("'Python' not found"),
    }
    
    // Option with unwrap_or
    let first_word = get_first_word(text).unwrap_or("No words");
    println!("First word: {}", first_word);
    
    let empty_text = "";
    let first_word = get_first_word(empty_text).unwrap_or("No words");
    println!("First word in empty text: {}", first_word);
    
    // Option and Result combination
    let a = Some(10.0);
    let b = Some(2.0);
    let c = Some(0.0);
    
    match divide_option(a, b) {
        Some(Ok(result)) => println!("Division result: {}", result),
        Some(Err(e)) => println!("Division error: {}", e),
        None => println!("Missing operands"),
    }
    
    match divide_option(a, c) {
        Some(Ok(result)) => println!("Division result: {}", result),
        Some(Err(e)) => println!("Division error: {}", e),
        None => println!("Missing operands"),
    }
    
    println!("\n=== Chaining Operations ===");
    
    // Chaining with map and and_then
    let result = Some("42")
        .map(|s| s.parse::<i32>())
        .and_then(|r| r.ok())
        .map(|n| n * 2);
    
    println!("Chained result: {:?}", result);
    
    // Using filter
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    
    println!("Even squares: {:?}", even_squares);
    
    // Early return with ?
    fn calculate() -> Result<i32, Box<dyn std::error::Error>> {
        let a: i32 = "10".parse()?;
        let b: i32 = "20".parse()?;
        let result = divide(a as f64, 2.0)? as i32;
        Ok(result + b)
    }
    
    match calculate() {
        Ok(result) => println!("Calculation result: {}", result),
        Err(e) => println!("Calculation error: {}", e),
    }
}