// Rust Variables and Data Types
// Learning about ownership, borrowing, and basic types

fn main() {
    // Variables and mutability
    let x = 5; // immutable
    let mut y = 10; // mutable
    
    println!("x: {}, y: {}", x, y);
    
    y = 15;
    println!("Updated y: {}", y);
    
    // Data types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';
    let string: String = String::from("Hello, Rust!");
    let string_slice: &str = "Hello, World!";
    
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("String: {}", string);
    println!("String slice: {}", string_slice);
    
    // Arrays and vectors
    let array: [i32; 3] = [1, 2, 3];
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    println!("Array: {:?}", array);
    println!("Vector: {:?}", vector);
    
    vector.push(6);
    println!("Updated vector: {:?}", vector);
    
    // Tuples
    let tuple: (i32, f64, char) = (42, 3.14, 'R');
    let (a, b, c) = tuple; // destructuring
    
    println!("Tuple: {:?}", tuple);
    println!("Destructured: a={}, b={}, c={}", a, b, c);
}