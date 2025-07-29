// Rust Functions and Control Flow
// Learning about functions, if statements, loops, and pattern matching

fn main() {
    // Function calls
    greet("Rust");
    
    let sum = add(5, 3);
    println!("Sum: {}", sum);
    
    let (quotient, remainder) = divide(10, 3);
    println!("10 / 3 = {} remainder {}", quotient, remainder);
    
    // Control flow
    let number = 6;
    
    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }
    
    // Loops
    println!("\nLoop examples:");
    
    // for loop
    for i in 1..=5 {
        println!("for loop: {}", i);
    }
    
    // while loop
    let mut counter = 0;
    while counter < 3 {
        println!("while loop: {}", counter);
        counter += 1;
    }
    
    // loop with break
    let mut x = 0;
    let result = loop {
        x += 1;
        if x == 10 {
            break x * 2;
        }
    };
    println!("Loop result: {}", result);
    
    // Pattern matching
    let value = 3;
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4..=10 => println!("Between 4 and 10"),
        _ => println!("Something else"),
    }
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // implicit return
}

fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}