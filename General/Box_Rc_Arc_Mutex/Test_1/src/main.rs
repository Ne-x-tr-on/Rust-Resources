use std::thread;
use std::sync::{Arc,Mutex};


fn main(){
    let shared_data = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..100{
        let data = Arc::clone(&shared_data);

        let handle = thread::spawn(move||{
            let mut num = data.lock().unwrap();
            *num+=1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
}