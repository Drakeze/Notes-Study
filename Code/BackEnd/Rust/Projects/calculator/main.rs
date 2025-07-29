// Simple Calculator Project
// A command-line calculator with basic arithmetic operations

use std::io;
use std::fmt;

#[derive(Debug)]
enum CalculatorError {
    InvalidInput,
    DivisionByZero,
    ParseError,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::InvalidInput => write!(f, "Invalid input format"),
            CalculatorError::DivisionByZero => write!(f, "Cannot divide by zero"),
            CalculatorError::ParseError => write!(f, "Failed to parse number"),
        }
    }
}

impl std::error::Error for CalculatorError {}

struct Calculator;

impl Calculator {
    fn new() -> Self {
        Calculator
    }
    
    fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }
    
    fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }
    
    fn multiply(&self, a: f64, b: f64) -> f64 {
        a * b
    }
    
    fn divide(&self, a: f64, b: f64) -> Result<f64, CalculatorError> {
        if b == 0.0 {
            Err(CalculatorError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    fn power(&self, base: f64, exponent: f64) -> f64 {
        base.powf(exponent)
    }
    
    fn sqrt(&self, n: f64) -> Result<f64, CalculatorError> {
        if n < 0.0 {
            Err(CalculatorError::InvalidInput)
        } else {
            Ok(n.sqrt())
        }
    }
    
    fn parse_expression(&self, input: &str) -> Result<f64, CalculatorError> {
        let input = input.trim();
        
        // Handle simple expressions like "5 + 3", "10 / 2", etc.
        if let Some(pos) = input.find(' ') {
            let parts: Vec<&str> = input.split_whitespace().collect();
            
            if parts.len() != 3 {
                return Err(CalculatorError::InvalidInput);
            }
            
            let a: f64 = parts[0].parse().map_err(|_| CalculatorError::ParseError)?;
            let operator = parts[1];
            let b: f64 = parts[2].parse().map_err(|_| CalculatorError::ParseError)?;
            
            match operator {
                "+" => Ok(self.add(a, b)),
                "-" => Ok(self.subtract(a, b)),
                "*" => Ok(self.multiply(a, b)),
                "/" => self.divide(a, b),
                "^" => Ok(self.power(a, b)),
                _ => Err(CalculatorError::InvalidInput),
            }
        } else if input.starts_with("sqrt(") && input.ends_with(")") {
            // Handle sqrt function
            let number_str = &input[5..input.len()-1];
            let number: f64 = number_str.parse().map_err(|_| CalculatorError::ParseError)?;
            self.sqrt(number)
        } else {
            // Single number
            input.parse().map_err(|_| CalculatorError::ParseError)
        }
    }
}

fn print_help() {
    println!("\n=== Calculator Help ===");
    println!("Supported operations:");
    println!("  Addition: 5 + 3");
    println!("  Subtraction: 10 - 4");
    println!("  Multiplication: 6 * 7");
    println!("  Division: 15 / 3");
    println!("  Power: 2 ^ 3");
    println!("  Square root: sqrt(16)");
    println!("\nCommands:");
    println!("  help - Show this help");
    println!("  history - Show calculation history");
    println!("  clear - Clear history");
    println!("  quit - Exit calculator");
    println!();
}

fn main() {
    let calculator = Calculator::new();
    let mut history: Vec<String> = Vec::new();
    
    println!("🧮 Welcome to Rust Calculator!");
    println!("Type 'help' for instructions or 'quit' to exit.");
    
    loop {
        print!("calc> ");
        io::Write::flush(&mut io::stdout()).unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                if input.is_empty() {
                    continue;
                }
                
                match input.to_lowercase().as_str() {
                    "quit" | "exit" | "q" => {
                        println!("Goodbye! 👋");
                        break;
                    }
                    "help" | "h" => {
                        print_help();
                        continue;
                    }
                    "history" => {
                        if history.is_empty() {
                            println!("No calculations in history.");
                        } else {
                            println!("\nCalculation History:");
                            for (i, calc) in history.iter().enumerate() {
                                println!("  {}. {}", i + 1, calc);
                            }
                        }
                        continue;
                    }
                    "clear" => {
                        history.clear();
                        println!("History cleared.");
                        continue;
                    }
                    _ => {}
                }
                
                match calculator.parse_expression(input) {
                    Ok(result) => {
                        println!("= {}", result);
                        history.push(format!("{} = {}", input, result));
                        
                        // Keep only last 10 calculations
                        if history.len() > 10 {
                            history.remove(0);
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        println!("Type 'help' for usage instructions.");
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_operations() {
        let calc = Calculator::new();
        
        assert_eq!(calc.add(2.0, 3.0), 5.0);
        assert_eq!(calc.subtract(5.0, 3.0), 2.0);
        assert_eq!(calc.multiply(4.0, 3.0), 12.0);
        assert!(calc.divide(6.0, 2.0).unwrap() == 3.0);
    }
    
    #[test]
    fn test_division_by_zero() {
        let calc = Calculator::new();
        assert!(calc.divide(5.0, 0.0).is_err());
    }
    
    #[test]
    fn test_parse_expression() {
        let calc = Calculator::new();
        
        assert_eq!(calc.parse_expression("5 + 3").unwrap(), 8.0);
        assert_eq!(calc.parse_expression("10 - 4").unwrap(), 6.0);
        assert_eq!(calc.parse_expression("6 * 7").unwrap(), 42.0);
        assert_eq!(calc.parse_expression("15 / 3").unwrap(), 5.0);
        assert_eq!(calc.parse_expression("2 ^ 3").unwrap(), 8.0);
        assert_eq!(calc.parse_expression("sqrt(16)").unwrap(), 4.0);
    }
    
    #[test]
    fn test_invalid_input() {
        let calc = Calculator::new();
        
        assert!(calc.parse_expression("5 +").is_err());
        assert!(calc.parse_expression("+ 3").is_err());
        assert!(calc.parse_expression("abc").is_err());
    }
}