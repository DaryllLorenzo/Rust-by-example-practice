pub struct FC{}

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

    }
}