use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("in thread: {}", i);
            thread::sleep(Duration::from_millis(500)); // Pause for 500ms
        }
    });

    // Main thread
    for i in 1..3 {
        println!("in main: {}", i);
        thread::sleep(Duration::from_millis(700));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();

}
