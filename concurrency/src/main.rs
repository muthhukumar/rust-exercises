use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            String::from("thread1: hi"),
            String::from("thread1: from"),
            String::from("thread1: spawned"),
            String::from("thread1: thread"),
        ];

        for m in messages {
            // let val = String::from("Hello main thread");
            tx2.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("thread2: hi"),
            String::from("thread2: from"),
            String::from("thread2: spawned"),
            String::from("thread2: thread"),
        ];

        for m in messages {
            // let val = String::from("Hello main thread");
            tx.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    for received in rx {
        println!("message from thread: '{}'", received);
    }
}

fn _example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("main: {i}");
        thread::sleep(Duration::from_millis(10));
    }

    let v = vec![1, 2, 4];

    let v_handle = thread::spawn(move || {
        println!("this is a vector: {:?}", v);
    });

    v_handle.join().unwrap();
}
