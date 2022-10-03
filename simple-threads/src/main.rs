use std::vec;
use std::thread;
use rand::Rng;
use std::sync::Arc;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};

/*
Creates a long vector of random numbers and
sums them using multiple threads.
*/

fn sum_seq(vals: &Vec<i32>) -> usize {
    (vals.iter().sum::<i32>()) as usize
}

fn sum_conc(vals: Arc<Vec<i32>>, n_threads: usize) -> usize {

    let n = vals.len();
    let mut handles = vec![];

    let sum = Arc::new(AtomicUsize::new(0));
    
    for id in 0..n_threads {
        let sum = Arc::clone(&sum);
        let vals = Arc::clone(&vals);

        let handle = thread::spawn(move || {
            let mut loc_sum: usize = 0;

            let offset = n/n_threads;
            let from = id*offset;
            let until = if id == n_threads-1 {n} else {from+offset};

            for i in from..until {
                loc_sum += vals[i] as usize;
            }

            // Atomically increment sum.
            sum.fetch_add(loc_sum, Ordering::Relaxed);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    return sum.load(Ordering::Relaxed);
}

fn main() {
    
    let n = 1_000_000_000;
    let mut vals: Vec<i32> = vec![0; n];

    for i in 0..vals.len() {
        vals[i] = rand::thread_rng().gen_range(0..3);
    }

    let start_time = Instant::now();
    let sum = sum_seq(&vals);
    let elapsed = start_time.elapsed();

    println!("Sum: {}, Time: {} ms", sum, elapsed.as_millis());

    let n_threads = n / 200_000_000;

    let start_time = Instant::now();
    let vals = Arc::new(vals);
    let sum = sum_conc(vals.clone(), n_threads);
    let elapsed = start_time.elapsed();

    println!("Sum: {}, Time: {} ms, # Threads: {}", 
        sum, 
        elapsed.as_millis(), 
        n_threads
    );
}
