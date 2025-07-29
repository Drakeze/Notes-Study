// Rust Collections Practice
// Working with Vec, HashMap, HashSet, and other collections

use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};

fn main() {
    println!("=== Vector Practice ===");
    
    // Creating and manipulating vectors
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Original vector: {:?}", numbers);
    
    // Adding elements
    numbers.push(6);
    numbers.extend([7, 8, 9]);
    println!("After adding elements: {:?}", numbers);
    
    // Accessing elements
    match numbers.get(2) {
        Some(third) => println!("Third element: {}", third),
        None => println!("No third element"),
    }
    
    // Iterating
    println!("Squares:");
    for (i, &num) in numbers.iter().enumerate() {
        println!("  {}^2 = {}", num, num * num);
    }
    
    // Filtering and mapping
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("Even squares: {:?}", even_squares);
    
    // Vector of strings
    let mut words = vec!["hello", "world", "rust", "programming"];
    words.sort();
    println!("Sorted words: {:?}", words);
    
    // Finding elements
    if let Some(pos) = words.iter().position(|&x| x == "rust") {
        println!("Found 'rust' at position {}", pos);
    }
    
    println!("\n=== HashMap Practice ===");
    
    // Creating and using HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for {}: {}", team_name, score);
    
    // Iterating over HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Updating values
    scores.entry(String::from("Blue")).and_modify(|e| *e += 10);
    scores.entry(String::from("Red")).or_insert(25);
    println!("Updated scores: {:?}", scores);
    
    // Word frequency counter
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word frequencies: {:?}", word_count);
    
    println!("\n=== HashSet Practice ===");
    
    // Creating and using HashSet
    let mut set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set2: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();
    
    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);
    
    // Set operations
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("Intersection: {:?}", intersection);
    
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("Union: {:?}", union);
    
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("Difference (set1 - set2): {:?}", difference);
    
    // Adding and removing elements
    set1.insert(10);
    set1.remove(&1);
    println!("Modified set1: {:?}", set1);
    
    // Checking membership
    if set1.contains(&3) {
        println!("Set1 contains 3");
    }
    
    println!("\n=== BTreeMap Practice ===");
    
    // BTreeMap maintains sorted order
    let mut btree_map = BTreeMap::new();
    btree_map.insert(3, "three");
    btree_map.insert(1, "one");
    btree_map.insert(4, "four");
    btree_map.insert(2, "two");
    
    println!("BTreeMap (sorted by key):");
    for (key, value) in &btree_map {
        println!("  {}: {}", key, value);
    }
    
    // Range queries
    println!("Keys from 2 to 3:");
    for (key, value) in btree_map.range(2..=3) {
        println!("  {}: {}", key, value);
    }
    
    println!("\n=== VecDeque Practice ===");
    
    // Double-ended queue
    let mut deque = VecDeque::new();
    
    // Add to both ends
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    deque.push_front(-1);
    
    println!("Deque: {:?}", deque);
    
    // Remove from both ends
    if let Some(front) = deque.pop_front() {
        println!("Removed from front: {}", front);
    }
    
    if let Some(back) = deque.pop_back() {
        println!("Removed from back: {}", back);
    }
    
    println!("Deque after removals: {:?}", deque);
    
    println!("\n=== Advanced Collection Operations ===");
    
    // Grouping data
    let people = vec![
        ("Alice", 25),
        ("Bob", 30),
        ("Charlie", 25),
        ("David", 30),
        ("Eve", 35),
    ];
    
    let mut age_groups: HashMap<i32, Vec<&str>> = HashMap::new();
    for (name, age) in &people {
        age_groups.entry(*age).or_insert_with(Vec::new).push(name);
    }
    
    println!("People grouped by age:");
    for (age, names) in &age_groups {
        println!("  Age {}: {:?}", age, names);
    }
    
    // Finding duplicates
    let numbers = vec![1, 2, 3, 2, 4, 3, 5, 1];
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    
    for &num in &numbers {
        if !seen.insert(num) {
            duplicates.insert(num);
        }
    }
    
    println!("Duplicates in {:?}: {:?}", numbers, duplicates);
    
    // Custom struct in collections
    #[derive(Debug, Hash, Eq, PartialEq, Clone)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let mut person_scores = HashMap::new();
    let alice = Person {
        name: String::from("Alice"),
        age: 25,
    };
    let bob = Person {
        name: String::from("Bob"),
        age: 30,
    };
    
    person_scores.insert(alice.clone(), 95);
    person_scores.insert(bob.clone(), 87);
    
    println!("Person scores:");
    for (person, score) in &person_scores {
        println!("  {:?}: {}", person, score);
    }
    
    // Nested collections
    let mut matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    
    println!("Matrix:");
    for row in &matrix {
        println!("  {:?}", row);
    }
    
    // Transpose matrix
    let transposed: Vec<Vec<i32>> = (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect();
    
    println!("Transposed matrix:");
    for row in &transposed {
        println!("  {:?}", row);
    }
    
    println!("\nCollection practice completed!");
}