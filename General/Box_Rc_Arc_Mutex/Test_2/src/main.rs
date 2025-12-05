use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // Shared Mind (state) across all robots
    let shared_mind = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    // Spawn 5 "robots" that read the shared mind
    for i in 0..5 {
        let mind = Arc::clone(&shared_mind);
        let handle = thread::spawn(move || {
            let read_guard = mind.read().unwrap();
            println!("Robot {} reads shared mind: {}", i, *read_guard);
            thread::sleep(Duration::from_millis(50)); // simulate work
        });
        handles.push(handle);
    }

    // Spawn 2 "robots" that write to the shared mind
    for i in 5..7 {
        let mind = Arc::clone(&shared_mind);
        let handle = thread::spawn(move || {
            let mut write_guard = mind.write().unwrap();
            *write_guard += 10; // simulate updating memories/experiences
            println!("Robot {} updates shared mind to: {}", i, *write_guard);
            thread::sleep(Duration::from_millis(50));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Shared Mind: {}", *shared_mind.read().unwrap());
}
