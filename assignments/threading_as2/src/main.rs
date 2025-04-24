use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a shared counter using Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    
    // Create a vector to store thread handles
    let mut handles = vec![];
    
    // Spawn 5 threads
    for i in 1..=5 {
        // Clone the Arc for the thread
        let counter_clone = Arc::clone(&counter);
        
        // Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                // Lock the mutex and increment the counter
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
            println!("Thread {} finished incrementing.", i);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}