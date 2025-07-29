// Rust Structs and Enums
// Learning about custom data types

// Struct definitions
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Struct with methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Associated function (like static method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit struct
struct AlwaysEqual;

// Enums
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    // Creating structs
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("User: {:?}", user1);
    
    // Struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // use remaining fields from user1
    };
    
    println!("User2: {:?}", user2);
    
    // Rectangle examples
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("Rectangle: {:?}", rect1);
    println!("Area: {}", rect1.area());
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // Associated function
    let square = Rectangle::square(3);
    println!("Square: {:?}", square);
    
    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black color: {:?}", black);
    println!("Origin point: {:?}", origin);
    
    // Unit struct
    let subject = AlwaysEqual;
    
    // Enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IP kinds: {:?}, {:?}", four, six);
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);
    
    // Message enum
    let m = Message::Write(String::from("hello"));
    m.call();
    
    let move_msg = Message::Move { x: 10, y: 20 };
    move_msg.call();
    
    // Option enum (built-in)
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);
    
    // Pattern matching with Option
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    match y {
        None => println!("No value"),
        Some(i) => println!("Got a value: {}", i),
    }
    
    // if let syntax
    if let Some(3) = some_number {
        println!("three");
    } else {
        println!("not three");
    }
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("IPv4"),
        IpAddrKind::V6 => println!("IPv6"),
    }
}