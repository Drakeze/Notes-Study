// Rust Ownership and Borrowing
// Core concepts of memory management in Rust

fn main() {
    // Ownership basics
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); // This would cause a compile error
    println!("s2: {}", s2);
    
    // Clone to avoid move
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);
    
    // Copy types (stack-allocated)
    let x = 5;
    let y = x; // x is copied, not moved
    println!("x: {}, y: {}", x, y);
    
    // Function ownership
    let s = String::from("hello");
    takes_ownership(s); // s is moved into the function
    // println!("{}", s); // This would cause a compile error
    
    let x = 5;
    makes_copy(x); // x is copied
    println!("x is still valid: {}", x);
    
    // Return values and scope
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {}, s3: {}", s1, s3);
    
    // References and borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // borrowing
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("Changed string: {}", s);
    
    // Multiple immutable references are allowed
    let s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    println!("{} and {}", r1, r2);
    
    // But only one mutable reference at a time
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // This would cause a compile error
    println!("{}", r1);
    
    // Slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("First word: {}, Second word: {}", hello, world);
    
    let first_word = first_word(&s);
    println!("First word using function: {}", first_word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, but nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // return value is moved to calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // return value is moved to calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but because it's a reference, nothing is dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}