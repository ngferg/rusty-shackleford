use std::thread;
use std::time::Duration;
use rand::Rng; // Import the Rng trait

fn main() {
    // Vector to store thread handles
    let mut handles = vec![];
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    // Spawn 5 threads
    for i in 0..5 {

        let sleep_time = rng.gen_range(100..=1000);

        // Spawn a new thread
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            
            // Simulate some work
            let result = i * 2;
            thread::sleep(Duration::from_millis(sleep_time)); // Small delay
            
            println!("Thread {} finished with result: {}", i, result);
            result
        });
        
        // Store the thread handle
        handles.push(handle);
    }

    // Collect results from all threads
    let mut results = vec![];
    for handle in handles {
        // Wait for thread to finish and get its result
        let result = handle.join().unwrap();
        results.push(result);
    }

    println!("All threads completed. Results: {:?}", results);
}
