// Rust Test File
// Basic Rust syntax and concepts testing

fn main() {
    println!("Hello, Rust!");
    
    // Variables and mutability
    let x = 5;
    let mut y = 10;
    y += x;
    
    println!("x: {}, y: {}", x, y);
    
    // Function call
    let result = add_numbers(x, y);
    println!("Sum: {}", result);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
    }
}