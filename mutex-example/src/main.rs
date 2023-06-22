use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 1..11 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();

            println!("thread:{i}, count: {:?}", count);

            *count += 1;
        });

        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!("count: {:?}", counter.lock().unwrap());
}
