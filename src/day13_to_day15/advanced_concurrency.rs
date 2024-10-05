use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn examples() {
    println!("Advanced Concurrency Examples:");
    
    // Example 1: Shared counter with Arc and Mutex
    shared_counter_example();

    // Example 2: Producer-Consumer pattern
    producer_consumer_example();
}

fn shared_counter_example() {
    println!("\nShared Counter Example:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter to {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

fn producer_consumer_example() {
    println!("\nProducer-Consumer Example:");
    let queue = Arc::new(Mutex::new(Vec::new()));
    let queue_clone = Arc::clone(&queue);

    // Producer thread
    let producer = thread::spawn(move || {
        for i in 0..5 {
            {
                let mut q = queue_clone.lock().unwrap();
                q.push(i);
                println!("Produced: {}", i);
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Consumer thread
    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            {
                let mut q = queue.lock().unwrap();
                if let Some(item) = q.pop() {
                    println!("Consumed: {}", item);
                }
            }
            thread::sleep(Duration::from_millis(150));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}