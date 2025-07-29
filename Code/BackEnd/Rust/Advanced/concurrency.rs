// Rust Concurrency
// Learning about threads, channels, and synchronization

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Basic Threading ===");
    
    // Basic thread creation
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for the thread to finish
    handle.join().unwrap();
    
    println!("\n=== Moving Data into Threads ===");
    
    // Moving data into threads
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    
    handle.join().unwrap();
    
    println!("\n=== Message Passing with Channels ===");
    
    // Single producer, single consumer
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // val is no longer accessible here
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    
    // Multiple messages
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
    
    println!("\n=== Multiple Producers ===");
    
    // Multiple producers
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
    
    println!("\n=== Shared State with Mutex ===");
    
    // Shared state with Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
    
    println!("\n=== Producer-Consumer Pattern ===");
    
    // Producer-Consumer pattern
    let (tx, rx) = mpsc::channel();
    let data = Arc::new(Mutex::new(Vec::new()));
    
    // Producer
    let producer_data = Arc::clone(&data);
    let producer = thread::spawn(move || {
        for i in 0..5 {
            let item = format!("item_{}", i);
            {
                let mut data = producer_data.lock().unwrap();
                data.push(item.clone());
            }
            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Consumer
    let consumer_data = Arc::clone(&data);
    let consumer = thread::spawn(move || {
        while let Ok(item) = rx.recv() {
            println!("Consumed: {}", item);
            let data = consumer_data.lock().unwrap();
            println!("Current data length: {}", data.len());
            thread::sleep(Duration::from_millis(150));
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    
    println!("\n=== Thread Pool Simulation ===");
    
    // Simple thread pool simulation
    let (job_tx, job_rx) = mpsc::channel();
    let job_rx = Arc::new(Mutex::new(job_rx));
    let mut workers = vec![];
    
    // Create worker threads
    for id in 0..4 {
        let job_rx = Arc::clone(&job_rx);
        let worker = thread::spawn(move || {
            loop {
                let job = job_rx.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Worker {} executing job: {}", id, job);
                        thread::sleep(Duration::from_millis(500));
                        println!("Worker {} finished job: {}", id, job);
                    }
                    Err(_) => {
                        println!("Worker {} shutting down", id);
                        break;
                    }
                }
            }
        });
        workers.push(worker);
    }
    
    // Send jobs
    for i in 0..8 {
        let job = format!("job_{}", i);
        job_tx.send(job).unwrap();
    }
    
    // Close the channel to signal workers to shut down
    drop(job_tx);
    
    // Wait for all workers to finish
    for worker in workers {
        worker.join().unwrap();
    }
    
    println!("\n=== Deadlock Prevention Example ===");
    
    // Deadlock prevention by ordering locks
    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle1 = thread::spawn(move || {
        // Always acquire locks in the same order
        let _guard1 = r1.lock().unwrap();
        println!("Thread 1: acquired resource1");
        thread::sleep(Duration::from_millis(100));
        
        let _guard2 = r2.lock().unwrap();
        println!("Thread 1: acquired resource2");
    });
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle2 = thread::spawn(move || {
        // Same order to prevent deadlock
        let _guard1 = r1.lock().unwrap();
        println!("Thread 2: acquired resource1");
        thread::sleep(Duration::from_millis(100));
        
        let _guard2 = r2.lock().unwrap();
        println!("Thread 2: acquired resource2");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("\n=== Atomic Operations ===");
    
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Atomic counter result: {}", counter.load(Ordering::SeqCst));
    
    println!("\nAll concurrency examples completed!");
}