pub struct FC{}

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


impl FC {
    pub fn run(){

        println!("------------------------Ex 8--------------------");
        // first thread
        let handle1 = thread::spawn(move || {
            let mut first_thread_sum = 0;
            for i in 0..10 {
                println!("First thread counting: {}", i);
                first_thread_sum += i;
                thread::sleep(Duration::from_millis(1));
            }
            first_thread_sum
        });


        let handle2 = thread::spawn(move || {
            let mut second_thread_sum = 0;
            for i in 0..20 {
                println!("Second thread counting: {}", i);
                second_thread_sum += i;
            }
            second_thread_sum
        });

        let res1 = handle1.join().unwrap();
        let res2 = handle2.join().unwrap();

        let final_res = res1 + res2;
        println!("Final result: {}", final_res);

        println!("-------- mutex test --------");
        FC::mutext_test();
        FC::mutex_test_2();
        FC::test_message_passing();

    }

        // For shared mutable state between threads:
        // std::sync::Mutex<T> - Provides mutual exclusion (lock/unlock)
        // std::sync::Arc<T>   - Atomic reference counting for shared ownership
        //
        // Arc<Mutex<T>> allows multiple threads to safely access and modify the same data.
        // Arc clones the pointer, not the data. Mutex ensures only one thread modifies at a time.

    fn mutext_test(){
        use std::sync::Mutex;
        use std::sync::Arc;

        let mutex = Mutex::new(0);

        let counter = Arc::new(mutex);

        // Clone reference for each thread
        let counter1 = Arc::clone(&counter);
        let counter2 = Arc::clone(&counter);
        let counter3 = Arc::clone(&counter);

        let handler1 = thread::spawn(move || {
            for _ in 1..20{
                let mut value = counter1.lock().unwrap();
                *value+=1;
                println!("Thread 1: {}", *value);
                thread::sleep(Duration::from_millis(50));
            }
        });

        let handler2 = thread::spawn(move || {
            for _ in 1..25{
                let mut value = counter2.lock().unwrap();
                *value+=1;
                println!("Thread 2: {}", *value);
                thread::sleep(Duration::from_millis(2));
            }
        });

        let handler3 = thread::spawn(move || {
            for _ in 1..30{
                let mut value = counter3.lock().unwrap();
                *value+=1;
                println!("Thread 3: {}", *value);
                thread::sleep(Duration::from_millis(50));
            }
        });

        // Wait for every thread
        handler1.join().unwrap();
        handler2.join().unwrap();
        handler3.join().unwrap();

        let result = *counter.lock().unwrap();
        println!("Total: {}", result);
    }

    fn mutex_test_2(){
        let counter = Arc::new(Mutex::new(0));
        let mut handlers = vec![];
        let num_threads = 10;

        for _ in 0..num_threads {
            let c = Arc::clone(&counter);

            let handler = thread::spawn(move || {
                let mut value = c.lock().unwrap();
                *value += 1;
            });

            handlers.push(handler);
        }

        for handler in handlers.into_iter() {
            handler.join().unwrap();
        } 

        let sum = *counter.lock().unwrap();
        println!("Sum: {}", sum);
    }


    // One major tool Rust has for accomplishing message-sending concurrency
    // is the channel, a programming concept that Rust’s standard library provides
    // an implementation of. You can imagine a channel in programming as being
    // like a channel of water, such as a stream or river. If you put something like
    // a rubber duck or boat into a stream, it will travel downstream to the end of
    // the waterway.
    // A channel
    fn test_message_passing(){
        use std::sync::mpsc; // multiple producer, single consumer

        let (tx, rx) = mpsc::channel(); // transmitter and receiver

        thread::spawn(move || {
            let val = String::from("My string");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
    
}