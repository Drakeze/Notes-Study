// Rust Traits and Generics
// Learning about trait definitions, implementations, and generic programming

use std::fmt::Display;

// Define a trait
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
}

// Structs that will implement the trait
#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// Implement the trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// Implement the trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Generic struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
}

// Implementation for specific type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic struct with multiple type parameters
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// Generic function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Function that takes a trait object
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
fn notify_verbose<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn notify_and_display<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Display: {}", item);
}

// Where clause for complex bounds
fn some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{
    42
}

// Return trait objects
fn returns_summarizable() -> Box<dyn Summary> {
    Box::new(Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    })
}

// Trait for comparison
trait Compare<T> {
    fn compare(&self, other: &T) -> std::cmp::Ordering;
}

struct Number {
    value: i32,
}

impl Compare<Number> for Number {
    fn compare(&self, other: &Number) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

fn main() {
    // Using traits
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    
    notify(&article);
    notify(&tweet);
    
    // Using generics
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    println!("Integer point: {:?}", integer_point);
    println!("Float point: {:?}", float_point);
    println!("Float point distance: {}", float_point.distance_from_origin());
    
    let mixed = MixedPoint { x: 5, y: 10.4 };
    let mixed2 = MixedPoint { x: "Hello", y: 'c' };
    let mixed3 = mixed.mixup(mixed2);
    
    println!("Mixed point: {:?}", mixed3);
    
    // Generic function
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    
    // Trait objects
    let summarizable = returns_summarizable();
    println!("Returned summary: {}", summarizable.summarize());
    
    // Custom comparison
    let num1 = Number { value: 10 };
    let num2 = Number { value: 20 };
    
    match num1.compare(&num2) {
        std::cmp::Ordering::Less => println!("num1 is less than num2"),
        std::cmp::Ordering::Greater => println!("num1 is greater than num2"),
        std::cmp::Ordering::Equal => println!("num1 is equal to num2"),
    }
}