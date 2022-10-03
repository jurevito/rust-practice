use std::vec;
use std::thread;
use rand::Rng;
use std::sync::Arc;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    
    let size = 1000_000_000;
    let mut vals: Vec<i32> = vec![0; size];

    // Fill vector with random numbers.
    for i in 0..vals.len() {
        vals[i] = rand::thread_rng().gen_range(0..3);
    }

    let start = Instant::now();
    let sum: i32 = vals.iter().sum();
    let elapsed = start.elapsed();

    println!("Sum: {}, Time: {} ms", sum, elapsed.as_millis());

    let start = Instant::now();
    let n_threads = size / 100_000_000;
    let mut handles = vec![];

    let sum = Arc::new(AtomicUsize::new(0));
    let vals = Arc::new(vals);

    for id in 0..n_threads {
        let sum = Arc::clone(&sum);
        let vals = Arc::clone(&vals);

        let handle = thread::spawn(move || {
            let mut loc_sum: usize = 0;

            let offset = size/n_threads;
            let from = id*offset;
            let until = if id == n_threads-1 {size} else {from+offset};

            for i in from..until {
                loc_sum += vals[i] as usize;
            }
            
            // Atomically increment sum.
            let current_sum = sum.load(Ordering::Acquire);
            sum.store(current_sum + loc_sum, Ordering::Release);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start.elapsed();
    println!("Sum: {}, Time: {} ms, # Threads: {}", 
        (*sum).load(Ordering::Acquire), 
        elapsed.as_millis(), 
        n_threads
    );
}
