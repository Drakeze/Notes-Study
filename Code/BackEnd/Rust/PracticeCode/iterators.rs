// Rust Iterators and Functional Programming
// Practice with iterator patterns, closures, and functional programming

fn main() {
    println!("=== Basic Iterator Usage ===");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Basic iteration
    println!("Numbers: {:?}", numbers);
    
    // for loop (syntactic sugar for iterator)
    print!("For loop: ");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();
    
    // Explicit iterator
    print!("Iterator: ");
    numbers.iter().for_each(|num| print!("{} ", num));
    println!();
    
    println!("\n=== Iterator Adaptors ===");
    
    // map - transform each element
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares);
    
    // filter - keep elements that match predicate
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);
    
    // enumerate - add indices
    let indexed: Vec<(usize, &i32)> = numbers.iter().enumerate().collect();
    println!("With indices: {:?}", indexed);
    
    // zip - combine with another iterator
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    let zipped: Vec<(&i32, &char)> = numbers.iter().zip(letters.iter()).collect();
    println!("Zipped: {:?}", zipped);
    
    // take and skip
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_two: Vec<&i32> = numbers.iter().skip(2).collect();
    println!("First three: {:?}", first_three);
    println!("Skip two: {:?}", skip_two);
    
    // chain - combine iterators
    let more_numbers = vec![11, 12, 13];
    let chained: Vec<&i32> = numbers.iter().chain(more_numbers.iter()).collect();
    println!("Chained: {:?}", chained);
    
    println!("\n=== Iterator Consumers ===");
    
    // reduce/fold operations
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    println!("Sum: {}, Product: {}", sum, product);
    
    // fold with custom operation
    let sum_of_squares = numbers.iter().fold(0, |acc, &x| acc + x * x);
    println!("Sum of squares: {}", sum_of_squares);
    
    // reduce (similar to fold but uses first element as initial value)
    let max = numbers.iter().reduce(|acc, x| if acc > x { acc } else { x });
    println!("Max: {:?}", max);
    
    // find operations
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even);
    
    let position_of_5 = numbers.iter().position(|&&x| x == 5);
    println!("Position of 5: {:?}", position_of_5);
    
    // any and all
    let has_even = numbers.iter().any(|&&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&&x| x > 0);
    println!("Has even: {}, All positive: {}", has_even, all_positive);
    
    // count
    let even_count = numbers.iter().filter(|&&x| x % 2 == 0).count();
    println!("Count of even numbers: {}", even_count);
    
    println!("\n=== Closures ===");
    
    // Different closure syntaxes
    let add_one = |x| x + 1;
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    let complex_closure = |x| {
        let y = x * 2;
        y + 1
    };
    
    println!("add_one(5): {}", add_one(5));
    println!("multiply(3, 4): {}", multiply(3, 4));
    println!("complex_closure(3): {}", complex_closure(3));
    
    // Capturing environment
    let factor = 10;
    let multiply_by_factor = |x| x * factor;
    let result: Vec<i32> = numbers.iter().map(multiply_by_factor).collect();
    println!("Multiplied by {}: {:?}", factor, result);
    
    // Move closures
    let data = vec![1, 2, 3];
    let closure = move |x| {
        println!("Data in closure: {:?}", data);
        x + data.len() as i32
    };
    println!("Closure result: {}", closure(5));
    // data is no longer accessible here
    
    println!("\n=== Advanced Iterator Patterns ===");
    
    // Chaining multiple operations
    let result: Vec<String> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // keep even numbers
        .map(|x| x * x)            // square them
        .filter(|&&x| x > 10)      // keep those > 10
        .map(|x| format!("#{}", x)) // format as strings
        .collect();
    
    println!("Chained operations result: {:?}", result);
    
    // Partition - split into two collections
    let (evens, odds): (Vec<&i32>, Vec<&i32>) = numbers
        .iter()
        .partition(|&&x| x % 2 == 0);
    
    println!("Partitioned - Evens: {:?}, Odds: {:?}", evens, odds);
    
    // Group by using fold
    use std::collections::HashMap;
    
    let grouped: HashMap<bool, Vec<&i32>> = numbers
        .iter()
        .fold(HashMap::new(), |mut acc, &x| {
            acc.entry(x % 2 == 0).or_insert_with(Vec::new).push(x);
            acc
        });
    
    println!("Grouped by even/odd: {:?}", grouped);
    
    // Flat map - flatten nested structures
    let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    let flattened: Vec<&i32> = nested.iter().flat_map(|v| v.iter()).collect();
    println!("Flattened: {:?}", flattened);
    
    // Scan - like fold but keeps intermediate results
    let running_sum: Vec<i32> = numbers
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    
    println!("Running sum: {:?}", running_sum);
    
    println!("\n=== Custom Iterator ===");
    
    // Custom iterator struct
    struct Counter {
        current: usize,
        max: usize,
    }
    
    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { current: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = usize;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let current = self.current;
                self.current += 1;
                Some(current)
            } else {
                None
            }
        }
    }
    
    let counter = Counter::new(5);
    let counter_values: Vec<usize> = counter.collect();
    println!("Custom counter: {:?}", counter_values);
    
    // Using custom iterator with other iterator methods
    let counter_squares: Vec<usize> = Counter::new(5)
        .map(|x| x * x)
        .collect();
    
    println!("Counter squares: {:?}", counter_squares);
    
    println!("\n=== Performance Considerations ===");
    
    // Iterator vs for loop performance
    let large_vec: Vec<i32> = (0..1_000_000).collect();
    
    // Iterator approach (zero-cost abstraction)
    let start = std::time::Instant::now();
    let sum_iter: i32 = large_vec.iter().map(|x| x * 2).sum();
    let iter_time = start.elapsed();
    
    // Traditional for loop
    let start = std::time::Instant::now();
    let mut sum_loop = 0;
    for &x in &large_vec {
        sum_loop += x * 2;
    }
    let loop_time = start.elapsed();
    
    println!("Iterator sum: {}, time: {:?}", sum_iter, iter_time);
    println!("Loop sum: {}, time: {:?}", sum_loop, loop_time);
    
    // Lazy evaluation demonstration
    println!("\n=== Lazy Evaluation ===");
    
    let lazy_iter = numbers
        .iter()
        .map(|x| {
            println!("Processing: {}", x);
            x * 2
        })
        .filter(|&&x| x > 10);
    
    println!("Iterator created (no processing yet)");
    
    // Only when we consume the iterator does processing happen
    let result: Vec<&i32> = lazy_iter.take(3).collect();
    println!("Result: {:?}", result);
    
    println!("\nIterator practice completed!");
}